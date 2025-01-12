use crate::*;
pub const CONFIG_SEED: &[u8] = b"mooni_config";

#[account]
#[derive(InitSpace)]
pub struct Config {
  pub is_initialized: bool,
  pub admin: Pubkey,
  pub fee_recipient: Pubkey,
  pub bump: u8
}