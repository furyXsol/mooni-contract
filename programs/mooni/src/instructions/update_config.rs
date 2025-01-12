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
  pub config: Box<Account<'info, Config>>,
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
    Ok(())
  }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigParams {
  pub admin: Option<Pubkey>,
  pub fee_recipient: Option<Pubkey>,
}

