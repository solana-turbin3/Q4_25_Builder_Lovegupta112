use anchor_lang::prelude::*;

use crate::state::{Config, UserAccount, CONFIG_SEED, USER_SEED};

#[derive(Accounts)]
pub struct InitializeUserContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer=user,
        space=UserAccount::DISCRIMINATOR.len()+UserAccount::INIT_SPACE,
        seeds=[USER_SEED.as_ref(),user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
    mut,
    seeds=[CONFIG_SEED.as_ref()],
    bump=config.config_bump
  )]
    pub config: Account<'info, Config>,
    system_program: Program<'info, System>,
}

impl<'info> InitializeUserContext<'info> {
    pub fn initialize_user(&mut self, bumps: &InitializeUserContextBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            owner: self.user.key(),
            total_yield: 0,
            total_amount_staked: 0,
            total_yield_spent:0,
            bump: bumps.user_account,
            created_at:Clock::get()?.unix_timestamp as u64
        });

        self.config.total_users += 1;
        msg!("User {} onboarderd successfully.", self.user.key());

        Ok(())
    }
}
