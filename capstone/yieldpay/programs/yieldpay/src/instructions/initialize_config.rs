use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{
    state::{Config, CONFIG_SEED,YIELD_MINT_SEED},
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeConfigArgs {
    pub max_stake: u64,
    pub min_deposit: u64,
    pub total_users: u64,
    pub total_merchants: u64,
    pub yield_min_period: u64,
    pub apy_bps: u64,
    pub yield_period_base:u64
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

    #[account(
        init,
        payer=admin,
        seeds=[YIELD_MINT_SEED.as_ref(),config.key().as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=config
    )]
    pub yield_mint:Account<'info,Mint>,
 
    pub token_program:Program<'info,Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeConfigContext<'info> {
    pub fn initialize_config(
        &mut self,
        args: InitializeConfigArgs,
        bumps: &InitializeConfigContextBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            admin:self.admin.key(),
            max_stake: args.max_stake,
            min_deposit: args.min_deposit,
            total_users: args.total_users,
            total_merchants: args.total_merchants,
            yield_min_period: args.yield_min_period,
            apy_bps: args.apy_bps,
            yield_period_base:args.yield_period_base,
            config_bump: bumps.config,
            yield_bump:bumps.yield_mint,
            yield_mint:self.yield_mint.key(),
        });
        msg!("config is initialized successfully.");
        Ok(())
    }
}
