use crate::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct Buy<'info> {

  pub token_mint: Box<InterfaceAccount<'info, Mint>>,

  #[account(
    seeds = [
      CONFIG_SEED,
    ],
    bump = config.bump
  )]
  pub config: Box<Account<'info, Config>>,

  /// CHECK
  #[account(
    mut,
    seeds = [
      BONDING_CURVE_SEED,
      token_mint.key().as_ref()
    ],
    bump = bonding_curve.bump
  )]
  pub bonding_curve: Account<'info, BondingCurve>,

  /// CHECK
  #[account(
    mut,
    address = bonding_curve.creator
  )]
  pub creator: UncheckedAccount<'info>,

  #[account(
    mut,
    associated_token::mint = token_mint,
    associated_token::authority = bonding_curve,
    token::token_program = token_program,
  )]
  pub associted_bonding_curve: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    mut,
    associated_token::mint = token_mint,
    associated_token::authority = user,
    token::token_program = token_program,
  )]
  pub associted_user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

  /// CHECK
  #[account(
    mut,
    address = config.fee_recipient
  )]
  pub fee_account: UncheckedAccount<'info>,

  /// CHECK
  // #[account(
  //   mut,
  //   address = config.liquidity
  // )]
  // pub liquidity: UncheckedAccount<'info>,

  #[account(mut)]
    pub user: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl Buy<'_> {
  pub fn apply(ctx: &mut Context<Buy>, amount: u64, max_sol_cost: u64) -> Result<()> {
    let decimals = ctx.accounts.token_mint.decimals;

    // check to ensure funding goal is not met
    // require!(
    //     (ctx.accounts.associted_bonding_curve.amount as u128) > INIT_SUPPLY,
    //     PumpFunError::AlreadyRaised
    // );

    let available_qty = ctx.accounts.associted_bonding_curve.amount;
    require!(amount <= available_qty - 2_000_000_000_000_000_000, PumpFunError::NotEnoughSuppply);

    let current_supply =
        T - (ctx.accounts.associted_bonding_curve.amount);
    let required_lamports = calculate_cost(current_supply / 1000_000_000, amount/1000_000_000);

    require!(
        max_sol_cost >= required_lamports,
        PumpFunError::InvalidSolAmount
    );

    //transfer sol to vault
    transfer_sol(
        ctx.accounts.user.to_account_info(),
        ctx.accounts.bonding_curve.to_account_info(),
        required_lamports
        // required_lamports * (97 as u64) / 100,
    )?;
    // //transfer sol to fee account
    // let buy_fee = required_lamports * (2 as u64) / 100;
    // transfer_sol(
    //   ctx.accounts.user.to_account_info(),
    //   ctx.accounts.fee_account.to_account_info(),
    //   buy_fee,
    // )?;

    //transfer sol to craetor account
    // let buy_fee = required_lamports / 100;
    // transfer_sol(
    //   ctx.accounts.user.to_account_info(),
    //   ctx.accounts.fee_account.to_account_info(),
    //   buy_fee,
    // )?;

    //transfer token from vault to user
    let token_mint = ctx.accounts.token_mint.key();
    let vault_seeds = &[
        BONDING_CURVE_SEED,
        token_mint.as_ref(),
        &[ctx.accounts.bonding_curve.bump],
    ];
    let vault_signer_seeds = &[&vault_seeds[..]];

    transfer_token_from_vault_to_user(
        ctx.accounts.bonding_curve.to_account_info(),
        ctx.accounts.associted_bonding_curve.to_account_info(),
        ctx.accounts.associted_user_token_account.to_account_info(),
        ctx.accounts.token_mint.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        amount,
        decimals,
        vault_signer_seeds,
    )?;
    emit!(BuyEvent {
        mint: ctx.accounts.token_mint.key(),
        token_output: amount,
        sol_input: required_lamports,
        buyer: ctx.accounts.user.key()
    });
    Ok(())
  }
}
