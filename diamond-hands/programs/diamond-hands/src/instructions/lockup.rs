use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::transfer_checked,
    token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{constants::*, state::Locker};

#[derive(Accounts)]
pub struct Lockup<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: No assumptions about authority needed
    pub authority: UncheckedAccount<'info>,

    // Dane
    #[account(
        init,
        payer = payer,
        space = 8 + Locker::INIT_SPACE,
        seeds = [LOCKUP_SEED, &payer.key().to_bytes()],
        bump
    )]
    pub locker: Account<'info, Locker>,

    // Tokeny
    #[account(
        init,
        payer = payer,
        seeds = [VAULT_SEED, &payer.key().to_bytes()],
        bump,
        token::mint = mint,
        token::authority = locker,
        token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = mint, token::authority = payer)]
    pub payer_ata: InterfaceAccount<'info, TokenAccount>,

    // Stałe
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(ctx: Context<Lockup>, amount: u64) -> Result<()> {
    *ctx.accounts.locker = Locker {
        amount,
        authority: ctx.accounts.authority.key(),
        expires: Clock::get().unwrap().unix_timestamp + 1,
    };

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.payer_ata.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts);
    transfer_checked(cpi_ctx, amount, ctx.accounts.mint.decimals)?;

    msg!("Hello, world! Counter initialized");
    Ok(())
}
