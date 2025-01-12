use crate::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount},
};

#[derive(Accounts)]
pub struct BuyInSol<'info> {
  pub token_mint: Box<InterfaceAccount<'info, Mint>>,
  /// CHECK
  #[account(
    mut,
    seeds = [
      BONDING_CURVE_SEED,
      token_mint.key().as_ref()
    ],
    bump = bonding_curve.bump,
  )]
  pub bonding_curve: Box<Account<'info, BondingCurve>>,

  #[account(
    mut,
    associated_token::mint = token_mint,
    associated_token::authority = bonding_curve,
    token::token_program = token_program,
  )]
  pub associted_bonding_curve: Box<InterfaceAccount<'info, TokenAccount>>,

  #[account(
    init_if_needed,
    associated_token::mint = token_mint,
    associated_token::authority = user,
    token::token_program = token_program,
    payer = user,
  )]
  pub associted_user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,


  /// CHECK
  #[account(
    mut,
    seeds=[
      CONFIG_SEED
    ],
    bump = config.bump
  )]
  pub config: Box<Account<'info, Config>>,

  /// CHECK
  #[account(
    mut,
    address = config.fee_recipient
  )]
  pub fee_account: UncheckedAccount<'info>,

  #[account(mut)]
    pub user: Signer<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl BuyInSol<'_> {
  pub fn apply(ctx: &mut Context<BuyInSol>, amount_min: u64, sol: u64) -> Result<()> {
    let decimals = ctx.accounts.token_mint.decimals;
    require!(decimals == 0, MooniError::InvalidToken);
    // check to ensure funding goal is not met
    require!(
        ctx.accounts.associted_bonding_curve.amount > LIQUIDITY,
        MooniError::AlreadyRaised
    );
    let current_supply =
      T - ctx.accounts.associted_bonding_curve.amount;

    let fee_sol = sol /100;

    let token_amount_to_purchased = calculate_token_amount(current_supply, sol - fee_sol);
    require!(token_amount_to_purchased >= amount_min, MooniError::SlippageExceed);

    let available_qty = ctx.accounts.associted_bonding_curve.amount as u128;

    require!((token_amount_to_purchased as u128) <= available_qty, MooniError::NotEnoughSuppply);

    //transfer sol to bonding_curve
    transfer_sol(
        ctx.accounts.user.to_account_info(),
        ctx.accounts.bonding_curve.to_account_info(),
        sol,
    )?;

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
        token_amount_to_purchased,
        decimals,
        vault_signer_seeds,
    )?;
    //transfer fee
    transfer_sol(
      ctx.accounts.user.to_account_info(),
      ctx.accounts.fee_account.to_account_info(),
      fee_sol
    )?;

    emit!(BuyEvent {
        mint: ctx.accounts.token_mint.key(),
        token_output: token_amount_to_purchased,
        sol_input: sol - fee_sol,
        buyer: ctx.accounts.user.key()
    });
    Ok(())
  }
}
