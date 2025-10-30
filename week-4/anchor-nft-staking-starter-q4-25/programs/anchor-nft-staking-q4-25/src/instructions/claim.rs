use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, MintTo, Token, TokenAccount, mint_to},
};

use crate::state::{StakeConfig, UserAccount};


//we will create mint token and will create mint ata for user 
//according to points of user stake, we will provide tokens..

#[derive(Accounts)]
pub struct Claim<'info> {

    #[account(mut)]
    pub user:Signer<'info>,

    #[account(
        mut,
        seeds=[b"user",user.key().as_ref()],
        bump=user_account.bump
    )]
    pub user_account:Account<'info,UserAccount>,

    #[account(
     seeds=[b"config"],
     bump=config.bump
    )]
    pub config:Account<'info,StakeConfig>,

    #[account(
        seeds=[b"rewards",config.key().as_ref()],
        bump=config.rewards_bump,
        mint::authority=config,
        mint::decimals=6
    )]
    pub reward_mint:Account<'info,Mint>,
   
   #[account(
    mut,
    associated_token::mint = reward_mint,
    associated_token::authority = config
   )]
   pub rewards_ata:Account<'info,TokenAccount>,

   pub associated_token_program:Program<'info,AssociatedToken>,
   pub token_program:Program<'info,Token>,
   pub system_program:Program<'info,System>
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {


    let total_rewards_points=self.user_account.points;
      
    
    let mint_accounts=MintTo{
        mint:self.reward_mint.to_account_info(),
        to:self.rewards_ata.to_account_info(),
        authority:self.config.to_account_info()
    };
    let signer_seeds:&[&[&[u8]]]=&[&[b"rewards",&self.config.key().to_bytes(),&[self.config.rewards_bump]]];

    let cpi_ctx=CpiContext::new_with_signer(self.token_program.to_account_info(), mint_accounts, signer_seeds);

    mint_to(cpi_ctx, total_rewards_points as u64)?;
    
    Ok(())
    }
}
