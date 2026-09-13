use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::*, state::Position, Pool};

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + Position::INIT_SPACE,
        seeds = [COUNTER_SEED, &payer.key().to_bytes()],
        bump
    )]
    pub position: Account<'info, Position>,

    #[account(seeds = [
            POOL_SEED, 
            &pool.lent_mint.key().to_bytes(), 
            &pool.collateral_mint.key().to_bytes(), 
        ], 
        bump,
        has_one = collateral_mint
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = payer,
        seeds = [COLLATERAL_TOKEN_VAULT_SEED, &collateral_mint.key().to_bytes(), &payer.key().to_bytes()],
        bump,
        token::mint = collateral_mint,
        token::authority = pool,
        token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub collateral_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

pub fn handle_register(ctx: Context<Register>) -> Result<()> {
    *ctx.accounts.position = Position {
        deposited: 0,
        collateral: 0,
        borrowed: 0,
        authority: ctx.accounts.payer.key(),
    };

    msg!("Hello, world! Counter initialized");
    Ok(())
}
