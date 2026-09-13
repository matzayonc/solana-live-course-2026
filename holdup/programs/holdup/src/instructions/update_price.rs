use anchor_lang::prelude::*;

use crate::{constants::*, Pool};

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub price_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            POOL_SEED, 
            &pool.lent_mint.to_bytes(), 
            &pool.collateral_mint.key().to_bytes(), 
        ],
        bump
    )]
    pub pool: Account<'info, Pool>,
}

pub fn handle_update_price(ctx: Context<UpdatePrice>, price: u32) -> Result<()> {
    ctx.accounts.pool.price = price;

    Ok(())
}
