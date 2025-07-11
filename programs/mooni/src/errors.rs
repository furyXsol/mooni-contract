use anchor_lang::prelude::error_code;

#[error_code]
pub enum MooniError {
  #[msg("AlreadyInitialized")]
  AlreadyInitialized,
  #[msg("Invalid Token")]
  InvalidToken,
  #[msg("Funding Already Raised")]
  AlreadyRaised,
  #[msg("Not enough available supply")]
  NotEnoughSuppply,
  #[msg("Incorrect value of SOL sent")]
  InvalidSolAmount,
  #[msg("BondingCurve: Input must be greater than zero")]
  InvalidInput,
  #[msg("Slippage Exceed")]
  SlippageExceed
}