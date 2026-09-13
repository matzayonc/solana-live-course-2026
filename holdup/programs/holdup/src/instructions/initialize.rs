use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{constants::*, Pool};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// CHECK: Any account can be an authority
    pub price_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + Pool::INIT_SPACE,
        seeds = [
            POOL_SEED, 
            &lent_mint.key().to_bytes(), 
            &collateral_mint.key().to_bytes(),
        ],
        bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = payer,
        seeds = [LENT_TOKEN_VAULT_SEED, &lent_mint.key().to_bytes()],
        bump,
        token::mint = lent_mint,
        token::authority = pool,
        token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub lent_mint: InterfaceAccount<'info, Mint>,
    pub collateral_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    *ctx.accounts.pool = Pool {
        lent_mint: ctx.accounts.lent_mint.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        price_authority: ctx.accounts.price_authority.key(),
        price: 1,
    };

    Ok(())
}
