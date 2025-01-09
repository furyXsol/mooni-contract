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
pub mod admin {
    use anchor_lang::declare_id;

    declare_id!("Deuj4zBPtc6gMwVviDG8yUZbXgL4fEWduVaVxNM2FdRL");
}
pub mod fee {
    use anchor_lang::declare_id;

    declare_id!("Deuj4zBPtc6gMwVviDG8yUZbXgL4fEWduVaVxNM2FdRL");
}

#[program]
pub mod mooni {
    use super::*;

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