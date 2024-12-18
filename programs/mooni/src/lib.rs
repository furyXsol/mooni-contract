
use anchor_lang::prelude::*;
mod instructions;
pub mod state;
pub mod utils;
mod events;
mod errors;
pub mod msg_codec;

use instructions::*;
use state::*;
use utils::*;
use events::*;
use errors::*;

declare_id!("BuvPLifeNiA7peR4bQ9yCARW54y3HpibeRaiUsqQWwX2");

//decimal: 9


// pub const INIT_SUPPLY:u128 = 100_000_000_000_000_000;


// pub const P0:u64 = 25; //0.000000025SOL
// pub const A:u64 = 12; // alpha
// pub const T_WITH_DECIMAL:u128 = 500_000_000; //with decimal
// pub const K: f64 = 0.000000024; // A/T_WITH_DECIMAL;
pub const T:u64 = 1_200_000_000_000_000_000; //1200M  (200M for liquidity)
pub const P0: u64 = 33; // 33 lamports
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

  pub fn buy(mut ctx: Context<Buy>, amount: u64, max_sol_cost: u64) -> Result<()> {
    Buy::apply(&mut ctx, amount, max_sol_cost)
  }

  pub fn buy_in_sol(mut ctx: Context<BuyInSol>, amount_min: u64, sol: u64) -> Result<()> {
    BuyInSol::apply(&mut ctx, amount_min, sol)
  }

  pub fn withdraw(mut ctx: Context<Withdraw>) -> Result<()> {
    Withdraw::apply(&mut ctx)
  }
}
