use anchor_lang::prelude::*;

pub const CONFIG_SEED: &str = "CONFIG";

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub max_stake: u64,
    pub min_deposit: u8,
    pub yield_mint:Pubkey,
    pub total_users: u64,
    pub total_merchants: u64,
    pub yield_min_period: u64,
    pub apy_bps: u8,
    pub bump: u8,
}
