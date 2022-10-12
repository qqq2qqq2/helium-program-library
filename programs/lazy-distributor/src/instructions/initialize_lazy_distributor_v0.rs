use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::instruction::AuthorityType;
use anchor_spl::token::{set_authority, SetAuthority};
use anchor_spl::token::{Mint, Token};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct InitializeLazyDistributorArgsV0 {
  pub oracles: Vec<OracleConfigV0>,
  pub authority: Pubkey,
}

#[derive(Accounts)]
#[instruction(args: InitializeLazyDistributorArgsV0)]
pub struct InitializeLazyDistributorV0<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(
    init,
    payer = payer,
    space = 60 + std::mem::size_of::<LazyDistributorV0>() + std::mem::size_of_val(&*args.oracles),
    seeds = ["lazy_distributor".as_bytes(), rewards_mint.key().as_ref()],
    bump,
  )]
  pub lazy_distributor: Box<Account<'info, LazyDistributorV0>>,
  #[account(mut)]
  pub rewards_mint: Box<Account<'info, Mint>>,
  pub mint_authority: Signer<'info>,
  pub freeze_authority: Signer<'info>,
  pub system_program: Program<'info, System>,
  pub token_program: Program<'info, Token>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
  ctx: Context<InitializeLazyDistributorV0>,
  args: InitializeLazyDistributorArgsV0,
) -> Result<()> {
  set_authority(
    CpiContext::new(
      ctx.accounts.token_program.to_account_info(),
      SetAuthority {
        account_or_mint: ctx.accounts.rewards_mint.to_account_info(),
        current_authority: ctx.accounts.mint_authority.to_account_info(),
      },
    ),
    AuthorityType::MintTokens,
    Some(ctx.accounts.lazy_distributor.key()),
  )?;
  set_authority(
    CpiContext::new(
      ctx.accounts.token_program.to_account_info(),
      SetAuthority {
        account_or_mint: ctx.accounts.rewards_mint.to_account_info(),
        current_authority: ctx.accounts.freeze_authority.to_account_info(),
      },
    ),
    AuthorityType::FreezeAccount,
    Some(ctx.accounts.lazy_distributor.key()),
  )?;

  ctx.accounts.lazy_distributor.set_inner(LazyDistributorV0 {
    rewards_mint: ctx.accounts.rewards_mint.key(),
    oracles: args.oracles,
    version: 0,
    authority: args.authority,
    bump_seed: ctx.bumps["lazy_distributor"],
  });

  Ok(())
}
