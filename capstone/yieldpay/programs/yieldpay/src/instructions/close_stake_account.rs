use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    errors::YieldpayError,
    state::{
        Config, StakeAccount, UserAccount, Vault, WhitelistToken, CONFIG_SEED, STAKE_SEED,
        USER_SEED, VAULT_SEED, WHITELIST_SEED, YIELD_MINT_SEED,
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
        seeds=[VAULT_SEED.as_ref(),mint_x.key().as_ref(),config.key().as_ref()],
        bump=vault_x.bump
    )]
    pub vault_x: Account<'info, Vault>,

    #[account(
        associated_token::mint=mint_x,
        associated_token::authority=vault_x
    )]
    pub vault_x_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl <'info>  CloseStakeAccountContext <'info>{

    pub fn close_stake_account(&mut self)->Result<()>{
       
        require!(self.stake_account.amount_staked==0,YieldpayError::MustUnstakeFirst);
        require!(self.stake_account.is_active==false,YieldpayError::StakeAccountStillActive);
    
         self.stake_account.close(self.user.to_account_info())?;
        Ok(())
    }
}