use anchor_lang::prelude::*;

#[constant]
pub const COUNTER_SEED: &[u8] = b"counter";

#[constant]
pub const POOL_SEED: &[u8] = b"pool";

#[constant]
pub const LENT_TOKEN_VAULT_SEED: &[u8] = b"lent_vault";

#[constant]
pub const COLLATERAL_TOKEN_VAULT_SEED: &[u8] = b"collateral_vault";

#[constant]
pub const HELLO_WORLD_LAMPORTS: u64 = 1;

#[constant]
pub const MAX_COUNT: u64 = 10;
