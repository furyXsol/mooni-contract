use std::ops::DerefMut;
use crate::*;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    /// Admin address
  #[account(
    mut,
    address = config.admin
  )]
  pub authority: Signer<'info>,

  #[account(
    mut,
    seeds=[
      CONFIG_SEED,
    ],
    bump = config.bump
  )]
  pub config: Account<'info, Config>,
}

impl UpdateConfig<'_> {
  pub fn apply(ctx: &mut Context<UpdateConfig>, params: &UpdateConfigParams) -> Result<()> {
    let config = ctx.accounts.config.deref_mut();

    if params.admin.is_some() {
      config.admin = params.admin.unwrap();
    }
    if params.fee_recipient.is_some() {
      config.fee_recipient = params.fee_recipient.unwrap();
    }
    // if params.liquidity.is_some() {
    //   config.liquidity = params.liquidity.unwrap();
    // }
    // if params.buy_fee.is_some() {
    //   config.buy_fee = params.buy_fee.unwrap();
    // }
    // if params.mint_fee.is_some() {
    //   config.mint_fee = params.mint_fee.unwrap();
    // }
    // if params.create_token_fee.is_some() {
    //   config.create_token_fee = params.create_token_fee.unwrap();
    // }
    Ok(())
  }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigParams {
  pub admin: Option<Pubkey>,
  pub fee_recipient: Option<Pubkey>,
  // pub liquidity: Option<Pubkey>,
  // pub buy_fee: Option<u32>,
  // pub mint_fee: Option<u32>,
  // pub create_token_fee: Option<u32>,
}

