use anchor_lang::prelude::*;

use crate::{constants::*, error::ErrorCode, state::Locker};

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [LOCKUP_SEED, &authority.key().to_bytes()], bump)]
    pub locker: Account<'info, Locker>,
    #[account(mut, seeds = [VAULT_SEED, &authority.key().to_bytes()], bump)]
    /// CHECK:
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_increment(ctx: Context<Increment>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.locker.authority,
        ctx.accounts.authority.key(),
        ErrorCode::Unauthorized,
    );

    let cpi_accounts = anchor_lang::system_program::Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.authority.to_account_info(),
    };

    let payer_key = ctx.accounts.authority.key.to_bytes();
    let seeds = &[VAULT_SEED, payer_key.as_ref(), &[ctx.bumps.vault]];
    let signer = &[&seeds[..]];

    let cpi_ctx =
        CpiContext::new_with_signer(anchor_lang::system_program::ID, cpi_accounts, signer);
    anchor_lang::system_program::transfer(cpi_ctx, ctx.accounts.locker.amount)?;

    msg!("Withdrew {} lamports", ctx.accounts.locker.amount);
    ctx.accounts.locker.amount = 0;
    Ok(())
}
