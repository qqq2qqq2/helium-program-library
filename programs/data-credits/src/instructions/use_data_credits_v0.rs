use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
  associated_token::AssociatedToken,
  token::{
    freeze_account, thaw_account, transfer, FreezeAccount, Mint, ThawAccount, Token, TokenAccount,
    Transfer,
  },
};
use helium_sub_daos::{DaoV0, SubDaoV0};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct UseDataCreditsArgsV0 {
  amount: u64,
}

#[derive(Accounts)]
#[instruction(args: UseDataCreditsArgsV0)]
pub struct UseDataCreditsV0<'info> {
  #[account(
    init, // prevents from reinit attack
    payer = payer,
    space = 60 + std::mem::size_of::<DataCreditsV0>(),
    seeds = ["in_use_dc".as_bytes(), sub_dao.key().as_ref(), owner.key().as_ref()],
    bump,
  )]
  pub in_use_data_credits: Box<Account<'info, InUseDataCreditsV0>>,
  #[account(
    has_one = dc_mint,
    seeds = ["dc".as_bytes(), dc_mint.key().as_ref()],
    bump = data_credits.data_credits_bump
  )]
  pub data_credits: Box<Account<'info, DataCreditsV0>>,
  pub dc_mint: Box<Account<'info, Mint>>,
  #[account(
    has_one = dc_mint
  )]
  pub dao: Box<Account<'info, DaoV0>>,
  #[account(
    has_one = dao
  )]
  pub sub_dao: Box<Account<'info, SubDaoV0>>,

  pub owner: Signer<'info>,

  #[account(
    mut,
    associated_token::authority = owner,
    associated_token::mint = dc_mint
  )]
  pub from_account: Box<Account<'info, TokenAccount>>,

  /// CHECK: Verified by cpi
  #[account(
    init_if_needed,
    payer = payer,
    seeds = ["in_use_dc_account".as_bytes(), in_use_data_credits.key().as_ref()],
    bump,
    token::mint = dc_mint,
    token::authority = in_use_data_credits
  )]
  pub escrow_account: Account<'info, TokenAccount>,

  #[account(mut)]
  pub payer: Signer<'info>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub token_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<UseDataCreditsV0>, args: UseDataCreditsArgsV0) -> Result<()> {
  ctx
    .accounts
    .in_use_data_credits
    .set_inner(InUseDataCreditsV0 {
      data_credits: ctx.accounts.data_credits.key(),
      owner: ctx.accounts.owner.key(),
      sub_dao: ctx.accounts.sub_dao.key(),
      escrow_account: ctx.accounts.escrow_account.key(),
      bump: ctx.bumps["in_use_data_credits"],
    });

  let signer_seeds: &[&[&[u8]]] = &[&[
    "dc".as_bytes(),
    ctx.accounts.dc_mint.to_account_info().key.as_ref(),
    &[ctx.accounts.data_credits.data_credits_bump],
  ]];

  if ctx.accounts.from_account.is_frozen() {
    thaw_account(CpiContext::new_with_signer(
      ctx.accounts.token_program.to_account_info(),
      ThawAccount {
        account: ctx.accounts.from_account.to_account_info(),
        mint: ctx.accounts.dc_mint.to_account_info(),
        authority: ctx.accounts.data_credits.to_account_info(),
      },
      signer_seeds,
    ))?;
  }

  transfer(
    CpiContext::new(
      ctx.accounts.token_program.to_account_info(),
      Transfer {
        from: ctx.accounts.from_account.to_account_info(),
        to: ctx.accounts.escrow_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
      },
    ),
    args.amount,
  )?;

  freeze_account(CpiContext::new_with_signer(
    ctx.accounts.token_program.to_account_info(),
    FreezeAccount {
      account: ctx.accounts.from_account.to_account_info(),
      mint: ctx.accounts.dc_mint.to_account_info(),
      authority: ctx.accounts.data_credits.to_account_info(),
    },
    signer_seeds,
  ))?;

  Ok(())
}
