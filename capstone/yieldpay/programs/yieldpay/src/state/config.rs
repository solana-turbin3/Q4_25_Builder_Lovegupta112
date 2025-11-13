use anchor_lang::prelude::*;

pub const CONFIG_SEED: &[u8] = b"CONFIG";
pub const YIELD_MINT_SEED: &[u8] = b"YIELD";

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin:Pubkey,
    pub max_stake: u64,
    pub min_deposit: u64,
    pub yield_mint:Pubkey,
    pub total_users: u64,
    pub total_merchants: u64,
    pub yield_min_period: u64,
    pub apy_bps: u64,
    pub config_bump: u8,
    pub yield_bump: u8,
    pub yield_period_base:u64,
}


