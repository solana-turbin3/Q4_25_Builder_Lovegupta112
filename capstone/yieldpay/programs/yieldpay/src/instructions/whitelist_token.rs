use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    errors::YieldpayError,
    state::{Config, Vault, WhitelistToken, CONFIG_SEED, VAULT_SEED, WHITELIST_SEED},
};

#[derive(Accounts)]
pub struct WhitelistTokenContext<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub mint_x: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer=admin,
        space=WhitelistToken::DISCRIMINATOR.len()+WhitelistToken::INIT_SPACE,
        seeds=[WHITELIST_SEED.as_ref(),config.key().as_ref()],
        bump
    )]
    pub whitelisted_tokens: Account<'info, WhitelistToken>,

    #[account(
        init,
        payer=admin,
        space=Vault::DISCRIMINATOR.len()+Vault::INIT_SPACE,
        seeds=[VAULT_SEED.as_ref(),mint_x.key().as_ref(),config.key().as_ref()],
        bump
    )]
    pub vault_x: Account<'info, Vault>,

    #[account(
        init,
        payer=admin,
        associated_token::mint=mint_x,
        associated_token::authority=vault_x
    )]
    pub vault_x_ata: Account<'info, TokenAccount>,

    #[account(
        constraint = config.admin == admin.key() @ YieldpayError::UnauthorizedAccess,
       seeds=[CONFIG_SEED.as_ref()],
        bump=config.config_bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> WhitelistTokenContext<'info> {
    pub fn whitelist_token(&mut self, bumps: &WhitelistTokenContextBumps) -> Result<()> {
        if self.whitelisted_tokens.bump == 0 {
            self.whitelisted_tokens.bump = bumps.whitelisted_tokens;
            msg!("Whitelist initialized successfully.");
        }
        self.whitelisted_tokens.whitelist_mint(&self.mint_x.key())?;

        self.vault_x.set_inner(Vault {
            mint: self.mint_x.key(),
            token_account: self.vault_x_ata.key(),
            total_amount_staked: 0,
            bump: bumps.vault_x,
        });
        msg!("Vault created successfully for mint: {}", self.mint_x.key());
        Ok(())
    }
}
