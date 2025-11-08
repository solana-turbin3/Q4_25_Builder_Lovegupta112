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
pub struct StakeAccountContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
    constraint= whitelisted_tokens.is_token_whitelisted(&mint_x.key()) @ YieldpayError::TokenNotWhitelisted,
    )]
    pub mint_x: Account<'info, Mint>,

    #[account(
        mut,
        seeds=[USER_SEED.as_ref(),user.key().as_ref()],
        bump
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
    init_if_needed,
    payer=user,
    space=StakeAccount::DISCRIMINATOR.len()+StakeAccount::INIT_SPACE,
    seeds=[STAKE_SEED.as_ref(),config.key().as_ref(),mint_x.key().as_ref(),user.key().as_ref()],
    bump
)]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds=[WHITELIST_SEED.as_ref(),config.key().as_ref()],
        bump
    )]
    pub whitelisted_tokens: Account<'info, WhitelistToken>,

    #[account(
        seeds=[VAULT_SEED.as_ref(),mint_x.key().as_ref(),config.key().as_ref()],
        bump
    )]
    pub vault_x: Account<'info, Vault>,

    #[account(
        associated_token::mint=mint_x,
        associated_token::authority=vault_x
    )]
    pub vault_x_ata: Account<'info, TokenAccount>,

    #[account(
        seeds=[YIELD_MINT_SEED.as_ref(),config.key().as_ref()],
        bump,
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

impl<'info> StakeAccountContext<'info> {
    pub fn stake(&mut self, amount: u64, bumps: &StakeAccountContextBumps) -> Result<()> {
        require!(
            self.config.max_stake < self.user_account.total_amount_staked + amount,
            YieldpayError::ExceedsMaxStake
        );

        require!(
            amount < self.config.min_deposit as u64,
            YieldpayError::DepositTooSmall
        );

        //if it's first time deposit  -------
        if self.stake_account.amount_staked == 0 {
            self.deposit(amount);
            self.updated_stake_account(amount, bumps);
        } else {
            //stake amount already exist -------
            //calulate yield for prev principal
            //then deposit new pricipal ---
        }

        Ok(())
    }
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.user_x_ata.to_account_info(),
            to: self.vault_x_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount);

        msg!("User deposited {} mint_x token successfully.", { amount });

        Ok(())
    }

    pub fn updated_stake_account(
        &mut self,
        amount: u64,
        bumps: &StakeAccountContextBumps,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        self.stake_account.set_inner(StakeAccount {
            owner: self.user_account.key(),
            mint: self.mint_x.key(),
            amount_staked: amount,
            total_yield: 0,
            staked_at: current_time, //
            last_yield_mint: 0,
            bump: bumps.stake_account,
        });
        Ok(())
    }

    pub fn calculate_yield(&mut self) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        let time_diff = current_time.saturating_sub(self.stake_account.staked_at);
        let elapsed_time=current_time.saturating_sub(self.stake_account.last_yield_mint);

        if time_diff < self.config.yield_min_period {
            Ok(())
        }
        else{
            // let yield_amount=self.stake_account.amount_staked.saturating_mul(self.config.apy_bps as u64)
            // .saturating_mul(time_diff)
            // .saturating_div()
            Ok(())
        }
    }
}
