use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    errors::YieldpayError,
    state::{
        Config, StakeAccount, UserAccount, Vault, WhitelistToken, CONFIG_SEED, STAKE_SEED,
        USER_SEED, VAULT_SEED, WHITELIST_SEED,
    },
};

#[derive(Accounts)]
pub struct UnstakeContext<'info> {
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
        mut,
        associated_token::mint=mint_x,
        associated_token::authority=user
    )]
    pub user_x_ata: Account<'info, TokenAccount>,

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
        mut,
        seeds=[VAULT_SEED.as_ref(),mint_x.key().as_ref(),config.key().as_ref()],
        bump=vault_x.bump
    )]
    pub vault_x: Account<'info, Vault>,

    #[account(
        mut,
        associated_token::mint=mint_x,
        associated_token::authority=vault_x
    )]
    pub vault_x_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> UnstakeContext<'info> {
    pub fn unstake(&mut self, amount: u64) -> Result<()> {
        require!(
            amount <= self.stake_account.amount_staked,
            YieldpayError::InvalidAmount
        );
        require!(amount > 0, YieldpayError::InvalidAmount);

        require!(
            self.stake_account.is_active,
            YieldpayError::StakeAccountInactive
        );
        require!(
            self.stake_account.amount_staked > 0,
            YieldpayError::NoActiveStake
        );

          if amount == self.stake_account.amount_staked {
            self.stake_account.last_yield_mint = 0;
            self.stake_account.is_active = false;
            msg!("Stake account fully unstaked and deactivated");
        }

        let program_id = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault_x_ata.to_account_info(),
            to: self.user_x_ata.to_account_info(),
            authority: self.vault_x.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            VAULT_SEED.as_ref(),
            self.mint_x.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.vault_x.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(program_id, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        self.stake_account.amount_staked = self
            .stake_account
            .amount_staked
            .checked_sub(amount)
            .ok_or(YieldpayError::Underflow)?;

        self.user_account.total_amount_staked = self
            .user_account
            .total_amount_staked
            .checked_sub(amount)
            .ok_or(YieldpayError::Underflow)?;

        self.vault_x.total_amount_staked = self
            .vault_x
            .total_amount_staked
            .checked_sub(amount)
            .ok_or(YieldpayError::Underflow)?;


        msg!("Unstaked {} tokens for user: {}", amount, self.user.key());

        Ok(())
    }
}
