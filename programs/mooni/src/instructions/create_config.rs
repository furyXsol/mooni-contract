use std::ops::DerefMut;
use crate::*;

#[derive(Accounts)]
pub struct CreateConfig<'info> {
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
    require!(!config.is_initialized, MooniError::AlreadyInitialized);
    config.admin = params.admin;
    config.fee_recipient = params.fee_recipient;
    config.bump = ctx.bumps.config;
    config.is_initialized = true;
    Ok(())
  }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateConfigParams {
  pub admin: Pubkey,
  pub fee_recipient: Pubkey,
}
