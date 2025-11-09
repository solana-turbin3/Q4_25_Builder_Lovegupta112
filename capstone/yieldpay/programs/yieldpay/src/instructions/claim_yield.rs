use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{
    errors::YieldpayError,
    state::{
        Config, StakeAccount, UserAccount, WhitelistToken, CONFIG_SEED, STAKE_SEED, USER_SEED,
        WHITELIST_SEED, YIELD_MINT_SEED,
    },
};

#[derive(Accounts)]
pub struct ClaimYieldAccountContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
    constraint= whitelisted_tokens.is_token_whitelisted(&mint_x.key()) @ YieldpayError::TokenNotWhitelisted,
    )]
    pub mint_x: Account<'info, Mint>,

    #[account(
        mut,
        seeds=[USER_SEED.as_ref(),user.key().as_ref()],
        bump=user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
    seeds=[CONFIG_SEED.as_ref()],
    bump=config.config_bump
  )]
    pub config: Account<'info, Config>,

    #[account(
    mut,
    seeds=[STAKE_SEED.as_ref(),config.key().as_ref(),mint_x.key().as_ref(),user.key().as_ref()],
    bump=stake_account.bump
)]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds=[WHITELIST_SEED.as_ref(),config.key().as_ref()],
        bump=whitelisted_tokens.bump
    )]
    pub whitelisted_tokens: Account<'info, WhitelistToken>,

    #[account(
        seeds=[YIELD_MINT_SEED.as_ref(),config.key().as_ref()],
        bump=config.yield_bump,
    )]
    pub yield_mint: Account<'info, Mint>,

    #[account(
       init_if_needed,
       payer=user,
       associated_token::mint=yield_mint,
       associated_token::authority=user_account
    )]
    pub yield_mint_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> ClaimYieldAccountContext<'info> {
    pub fn claim_yield(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        let time_elapsed;
        if self.stake_account.last_yield_mint == 0 {
            time_elapsed = current_time - self.stake_account.staked_at;
        } else {
            time_elapsed = current_time - self.stake_account.last_yield_mint;
        }

        require!(
            self.config.yield_min_period < time_elapsed,
            YieldpayError::MinPeriodNotMet
        );

        let yield_amt = self.calculate_yield(time_elapsed)?;

        self.mint_yield(yield_amt)?;

        Ok(())
    }

    pub fn calculate_yield(&mut self, time_elapsed: u64) -> Result<u64> {
        let basis_points = 10_000;
        let sec_per_yr = 31_536_000;

        let yield_amount = self
            .stake_account
            .amount_staked
            .checked_mul(self.config.apy_bps)
            .ok_or(YieldpayError::Overflow)?
            .checked_mul(time_elapsed)
            .ok_or(YieldpayError::Overflow)?
            .checked_div(basis_points * sec_per_yr)
            .ok_or(YieldpayError::Overflow)?;

        Ok(yield_amount)
    }

    pub fn mint_yield(&mut self, yield_amt: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.yield_mint.to_account_info(),
            to: self.yield_mint_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[CONFIG_SEED.as_ref(), &[self.config.config_bump]]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, yield_amt)?;
        msg!(
            "Minted {} YIELD tokens for user: {}",
            yield_amt,
            self.user_account.key()
        );
        self.stake_account.last_yield_mint = Clock::get()?.unix_timestamp as u64;
        
        self.user_account.total_yield = self
            .user_account
            .total_yield
            .checked_add(yield_amt)
            .ok_or(YieldpayError::Overflow)?;

        self.stake_account.total_yield = self
            .stake_account
            .total_yield
            .checked_add(yield_amt)
            .ok_or(YieldpayError::Overflow)?;
        Ok(())
    }
}
