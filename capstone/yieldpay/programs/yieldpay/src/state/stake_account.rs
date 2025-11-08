use anchor_lang::prelude::*;

pub const STAKE_SEED: &[u8] = b"STAKE";


#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner:Pubkey,
    pub mint:Pubkey,
    pub amount_staked:u64,
    pub total_yield:u64,
    pub staked_at:u64,
    pub last_yield_mint:u64,   
     pub bump:u8
}
