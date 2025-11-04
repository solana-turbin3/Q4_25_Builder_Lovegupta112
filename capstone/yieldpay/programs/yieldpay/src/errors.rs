use anchor_lang::prelude::*;


#[error_code]
pub enum YieldpayError{
     #[msg("Deposit amount is below the required minimum threshold.")]
    DepositTooSmall,

    #[msg("Deposit exceeds the maximum staking limit allowed per user.")]
    ExceedsMaxStake,

    #[msg("Insufficient balance to complete this operation.")]
    InsufficientFunds,

    #[msg("Unauthorized access: the provided account is not the owner.")]
    UnauthorizedAccess,

    #[msg("Minimum yield period has not yet elapsed.")]
    MinPeriodNotMet,

    #[msg("Invalid payment amount: must be greater than zero and within limits.")]
    InvalidPaymentAmount,
}