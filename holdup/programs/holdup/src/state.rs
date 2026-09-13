use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Position {
    pub deposited_shares: u64,
    pub collateral: u64,
    pub borrowed: u64,
    pub authority: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub lent_mint: Pubkey,
    pub collateral_mint: Pubkey,
    pub price_authority: Pubkey,
    pub price: u32, // lent_token_price / collateral_token_price in bps
    pub total_deposited: u64,
    pub total_shares: u64,
}
