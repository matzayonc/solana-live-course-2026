use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::transfer_checked,
    token_interface::{ Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Position, Pool, constants::*};

#[derive(Accounts)]
pub struct DepositCollateral<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [
            POOL_SEED, 
            &pool.lent_mint.key().to_bytes(), 
            &pool.collateral_mint.key().to_bytes(), 
        ], 
        bump,
        has_one = collateral_mint
    )]
    pub pool: Account<'info, Pool>,


    #[account(mut, 
        seeds = [COUNTER_SEED, &payer.key().to_bytes()],
        bump
    )]
    pub position: Account<'info, Position>,


    #[account(
        mut,
        seeds = [COLLATERAL_TOKEN_VAULT_SEED, &collateral_mint.key().to_bytes(), &payer.key().to_bytes()],
        bump,
        token::mint = collateral_mint,
        token::authority = pool,
        token::token_program = token_program
    )]
    pub vault_collateral: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, token::mint = collateral_mint, token::authority = payer)]
    pub payer_ata_collateral: InterfaceAccount<'info, TokenAccount>,

    pub collateral_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handle_deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.payer_ata_collateral.to_account_info(),
        to: ctx.accounts.vault_collateral.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
        mint: ctx.accounts.collateral_mint.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.key(), cpi_accounts);
    transfer_checked(cpi_ctx, amount, ctx.accounts.collateral_mint.decimals)?;

    ctx.accounts.position.collateral += amount;

    msg!("Hello, world! Counter initialized");
    Ok(())
}
