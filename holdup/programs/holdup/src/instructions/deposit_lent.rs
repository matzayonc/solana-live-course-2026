use std::ops::Div;

use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::transfer_checked,
    token_interface::{ Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Position, Pool, constants::*};

#[derive(Accounts)]
pub struct DepositLent<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [
            POOL_SEED, 
            &pool.lent_mint.key().to_bytes(), 
            &pool.collateral_mint.key().to_bytes(), 
        ], 
        bump,
        has_one = lent_mint
    )]
    pub pool: Account<'info, Pool>,


    #[account(mut, 
        seeds = [COUNTER_SEED, &payer.key().to_bytes()],
        bump
    )]
    pub position: Account<'info, Position>,


    #[account(
        mut,
        seeds = [LENT_TOKEN_VAULT_SEED, &lent_mint.key().to_bytes()],
        bump,
        token::mint = lent_mint,
        token::authority = pool,
        token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = lent_mint, token::authority = payer)]
    pub payer_ata_lent: InterfaceAccount<'info, TokenAccount>,

    pub lent_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handle_deposit_lent(ctx: Context<DepositLent>, amount: u64) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.payer_ata_lent.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
        mint: ctx.accounts.lent_mint.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts);
    transfer_checked(cpi_ctx, amount, ctx.accounts.lent_mint.decimals)?;


    let pool = &mut *ctx.accounts.pool;
    let position = &mut *ctx.accounts.position;

    if pool.total_deposited == 0 {
        pool.total_deposited = amount;
        pool.total_shares = amount;
        position.deposited_shares = amount;
    }

    let shares = amount.checked_mul(pool.total_shares).unwrap().checked_div( pool.total_deposited).unwrap();

    pool.total_deposited += amount;
    pool.total_shares += shares;
    position.deposited_shares += shares;


    msg!("Hello, world! Counter initialized");
    Ok(())
}
