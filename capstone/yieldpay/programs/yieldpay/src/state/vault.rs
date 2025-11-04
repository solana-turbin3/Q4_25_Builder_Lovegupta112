use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub mint:Pubkey,
    pub vault_ata:Pubkey,
    pub total_amount_staked:u64,
    pub bump:u8
}
