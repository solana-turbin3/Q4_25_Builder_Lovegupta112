use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};

use crate::{
    errors::YieldpayError,
    state::{
        Config, StakeAccount, UserAccount, Vault, WhitelistToken, CONFIG_SEED, STAKE_SEED,
        USER_SEED, VAULT_SEED, WHITELIST_SEED, YIELD_MINT_SEED,
    },
};

// #[derive(Accounts)]
// pub struct StakeAccountContext<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>,

//     #[account(
//     constraint= whitelisted_tokens.is_token_whitelisted(&mint_x.key()) @ YieldpayError::TokenNotWhitelisted,
//     )]
//     pub mint_x: Account<'info, Mint>,

//     #[account(
//         mut,
//         seeds=[USER_SEED.as_ref(),user.key().as_ref()],
//         bump=user_account.bump
//     )]
//     pub user_account: Account<'info, UserAccount>,

//     #[account(
//         mut,
//         associated_token::mint=mint_x,
//         associated_token::authority=user
//     )]
//     pub user_x_ata: Account<'info, TokenAccount>,

//     #[account(
//     seeds=[CONFIG_SEED.as_ref()],
//     bump=config.config_bump
//   )]
//     pub config: Account<'info, Config>,

//     #[account(
//     init_if_needed,
//     payer=user,
//     space=StakeAccount::DISCRIMINATOR.len()+StakeAccount::INIT_SPACE,
//     seeds=[STAKE_SEED.as_ref(),config.key().as_ref(),mint_x.key().as_ref(),user_account.key().as_ref()],
//     bump
// )]
//     pub stake_account: Account<'info, StakeAccount>,

//     #[account(
//         seeds=[WHITELIST_SEED.as_ref(),config.key().as_ref()],
//         bump=whitelisted_tokens.bump
//     )]
//     pub whitelisted_tokens: Account<'info, WhitelistToken>,

//     #[account(
//         mut,
//         seeds=[VAULT_SEED.as_ref(),mint_x.key().as_ref(),config.key().as_ref()],
//         bump=vault_x.bump
//     )]
//     pub vault_x: Account<'info, Vault>,

//     #[account(
//         mut,
//         associated_token::mint=mint_x,
//         associated_token::authority=vault_x
//     )]
//     pub vault_x_ata: Account<'info, TokenAccount>,

//     #[account(
//         seeds=[YIELD_MINT_SEED.as_ref(),config.key().as_ref()],
//         bump=config.yield_bump,
//     )]
//     pub yield_mint: Account<'info, Mint>,

//     #[account(
//        init_if_needed,
//        payer=user,
//        associated_token::mint=yield_mint,
//        associated_token::authority=user_account
//     )]
//     pub yield_mint_user_ata: Account<'info, TokenAccount>,

//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub associated_token_program: Program<'info, AssociatedToken>,
// }

// impl<'info> StakeAccountContext<'info> {
//     pub fn stake(&mut self, amount: u64, bumps: &StakeAccountContextBumps) -> Result<()> {
//         require!(
//             self.user_account.total_amount_staked + amount <= self.config.max_stake,
//             YieldpayError::ExceedsMaxStake
//         );

//         require!(
//             amount >= self.config.min_deposit as u64,
//             YieldpayError::DepositTooSmall
//         );
        // let stake_info = self.stake_account.to_account_info();
        // let data = stake_info.try_borrow_data()?;
        // let is_new_account = data.len() < 8 || &data[..8] != StakeAccount::DISCRIMINATOR;

//         self.deposit(amount)?;

//         if is_new_account {
//             msg!("Creating new StakeAccount for user {}", self.user.key());
//             self.stake_account.set_inner(StakeAccount {
//                 is_active: true,
//                 owner: self.user_account.key(),
//                 mint: self.mint_x.key(),
//                 amount_staked: amount,
//                 total_yield: 0,
//                 staked_at: Clock::get()?.unix_timestamp as u64,
//                 last_yield_mint: 0,
//                 bump: bumps.stake_account,
//             });
//         } else {
//             //stake amount already exist -------
//             //calulate yield for prev principal
//             //then deposit new pricipal ---
//             msg!(
//                 "Updating existing StakeAccount for user {}",
//                 self.user.key()
//             );
//             let yield_amt = self.calculate_yield()?;
//             if yield_amt > 0 {
//                 self.mint_yield(yield_amt)?;
//             }
//             self.stake_account.amount_staked = self
//                 .stake_account
//                 .amount_staked
//                 .checked_add(amount)
//                 .ok_or(YieldpayError::Overflow)?;
//         }

//         self.user_account.total_amount_staked = self
//             .user_account
//             .total_amount_staked
//             .checked_add(amount)
//             .ok_or(YieldpayError::Overflow)?;

//         msg!(
//             "Stake successful: user={}, mint={}, amount={}",
//             self.user.key(),
//             self.mint_x.key(),
//             amount
//         );
//         Ok(())
//     }
//     pub fn deposit(&mut self, amount: u64) -> Result<()> {
//         let cpi_accounts = Transfer {
//             from: self.user_x_ata.to_account_info(),
//             to: self.vault_x_ata.to_account_info(),
//             authority: self.user.to_account_info(),
//         };

