use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

use crate::{constants::*, error::ErrorCode, state::Lock};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(address = lock.mint)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub owner_token_account: InterfaceAccount<'info, TokenAccount>,

    /// `has_one` ties the lock to the signer; the seeds already do, but the
    /// constraint keeps the guarantee explicit and survives a seed change.
    #[account(
        mut,
        close = owner,
        has_one = owner,
        seeds = [LOCK_SEED, owner.key().as_ref(), mint.key().as_ref()],
        bump = lock.bump,
    )]
    pub lock: Account<'info, Lock>,

    #[account(
        mut,
        seeds = [VAULT_SEED, lock.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = lock,
        token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handle_withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(now >= ctx.accounts.lock.unlocks_at, ErrorCode::StillLocked);

    let owner = ctx.accounts.owner.key();
    let mint = ctx.accounts.mint.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        LOCK_SEED,
        owner.as_ref(),
        mint.as_ref(),
        &[ctx.accounts.lock.bump],
    ]];

    // Drain whatever the vault actually holds rather than the recorded amount,
    // so a direct transfer into the vault cannot leave dust behind and block
    // the close below.
    let amount = ctx.accounts.vault.amount;

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.owner_token_account.to_account_info(),
                authority: ctx.accounts.lock.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    // The vault is empty now, so its rent can go back to the owner too.
    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.key(),
        CloseAccount {
            account: ctx.accounts.vault.to_account_info(),
            destination: ctx.accounts.owner.to_account_info(),
            authority: ctx.accounts.lock.to_account_info(),
        },
        signer_seeds,
    ))?;

    msg!("Withdrew {}", amount);
    Ok(())
}
