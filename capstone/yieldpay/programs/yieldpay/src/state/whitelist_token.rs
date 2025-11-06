use anchor_lang::prelude::*;
use crate::{errors::YieldpayError};

pub const WHITELIST_SEED: &str = "WHITELIST";

#[account]
#[derive(InitSpace)]
pub struct WhitelistToken {
    pub tokens:[Pubkey;2],
    pub supported_token_num:u8,
    pub bump:u8
}
impl WhitelistToken{

    pub fn whitelist_mint(&mut self,mint:&Pubkey)->Result<()>{

        if self.supported_token_num as usize >= self.tokens.len() {
            return err!(YieldpayError::TokenListFull);
        }

        if self.tokens.contains(&mint) {
           return err!(YieldpayError::TokenAlreadyWhitelisted);
        }

        self.tokens[self.supported_token_num as usize]=mint.key();
        self.supported_token_num +=1;

        Ok(())
    }
}