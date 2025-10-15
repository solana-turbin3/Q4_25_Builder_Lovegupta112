use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_id!("HahaynbxzxXHaMAwb9ssJNFg34ZWLc8BpoSQBYEJu918");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amt: u64) -> Result<()> {
        //msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.initialize(amt, &ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amt: u64) -> Result<()> {
        ctx.accounts.deposit(amt)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amt: u64) -> Result<()> {
        ctx.accounts.withdraw(amt)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init,payer=user,space=8+VaultState::INIT_SPACE,seeds=[b"vault",user.key().as_ref()],bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds=[b"vault".as_ref(),vault_state.key().as_ref()],bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, amount: u64, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.amount = amount;
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}

// -----------------deposit---------------------------

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut, seeds=[b"vault".as_ref(),user.key().as_ref()],bump=vault_state.vault_bump)]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut,seeds=[b"vault".as_ref(),vault_state.key().as_ref()],bump=vault_state.state_bump)]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let from_user = self.user.to_account_info();
        let to_vault = self.vault.to_account_info();
        let programId = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: from_user,
            to: to_vault,
        };

        let cpi_ctx = CpiContext::new(programId, cpi_accounts);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

// -------------------withdraw ---------------------

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut,seeds=[b"vault".as_ref(),user.key().as_ref()],bump=vault_state.vault_bump)]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut,seeds=[b"vault".as_ref(),vault_state.key().as_ref()],bump=vault_state.state_bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let from_vault = self.vault.to_account_info();
        let to_user = self.user.to_account_info();
        let vault_state = self.vault_state.to_account_info();
        let program_id = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: from_vault,
            to: to_user,
        };

        let seeds = &[
            b"vault".as_ref(),
            vault_state.key.as_ref(),
            &[self.vault_state.state_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(program_id, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub amount: u64,
    pub vault_bump: u8,
    pub state_bump: u8,
}
