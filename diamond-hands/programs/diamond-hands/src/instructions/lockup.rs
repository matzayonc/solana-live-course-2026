use anchor_lang::prelude::*;

use crate::{constants::*, state::Locker};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + Locker::INIT_SPACE,
        seeds = [LOCKUP_SEED, &payer.key().to_bytes()],
        bump
    )]
    pub counter: Account<'info, Locker>,
    #[account(
        init,
        payer = payer,
        space = 0,
        owner = system_program.key(),
        seeds = [VAULT_SEED, &payer.key().to_bytes()],
        bump
    )]
    /// CHECK: system-owned PDA that only ever holds lamports
    pub vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
    *ctx.accounts.counter = Locker {
        amount,
        authority: ctx.accounts.payer.key(),
    };

    let cpi_accounts = anchor_lang::system_program::Transfer {
        from: ctx.accounts.payer.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(anchor_lang::system_program::ID, cpi_accounts);
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;

    msg!("Hello, world! Counter initialized");
    Ok(())
}
