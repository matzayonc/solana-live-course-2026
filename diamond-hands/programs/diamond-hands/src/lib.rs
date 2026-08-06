pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EjymiXi4eWFKdCJmQKjHv9nFr3uFmaUYQJaicQjTbrcn");

#[program]
pub mod diamond_hands {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        crate::instructions::lockup::handle_initialize(ctx, amount)
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        crate::instructions::withdraw::handle_increment(ctx)
    }
}
