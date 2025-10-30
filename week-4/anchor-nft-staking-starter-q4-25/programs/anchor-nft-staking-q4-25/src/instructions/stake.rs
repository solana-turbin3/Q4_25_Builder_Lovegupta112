use anchor_lang::prelude::*;
use mpl_core::{
    instructions::AddPluginV1CpiBuilder,
    types::{FreezeDelegate, Plugin, PluginAuthority},
    ID as CORE_PROGRAM_ID,
};

use crate::{
    errors::StakeError,
    state::{StakeAccount, StakeConfig, UserAccount},
};

#[derive(Accounts)]
pub struct Stake<'info> {
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

    // #[account(
    //     seeds = [b"collection_info", collection.key().as_ref()],
    //     bump = collection_info.bump,
    // )]
    // pub collection_info: Account<'info, CollectionInfo>,
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
    pub user_account:Account<'info,UserAccount>,

    #[account(address = CORE_PROGRAM_ID)]
    /// CHECK: Verified by address constraint
    pub core_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {

        require!(self.user_account.amount_staked<self.config.max_stake,StakeError::MaxStakeReached);
        //will set user stake account ------
        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.asset.key(),
            staked_at:Clock::get()?.unix_timestamp as u64,
            bump: bumps.stake_account
        });

        // let signers_seeds:&[&[&[u8]]]=&[&[b"collection",&self.collection.key().to_bytes(),&[self.collection_info.bump]]];

        self.user_account.amount_staked +=1;

        AddPluginV1CpiBuilder::new(&self.core_program.to_account_info())
        .asset(&self.asset.to_account_info())
        .collection(Some(&self.collection.to_account_info()))
        .authority(None)
        .init_authority(PluginAuthority::Address { address: self.stake_account.key() })
        .payer(&self.user.to_account_info())
        .system_program(&self.system_program.to_account_info())
        .plugin(Plugin::FreezeDelegate(FreezeDelegate { frozen: true }))
        .invoke()?;

        Ok(())
    }
}