//         let cpi_program = self.token_program.to_account_info();

//         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
//         transfer(cpi_ctx, amount)?;
//         self.vault_x.total_amount_staked = self
//             .vault_x
//             .total_amount_staked
//             .checked_add(amount)
//             .ok_or(YieldpayError::Overflow)?;
//         msg!("User deposited {} mint_x token successfully.", amount);

//         Ok(())
//     }

//     pub fn calculate_yield(&mut self) -> Result<u64> {
//         let current_time = Clock::get()?.unix_timestamp as u64;

//         let basis_points: u64 = 10_000;
//         let sec_per_yr: u64 = 31_536_000;

//         let time_elapsed;

//         if self.stake_account.last_yield_mint == 0 {
//             time_elapsed = current_time - self.stake_account.staked_at;
//         } else {
//             time_elapsed = current_time - self.stake_account.last_yield_mint;
//         }

//         if time_elapsed < self.config.yield_min_period {
//             return Ok(0);
//         }

//         let denominator = basis_points
//             .checked_mul(sec_per_yr)
//             .ok_or(YieldpayError::Overflow)?;

//         let yield_amount = self
//             .stake_account
//             .amount_staked
//             .checked_mul(self.config.apy_bps)
//             .ok_or(YieldpayError::Overflow)?
//             .checked_mul(time_elapsed)
//             .ok_or(YieldpayError::Overflow)?
//             .checked_div(denominator)
//             .ok_or(YieldpayError::Overflow)?;

//         Ok(yield_amount)
//     }

//     pub fn mint_yield(&mut self, yield_amt: u64) -> Result<()> {
//         let cpi_program = self.token_program.to_account_info();

//         let cpi_accounts = MintTo {
//             mint: self.yield_mint.to_account_info(),
//             to: self.yield_mint_user_ata.to_account_info(),
//             authority: self.config.to_account_info(),
//         };

//         let signer_seeds: &[&[&[u8]]] = &[&[CONFIG_SEED.as_ref(), &[self.config.config_bump]]];

//         let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

//         mint_to(cpi_ctx, yield_amt)?;
//         msg!(
//             "Minted {} YIELD tokens for user: {}",
//             yield_amt,
//             self.user_account.key()
//         );
//         self.stake_account.last_yield_mint = Clock::get()?.unix_timestamp as u64;
//         self.user_account.total_yield = self
//             .user_account
//             .total_yield
//             .checked_add(yield_amt)
//             .ok_or(YieldpayError::Overflow)?;
//         self.stake_account.total_yield = self
//             .stake_account
//             .total_yield
//             .checked_add(yield_amt)
//             .ok_or(YieldpayError::Overflow)?;
//         Ok(())
//     }
// }

//-----converting one instruction into two instructions 
//1. initialize stake
//2. add stake 

#[derive(Accounts)]
pub struct InitializeStakeContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        constraint = whitelisted_tokens.is_token_whitelisted(&mint_x.key()) @ YieldpayError::TokenNotWhitelisted,
    )]
    pub mint_x: Account<'info, Mint>,

    #[account(
        mut,
        seeds=[USER_SEED.as_ref(), user.key().as_ref()],
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
        init,
        payer=user,
        space=StakeAccount::DISCRIMINATOR.len() + StakeAccount::INIT_SPACE,
        seeds=[STAKE_SEED.as_ref(), config.key().as_ref(), mint_x.key().as_ref(), user_account.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds=[WHITELIST_SEED.as_ref(), config.key().as_ref()],
        bump=whitelisted_tokens.bump
    )]
    pub whitelisted_tokens: Account<'info, WhitelistToken>,

    #[account(
        mut,
        seeds=[VAULT_SEED.as_ref(), mint_x.key().as_ref(), config.key().as_ref()],
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

#[derive(Accounts)]
pub struct AddStakeContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint_x: Account<'info, Mint>,

    #[account(
        mut,
        seeds=[USER_SEED.as_ref(), user.key().as_ref()],
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
        mut,
        seeds=[CONFIG_SEED.as_ref()],
        bump=config.config_bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds=[STAKE_SEED.as_ref(), config.key().as_ref(), mint_x.key().as_ref(), user_account.key().as_ref()],
        bump=stake_account.bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds=[WHITELIST_SEED.as_ref(), config.key().as_ref()],
        bump=whitelisted_tokens.bump
    )]
    pub whitelisted_tokens: Account<'info, WhitelistToken>,

    #[account(
        mut,
        seeds=[VAULT_SEED.as_ref(), mint_x.key().as_ref(), config.key().as_ref()],
        bump=vault_x.bump
    )]
    pub vault_x: Account<'info, Vault>,

    #[account(
        mut,
        associated_token::mint=mint_x,
        associated_token::authority=vault_x
    )]
    pub vault_x_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[YIELD_MINT_SEED.as_ref(), config.key().as_ref()],
        bump=config.yield_bump,
    )]
    pub yield_mint: Account<'info, Mint>,

       #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=yield_mint,
        associated_token::authority=user_account
    )]
    pub yield_mint_user_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


