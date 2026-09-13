use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::transfer_checked,
    token_interface::{ Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Position, Pool, constants::*};

#[derive(Accounts)]
pub struct Borrow<'info> {
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


    // Tokeny
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

    // Stałe
    pub lent_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handle_borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {

    let price = ctx.accounts.pool.price as u64;

    let position = &mut  *ctx.accounts.position;
    position.borrowed += amount;

    let collateral_value = position.collateral * price / 10000;

    require_gte!(collateral_value, position.borrowed, crate::error::ErrorCode::NotEnoughCollateral);


    let cpi_accounts = TransferChecked {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.payer_ata_lent.to_account_info(),
        authority: ctx.accounts.pool.to_account_info(),
        mint: ctx.accounts.lent_mint.to_account_info(),
    };

    let seeds = &[
            POOL_SEED, 
            &ctx.accounts.pool.lent_mint.key().to_bytes(), 
            &ctx.accounts.pool.collateral_mint.key().to_bytes(), 
            &[ctx.bumps.pool]];
    let singer = &[&seeds[..]];

    let cpi_ctx= CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts)
        .with_signer(singer);

    transfer_checked(cpi_ctx, amount, ctx.accounts.lent_mint.decimals)?;


    msg!("Hello, world! Counter initialized");
    Ok(())
}
