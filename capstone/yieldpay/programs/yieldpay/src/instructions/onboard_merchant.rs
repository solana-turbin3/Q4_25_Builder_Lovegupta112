use anchor_lang::prelude::*;

use crate::state::{MerchantAccount, MERCHANT_SEED};

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
    system_program: Program<'info, System>,
}

impl<'info> InitializeMerchantContext<'info> {
    pub fn initialize_merchant(&mut self,business_name:String, bumps: &InitializeMerchantContextBumps) -> Result<()> {
        self.merchant_account.set_inner(MerchantAccount {
            owner: self.merchant.key(),
            business_name,
            total_received:0,
            bump:bumps.merchant_account,
        });
        //todo: increase merchant inconfig -----also check in tests state of config

        msg!("Merchant {} onboarderd successfully.", self.merchant.key());

        Ok(())
    }
}
