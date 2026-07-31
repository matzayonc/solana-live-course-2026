use anchor_lang::prelude::*;

/// A single time-locked deposit, one per (owner, mint).
///
/// The account doubles as the authority over its vault token account, so the
/// tokens can only move in a transaction this program signs for.
#[account]
#[derive(InitSpace)]
pub struct Lock {
    /// The only address allowed to withdraw.
    pub owner: Pubkey,
    /// Mint of the locked tokens.
    pub mint: Pubkey,
    /// Amount held in the vault, in the mint's base units.
    pub amount: u64,
    /// Unix timestamp before which `withdraw` fails.
    pub unlocks_at: i64,
    /// Bump of this account's PDA, stored so signing need not re-derive it.
    pub bump: u8,
}
