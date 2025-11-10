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

    #[msg("Invalid amount: must be greater than zero and within limits.")]
    InvalidAmount,

    #[msg("Token List is full")]
    TokenListFull,

    #[msg("Token is alreday whitelisted")]
    TokenAlreadyWhitelisted,

    #[msg("Token is not whitelisted")]
    TokenNotWhitelisted,

    #[msg("Already Initialized")]
    AlreadyInitialized,

    #[msg("Amount Overflow detected")]
    Overflow,

    #[msg("Amount Underflow detected")]
    Underflow,

    #[msg("Stake account is not active")]
    StakeAccountInactive,

    #[msg("Stake account is still active")]
    StakeAccountStillActive,

    #[msg("No active stake found for this mint")]
    NoActiveStake,

    #[msg("User must unstake total stake amount.")]
    MustUnstakeFirst
}
