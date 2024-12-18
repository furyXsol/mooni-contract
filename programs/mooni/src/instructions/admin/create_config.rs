use std::ops::DerefMut;
use crate::*;

#[derive(Accounts)]
pub struct CreateConfig<'info> {
  /// authority to withdraw from bonding_curve_account
  #[account(
    mut,
  )]
  pub payer: Signer<'info>,

  #[account(
    init,
    seeds=[
      CONFIG_SEED,
    ],
    bump,
    payer = payer,
    space = 8 + Config::INIT_SPACE
  )]
  pub config: Account<'info, Config>,

  pub system_program: Program<'info, System>,
}
impl CreateConfig<'_> {
  pub fn apply(ctx: &mut Context<CreateConfig>, params: &CreateConfigParams) -> Result<()> {
    let config = ctx.accounts.config.deref_mut();
    config.admin = params.admin;
    config.fee_recipient = params.fee_recipient;
    // config.liquidity = params.liquidity;
    // config.buy_fee = params.buy_fee; // 100:   1%
    // config.mint_fee = params.mint_fee; // 100: 1%
    // config.create_token_fee = params.create_token_fee; //0.1SOL -lamports
    config.bump = ctx.bumps.config;
    Ok(())
  }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateConfigParams {
  pub admin: Pubkey,
  pub fee_recipient: Pubkey,
  // pub liquidity: Pubkey,
  // pub buy_fee: u32,
  // pub mint_fee: u32,
  // pub create_token_fee: u32,
}
