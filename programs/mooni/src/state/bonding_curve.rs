
use crate::*;

pub const BONDING_CURVE_SEED: &[u8] = b"mooni_bonding_curve";
// pub const WITHDRAWABLE_MIN_SOL_AMOUNT: u64 = 85000000000; //85 SOL

#[account]
#[derive(InitSpace)]
pub struct BondingCurve {
  pub creator: Pubkey,
  pub bump: u8
}