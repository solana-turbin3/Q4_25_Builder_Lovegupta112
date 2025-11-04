use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MerchantAccount {
    pub owner:Pubkey,
    #[max_len(32)]
    pub business_name:String,
    pub total_received:u64,
    
}
