pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("8iF1k7sBszyCcduwxUn73bQMNxne79BsVgoAnNtcdLiv");

#[program]
pub mod holdup {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn register(ctx: Context<Register>) -> Result<()> {
        crate::instructions::register::handle_register(ctx)
    }

    pub fn deposit_lent(ctx: Context<DepositLent>, amount: u64) -> Result<()> {
        crate::instructions::deposit_lent::handle_deposit_lent(ctx, amount)
    }

    pub fn deposit_collateral(ctx: Context<DepositCollateral>, amount: u64) -> Result<()> {
        crate::instructions::deposit_collateral::handle_deposit_collateral(ctx, amount)
    }

    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        crate::instructions::borrow::handle_borrow(ctx, amount)
    }

    pub fn update_price(ctx: Context<UpdatePrice>, price: u32) -> Result<()> {
        crate::instructions::update_price::handle_update_price(ctx, price)
    }
}
