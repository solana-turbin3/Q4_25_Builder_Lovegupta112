use anchor_lang::prelude::*;
mod errors;
mod instructions;
mod state;

use errors::*;
use instructions::*;

declare_id!("212R2vTs1FXvMJFSrKkDENoH2SLMGX2gfcHg6P77mVnY");

#[program]
pub mod yieldpay {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeConfigContext>,
        args: InitializeConfigArgs,
    ) -> Result<()> {
        ctx.accounts.initialize_config(args, &ctx.bumps)
    }

    //whitelisting token and create vault for that token mint ------------
    pub fn whitelist_token(ctx: Context<WhitelistTokenContext>) -> Result<()> {
        ctx.accounts.whitelist_token(&ctx.bumps)
    }

    //onboarding user and merchant ----------
    pub fn onboard_user(ctx: Context<InitializeUserContext>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn onboard_merchant(ctx: Context<InitializeMerchantContext>,business_name: String,) -> Result<()> {
        ctx.accounts.initialize_merchant(business_name, &ctx.bumps)
    }

    //staking or deposit token --------

    pub fn stake(ctx: Context<StakeAccountContext>, amount: u64) -> Result<()> {
        ctx.accounts.stake(amount, &ctx.bumps)
    }

    pub fn claim_yield(ctx: Context<ClaimYieldAccountContext>) -> Result<()> {
        ctx.accounts.claim_yield()
    }

    pub fn pay_merchant(ctx: Context<PayMerchantContext>, amount: u64) -> Result<()> {
        ctx.accounts.pay_merchant(amount)
    }

    // pub fn unstake(ctx:Context<>,amount:u64)->Result<()>{
    //     Ok(())
    // }

    //TODO: add close account instructions
}
