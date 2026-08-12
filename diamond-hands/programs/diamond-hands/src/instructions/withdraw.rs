use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{constants::*, error::ErrorCode, state::Locker};

const FEE_BPS: u16 = 5;

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub authority: Signer<'info>,

    // Dane
    #[account(
        mut, seeds = [LOCKUP_SEED, &payer.key().to_bytes()], bump,
        has_one = authority
    )]
    pub locker: Account<'info, Locker>,

    // Tokeny
    #[account(
        mut,
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

pub fn handle_increment(ctx: Context<Increment>) -> Result<()> {
    let locker = &mut ctx.accounts.locker;

    let timestamp = Clock::get().expect("Couldn't get clock").unix_timestamp;
    require_gte!(timestamp, locker.expires, ErrorCode::NotYetFinished);

    // Obliczenia
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.payer_ata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: locker.to_account_info(),
    };

    let amount = locker.amount;
    let fee = (amount as u128 * FEE_BPS as u128).div_ceil(10000);
    let to_withdraw = amount - u64::try_from(fee).unwrap_or(u64::MAX);

    locker.amount = 0;

    let payer_key = ctx.accounts.payer.key.to_bytes();
    let seeds = &[LOCKUP_SEED, payer_key.as_ref(), &[ctx.bumps.locker]];
    let signer = &[&seeds[..]];

    let cpi_ctx =
        CpiContext::new_with_signer(ctx.accounts.token_program.key(), cpi_accounts, signer);
    transfer_checked(cpi_ctx, to_withdraw, ctx.accounts.mint.decimals)?;

    msg!("Withdrew {} lamports", to_withdraw);
    Ok(())
}

#[test]
fn test() {
    let amount = 1000000;
    let fee = (amount as u128 * FEE_BPS as u128).div_ceil(10000);
    let to_withdraw = amount - u64::try_from(fee).unwrap_or(u64::MAX);

    assert_eq!(to_withdraw, 999500)
}
