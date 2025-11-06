use anchor_lang::prelude::*;

#[error_code]
pub enum YieldpayError {
    #[msg("Deposit amount is below the required minimum threshold.")]
    DepositTooSmall,

    #[msg("Deposit exceeds the maximum staking limit allowed per user.")]
    ExceedsMaxStake,

    #[msg("Insufficient balance to complete this operation.")]
    InsufficientFunds,

    #[msg("Unauthorized access: this account is not the owner or authorized authority.")]
    UnauthorizedAccess,

    #[msg("Minimum yield period has not yet elapsed.")]
    MinPeriodNotMet,

    #[msg("Invalid payment amount: must be greater than zero and within limits.")]
    InvalidPaymentAmount,
    
    #[msg("Token List is full")]
    TokenListFull,

    #[msg("Token is alreday whitelisted")]
    TokenAlreadyWhitelisted,

    #[msg("Already Initialized")]
    AlreadyInitialized,
}