impl<'info> InitializeStakeContext<'info> {
    pub fn initialize_stake(&mut self, amount: u64, bumps: &InitializeStakeContextBumps) -> Result<()> {
        require!(
            amount >= self.config.min_deposit as u64,
            YieldpayError::DepositTooSmall
        );

        require!(
            amount <= self.config.max_stake,
            YieldpayError::ExceedsMaxStake
        );

        // Transfer tokens from user to vault
        self.deposit(amount)?;

        // Initialize the stake account
        self.stake_account.set_inner(StakeAccount {
            is_active: true,
            owner: self.user_account.key(),
            mint: self.mint_x.key(),
            amount_staked: amount,
            total_yield: 0,
            staked_at: Clock::get()?.unix_timestamp as u64,
            last_yield_mint: 0,
            bump: bumps.stake_account,
        });

        // Update user's total staked amount
        self.user_account.total_amount_staked = self
            .user_account
            .total_amount_staked
            .checked_add(amount)
            .ok_or(YieldpayError::Overflow)?;

        msg!(
            "Initialized stake: user={}, mint={}, amount={}",
            self.user.key(),
            self.mint_x.key(),
            amount
        );

        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.user_x_ata.to_account_info(),
            to: self.vault_x_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(cpi_ctx, amount)?;

        self.vault_x.total_amount_staked = self
            .vault_x
            .total_amount_staked
            .checked_add(amount)
            .ok_or(YieldpayError::Overflow)?;

        msg!("User deposited {} tokens successfully.", amount);
        Ok(())
    }
}

impl<'info> AddStakeContext<'info> {
    pub fn add_stake(&mut self, amount: u64) -> Result<()> {

         require!(
            self.whitelisted_tokens.is_token_whitelisted(&self.mint_x.key()),
            YieldpayError::TokenNotWhitelisted
        );
         require!(
            self.stake_account.is_active,
            YieldpayError::StakeAccountInactive
        );

        require!(
            self.stake_account.owner == self.user_account.key(),
            YieldpayError::UnauthorizedAccess
        );

        require!(
            self.user_account.total_amount_staked + amount <= self.config.max_stake,
            YieldpayError::ExceedsMaxStake
        );

        require!(
            amount >= self.config.min_deposit as u64,
            YieldpayError::DepositTooSmall
        );

        // Calculate and mint yield for existing stake
        let yield_amt = self.calculate_yield()?;
        if yield_amt > 0 {
            self.mint_yield(yield_amt)?;
        }

        // Transfer new stake amount
        self.deposit(amount)?;

        // Update stake amount
        self.stake_account.amount_staked = self
            .stake_account
            .amount_staked
            .checked_add(amount)
            .ok_or(YieldpayError::Overflow)?;

        // Update user's total staked amount
        self.user_account.total_amount_staked = self
            .user_account
            .total_amount_staked
            .checked_add(amount)
            .ok_or(YieldpayError::Overflow)?;

        msg!(
            "Added to stake: user={}, mint={}, amount={}",
            self.user.key(),
            self.mint_x.key(),
            amount
        );

        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.user_x_ata.to_account_info(),
            to: self.vault_x_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(cpi_ctx, amount)?;

        self.vault_x.total_amount_staked = self
            .vault_x
            .total_amount_staked
            .checked_add(amount)
            .ok_or(YieldpayError::Overflow)?;

        msg!("User deposited {} tokens successfully.", amount);
        Ok(())
    }

    pub fn calculate_yield(&self) -> Result<u64> {
        let current_time = Clock::get()?.unix_timestamp as u64;

        let basis_points: u64 = 10_000;
        let sec_per_yr: u64 = self.config.yield_period_base;

        let time_elapsed = if self.stake_account.last_yield_mint == 0 {
            current_time
                .checked_sub(self.stake_account.staked_at)
                .ok_or(YieldpayError::Overflow)?
        } else {
            current_time
                .checked_sub(self.stake_account.last_yield_mint)
                .ok_or(YieldpayError::Overflow)?
        };

        if time_elapsed < self.config.yield_min_period {
            return Ok(0);
        }

        let denominator = basis_points
            .checked_mul(sec_per_yr)
            .ok_or(YieldpayError::Overflow)?;

        let yield_amount = self
            .stake_account
            .amount_staked
            .checked_mul(self.config.apy_bps)
            .ok_or(YieldpayError::Overflow)?
            .checked_mul(time_elapsed)
            .ok_or(YieldpayError::Overflow)?
            .checked_div(denominator)
            .ok_or(YieldpayError::Overflow)?;

        Ok(yield_amount)
    }

    pub fn mint_yield(&mut self, yield_amt: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: self.yield_mint.to_account_info(),
            to: self.yield_mint_user_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[CONFIG_SEED.as_ref(), &[self.config.config_bump]]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        mint_to(cpi_ctx, yield_amt)?;

        msg!(
            "Minted {} YIELD tokens for user: {}",
            yield_amt,
            self.user_account.key()
        );

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

        // Update last yield mint timestamp
        self.stake_account.last_yield_mint = Clock::get()?.unix_timestamp as u64;
        Ok(())
    }
}