use anchor_lang::prelude::*;
mod instructions;
mod state;
mod utils;
mod events;
mod errors;

use instructions::*;
use state::*;
use utils::*;
use events::*;
use errors::*;

declare_id!("HUnqKTdaeyLsKUzEdEaXNjdGQEh8TWXsvRhb4MZ8KoMq");

pub const T:u64 = 1_000_000_000; //1000M  (Total supply)
pub const LIQUIDITY:u64 = 360_000_000; // 360M  (For liquidity)

#[program]
pub mod mooni {
    use super::*;

    pub fn create_config(mut ctx: Context<CreateConfig>, params: CreateConfigParams) -> Result<()> {
        CreateConfig::apply(&mut ctx, &params)
    }

    pub fn update_config(mut ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
        UpdateConfig::apply(&mut ctx, &params)
    }

    // create meme token
    pub fn create_token(
        mut ctx: Context<CreateToken>,
        params: CreateTokenParams,
    ) -> Result<()> {
        CreateToken::apply(&mut ctx, &params)
    }

    pub fn buy_in_sol(mut ctx: Context<BuyInSol>, amount_min: u64, sol: u64) -> Result<()> {
        BuyInSol::apply(&mut ctx, amount_min, sol)
    }

    pub fn withdraw(mut ctx: Context<Withdraw>) -> Result<()> {
        Withdraw::apply(&mut ctx)
    }
}