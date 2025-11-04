use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

use crate::{
    errors::YieldpayError,
    state::{Config, CONFIG_SEED},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeConfigArgs {
    pub max_stake: u64,
    pub min_deposit: u8,
    pub total_users: u64,
    pub total_merchants: u64,
    pub yield_min_period: u64,
    pub apy_bps: u8,
}

#[derive(Accounts)]
pub struct InitializeConfigContext<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
    init,
    payer=admin,
    space=Config::DISCRIMINATOR.len()+Config::INIT_SPACE,
    seeds=[CONFIG_SEED.as_ref()],
    bump
  )]
    pub config: Account<'info, Config>,

    #[account()]
    pub yield_mint:Account<'info,Mint>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfigContext<'info> {
    pub fn initialize_config(
        &mut self,
        args: InitializeConfigArgs,
        bumps: &InitializeConfigContextBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            max_stake: args.max_stake,
            min_deposit: args.min_deposit,
            total_users: args.total_users,
            total_merchants: args.total_merchants,
            yield_min_period: args.yield_min_period,
            apy_bps: args.apy_bps,
            bump: bumps.config,
            yield_mint:self.yield_mint.key()
        });
        msg!("successfully initialize_config");
        Ok(())
    }
}
