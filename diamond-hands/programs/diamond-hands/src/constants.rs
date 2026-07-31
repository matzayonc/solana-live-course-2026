use anchor_lang::prelude::*;

#[constant]
pub const LOCK_SEED: &[u8] = b"lock";

#[constant]
pub const VAULT_SEED: &[u8] = b"vault";

/// Upper bound on a lock duration: ten years. Guards against a fat-fingered
/// duration that would strand the deposit past any useful horizon.
#[constant]
pub const MAX_LOCK_SECONDS: i64 = 10 * 365 * 24 * 60 * 60;
