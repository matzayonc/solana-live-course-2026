use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Locker {
    pub amount: u64,
    pub expires: i64,
    pub authority: Pubkey,
}
