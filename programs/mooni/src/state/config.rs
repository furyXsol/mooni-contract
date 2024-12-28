
use crate::*;
pub const CONFIG_SEED: &[u8] = b"mooni_config";
pub const TOKEN_SEED: &[u8] = b"mooni_token";
// pub const WITHDRAWABLE_MIN_SOL_AMOUNT: u64 = 85000000000; //85 SOL

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey, //also authority to withdraw..
    pub bump: u8
}