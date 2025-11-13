use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    errors::YieldpayError,
    state::{
        Config, MerchantAccount, StakeAccount, UserAccount, WhitelistToken, CONFIG_SEED,
        MERCHANT_SEED, STAKE_SEED, USER_SEED, WHITELIST_SEED, YIELD_MINT_SEED,
    },
};

#[derive(Accounts)]
pub struct PayMerchantContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    ///CHECK: merchant is validated via merchant_account constraint
    pub merchant: UncheckedAccount<'info>,

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
        mut,
        constraint=merchant_account.owner==merchant.key() @YieldpayError::UnauthorizedAccess ,
        seeds=[MERCHANT_SEED.as_ref(),merchant.key().as_ref()],
        bump=merchant_account.bump
    )]
    pub merchant_account: Account<'info, MerchantAccount>,

    #[account(
    seeds=[CONFIG_SEED.as_ref()],
    bump=config.config_bump
  )]
    pub config: Account<'info, Config>,

    #[account(
    mut,
    seeds=[STAKE_SEED.as_ref(),config.key().as_ref(),mint_x.key().as_ref(),user_account.key().as_ref()],
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
        mut,
       associated_token::mint=yield_mint,
       associated_token::authority=user_account
    )]
    pub yield_mint_user_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
       associated_token::mint=yield_mint,
       associated_token::authority=merchant_account
    )]
    pub yield_mint_merchant_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> PayMerchantContext<'info> {
    pub fn pay_merchant(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, YieldpayError::InvalidAmount);
        require!(
            self.yield_mint_user_ata.amount >= amount,
            YieldpayError::InsufficientFunds
        );

        let cpi_accounts = Transfer {
            from: self.yield_mint_user_ata.to_account_info(),
            to: self.yield_mint_merchant_ata.to_account_info(),
            authority: self.user_account.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            USER_SEED.as_ref(),
            self.user.to_account_info().key.as_ref(),
            &[self.user_account.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer(cpi_ctx, amount)?;

        //for merchant level ----

        self.merchant_account.total_received = self
            .merchant_account
            .total_received
            .checked_add(amount)
            .ok_or(YieldpayError::Overflow)?;

        //for stake_account level -----

        self.stake_account.total_yield = self
            .stake_account
            .total_yield
            .checked_sub(amount)
            .ok_or(YieldpayError::Underflow)?;

        // for user_account level ---
        // self.user_account.total_yield = self
        //     .user_account
        //     .total_yield
        //     .checked_sub(amount)
        //     .ok_or(YieldpayError::Underflow)?;

        self.user_account.total_yield_spent = self
            .user_account
            .total_yield_spent
            .checked_add(amount)
            .ok_or(YieldpayError::Overflow)?;

        msg!(
            "Paid {} YIELD from user_account {} → merchant_account {}",
            amount,
            self.user_account.key(),
            self.merchant_account.key()
        );
        Ok(())
    }
}
