use anchor_lang::prelude::*;

pub const MERCHANT_SEED: &str = "MERCHANT";

#[account]
#[derive(InitSpace)]
pub struct MerchantAccount {
    pub owner:Pubkey,
    #[max_len(32)]
    pub business_name:String,
    pub total_received:u64,
    pub bump:u8
}
