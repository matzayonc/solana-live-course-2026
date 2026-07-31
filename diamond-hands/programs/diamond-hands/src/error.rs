use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Duration must be greater than zero")]
    ZeroDuration,
    #[msg("Duration exceeds the maximum lock period")]
    DurationTooLong,
    #[msg("Lock has not expired yet")]
    StillLocked,
}
