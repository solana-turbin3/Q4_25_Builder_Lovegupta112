use anchor_lang::prelude::*;
mod errors;
mod instructions;
mod state;

use errors::*;
use instructions::*;

declare_id!("212R2vTs1FXvMJFSrKkDENoH2SLMGX2gfcHg6P77mVnY");

#[program]
pub mod yieldpay {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeConfigContext>,
        args: InitializeConfigArgs,
    ) -> Result<()> {
        ctx.accounts.initialize_config(args, &ctx.bumps)
    }

    // pub fn onboard_user() -> Result<()> {}
    // pub fn onboard_merchant() -> Result<()> {}
}
