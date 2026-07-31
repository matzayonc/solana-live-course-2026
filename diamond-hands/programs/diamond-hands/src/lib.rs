pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6PisfLVTpenAEvaZmSSswrf2hZy6F3YZ6zvHqBZDMXna");

/// Every Anchor account is prefixed with an 8-byte discriminator.
pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[program]
pub mod diamond_hands {
    use super::*;

    /// Move `amount` tokens into a program-owned vault, unwithdrawable for
    /// `duration` seconds.
    pub fn lock(ctx: Context<LockTokens>, amount: u64, duration: i64) -> Result<()> {
        crate::instructions::lock::handle_lock(ctx, amount, duration)
    }

    /// Return the full balance to the owner once the lock has expired, closing
    /// the vault and lock accounts and refunding their rent.
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        crate::instructions::withdraw::handle_withdraw(ctx)
    }
}
