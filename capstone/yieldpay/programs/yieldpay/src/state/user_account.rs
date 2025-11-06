use anchor_lang::prelude::*;

pub const USER_SEED: &str = "USER";

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub owner:Pubkey,
    pub total_yield:u64,
    pub total_amount_staked:u64,
    pub bump:u8
}

