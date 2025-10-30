use anchor_lang::prelude::*;
use mpl_core::{
    instructions::{RemovePluginV1CpiBuilder, UpdatePluginV1CpiBuilder},
    types::{FreezeDelegate, Plugin, PluginType},
    ID as CORE_PROGRAM_ID,
};

use crate::{
    errors::StakeError,
    state::{StakeAccount, StakeConfig, UserAccount},
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        constraint = !asset.data_is_empty() @ StakeError::AssetNotInitialized
    )]
    /// CHECK: Verified by mpl-core
    pub asset: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = collection.owner == &CORE_PROGRAM_ID @ StakeError::InvalidCollection,
        constraint = !collection.data_is_empty() @ StakeError::CollectionNotInitialized
    )]
    /// CHECK: Verified by mpl-core
    pub collection: UncheckedAccount<'info>,

    #[account(
        seeds=[b"config"],
        bump=config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        init,
        payer=user,
        space=StakeConfig::DISCRIMINATOR.len()+ StakeConfig::INIT_SPACE,
        seeds=[b"stake",config.key().as_ref(),asset.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds=[b"user",user.key().as_ref()],
        bump=user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(address = CORE_PROGRAM_ID)]
    /// CHECK: Verified by address constraint
    pub core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let staked_time = Clock::get()?.unix_timestamp as u64 - self.stake_account.staked_at;

        require!(
            staked_time >= self.config.freeze_period,
            StakeError::FreezePeriodNotPassed
        );

        self.user_account.amount_staked -= 1;

        self.user_account.points += self.config.points_per_stake as u32 * staked_time as u32;

        UpdatePluginV1CpiBuilder::new(&self.core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.user.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .authority(None)
            .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: false }))
            .invoke()?;

        RemovePluginV1CpiBuilder::new(&self.core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.user.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .plugin_type(PluginType::FreezeDelegate)
            .invoke()?;

        Ok(())
    }
}
