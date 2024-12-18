
use crate::*;
pub const CONFIG_SEED: &[u8] = b"pumpfun_config";
pub const TOKEN_SEED: &[u8] = b"pumpfun_token";
// pub const TOKEN_MINT_AUTHORITY_SEED: &str = "pumpfun_mint_authority";
// pub const WITHDRAWABLE_MIN_SOL_AMOUNT: u64 = 85000000000; //85 SOL

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey, //also authority to withdraw..
    pub fee_recipient: Pubkey,
    // pub liquidity: Pubkey,
    // pub buy_fee: u32,
    // pub mint_fee: u32,
    // pub create_token_fee: u32, //sol-lamports 0.1SOL
    pub bump: u8
}