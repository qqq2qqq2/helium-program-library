use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod circuit_breaker;

pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("5BAQuzGE1z8CTcrSdfbfdBF2fdXrwb4iMcxDMrvhz8L8");

#[program]
pub mod data_credits {
  use super::*;

  pub fn initialize_data_credits_v0(
    ctx: Context<InitializeDataCreditsV0>,
    args: InitializeDataCreditsArgsV0,
  ) -> Result<()> {
    instructions::initialize_data_credits_v0::handler(ctx, args)
  }

  pub fn mint_data_credits_v0(
    ctx: Context<MintDataCreditsV0>,
    args: MintDataCreditsArgsV0,
  ) -> Result<()> {
    instructions::mint_data_credits_v0::handler(ctx, args)
  }

  pub fn burn_in_use_data_credits_v0(
    ctx: Context<BurnInUseDataCreditsV0>,
    args: BurnInUseDataCreditsArgsV0,
  ) -> Result<()> {
    instructions::burn_in_use_data_credits_v0::handler(ctx, args)
  }

  pub fn burn_from_issuance_v0(
    ctx: Context<BurnFromIssuanceV0>,
    args: BurnFromIssuanceArgsV0,
  ) -> Result<()> {
    instructions::burn_from_issuance_v0::handler(ctx, args)
  }

  pub fn use_data_credits_v0(
    ctx: Context<UseDataCreditsV0>,
    args: UseDataCreditsArgsV0,
  ) -> Result<()> {
    instructions::use_data_credits_v0::handler(ctx, args)
  }
}
