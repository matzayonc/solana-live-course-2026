use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::{constants::*, error::ErrorCode, state::Lock};

#[derive(Accounts)]
pub struct LockTokens<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    /// Where the tokens come from. Ownership is enforced, so a caller cannot
    /// drain somebody else's account by passing it here.
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub owner_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = owner,
        space = crate::ANCHOR_DISCRIMINATOR + Lock::INIT_SPACE,
        seeds = [LOCK_SEED, owner.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub lock: Account<'info, Lock>,

    /// The vault is owned by the `lock` PDA, so only this program can move the
    /// tokens out — and only through `withdraw`.
    #[account(
        init,
        payer = owner,
        seeds = [VAULT_SEED, lock.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = lock,
        token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handle_lock(ctx: Context<LockTokens>, amount: u64, duration: i64) -> Result<()> {
    require!(amount > 0, ErrorCode::ZeroAmount);
    require!(duration > 0, ErrorCode::ZeroDuration);
    require!(duration <= MAX_LOCK_SECONDS, ErrorCode::DurationTooLong);

    let now = Clock::get()?.unix_timestamp;

    // `checked_add` rather than `+`: a hostile duration must not wrap the
    // timestamp into the past and unlock the deposit immediately.
    let unlocks_at = now
        .checked_add(duration)
        .ok_or(ErrorCode::DurationTooLong)?;

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.owner_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    ctx.accounts.lock.set_inner(Lock {
        owner: ctx.accounts.owner.key(),
        mint: ctx.accounts.mint.key(),
        amount,
        unlocks_at,
        bump: ctx.bumps.lock,
    });

    msg!("Locked {} until {}", amount, unlocks_at);
    Ok(())
}
