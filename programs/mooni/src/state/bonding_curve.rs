
use crate::*;

pub const BONDING_CURVE_SEED: &[u8] = b"mooni_bonding_curve";

#[account]
#[derive(InitSpace)]
pub struct BondingCurve {
  pub creator: Pubkey,
  pub bump: u8
}