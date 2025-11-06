use anchor_lang::prelude::*;
pub const VAULT_SEED: &str = "VAULT";

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub mint:Pubkey,
    pub token_account:Pubkey,
    pub total_amount_staked:u64,
    pub bump:u8
}
