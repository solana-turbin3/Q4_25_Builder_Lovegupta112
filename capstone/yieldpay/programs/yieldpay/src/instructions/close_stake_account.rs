use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount},
};

use crate::{
    errors::YieldpayError,
    state::{
        Config, StakeAccount, UserAccount, WhitelistToken, CONFIG_SEED, STAKE_SEED,
        USER_SEED, WHITELIST_SEED,
    },
};

#[derive(Accounts)]
pub struct CloseStakeAccountContext<'info> {
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
    bump=stake_account.bump,
    close=user
)]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds=[WHITELIST_SEED.as_ref(),config.key().as_ref()],
        bump=whitelisted_tokens.bump
    )]
    pub whitelisted_tokens: Account<'info, WhitelistToken>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CloseStakeAccountContext<'info> {
    pub fn close_stake_account(&mut self) -> Result<()> {
        require!(
            self.stake_account.amount_staked == 0,
            YieldpayError::MustUnstakeFirst
        );
        require!(
            self.stake_account.is_active == false,
            YieldpayError::StakeAccountStillActive
        );

        self.stake_account.close(self.user.to_account_info())?;
        Ok(())
    }
}
