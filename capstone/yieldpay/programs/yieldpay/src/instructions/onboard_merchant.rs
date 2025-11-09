use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{Config, MerchantAccount, CONFIG_SEED, MERCHANT_SEED, YIELD_MINT_SEED};

#[derive(Accounts)]
pub struct InitializeMerchantContext<'info> {
    #[account(mut)]
    pub merchant: Signer<'info>,

    #[account(
        init,
        payer=merchant,
        space=MerchantAccount::DISCRIMINATOR.len()+MerchantAccount::INIT_SPACE,
        seeds=[MERCHANT_SEED.as_ref(),merchant.key().as_ref()],
        bump
    )]
    pub merchant_account: Account<'info, MerchantAccount>,

    #[account(
    mut,
    seeds=[CONFIG_SEED.as_ref()],
    bump=config.config_bump
  )]
    pub config: Account<'info, Config>,

    #[account(
        seeds=[YIELD_MINT_SEED.as_ref(),config.key().as_ref()],
        bump=config.yield_bump,
    )]
    pub yield_mint: Account<'info, Mint>,

    #[account(
       init_if_needed,
       payer=merchant,
       associated_token::mint=yield_mint,
       associated_token::authority=merchant_account
    )]
    pub yield_mint_merchant_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl<'info> InitializeMerchantContext<'info> {
    pub fn initialize_merchant(
        &mut self,
        business_name: String,
        bumps: &InitializeMerchantContextBumps,
    ) -> Result<()> {
        self.merchant_account.set_inner(MerchantAccount {
            owner: self.merchant.key(),
            business_name,
            total_received: 0,
            bump: bumps.merchant_account,
        });
        self.config.total_merchants += 1;
        msg!("Merchant {} onboarderd successfully.", self.merchant.key());
        Ok(())
    }
}
