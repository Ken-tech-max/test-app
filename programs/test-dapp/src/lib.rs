use anchor_lang::{prelude::*, 
    // solana_program::system_program, 
    AnchorDeserialize, 
    AnchorSerialize};
use std::default::Default;
declare_id!("BEgqTzvaqkGTXCx5hM7ownfjVAGTNWhnYEdg5zYXmcF3");

#[program]
pub mod test_dapp {
    use super::*;
    pub fn initialize_user(ctx: Context<InitializeUser>, 
        strenth: u64, 
        magic: u64,
        vitality: u64,
        spirit: u64,
        luck: u64,
        life_origins: u8,
    ) -> ProgramResult {
        let attribute = Attribute{
            strenth:strenth,
            magic:magic, 
            vitality:vitality, 
            spirit:spirit, 
            luck:luck
        };
        if strenth > 20 || magic > 20 || vitality >20 || spirit > 20 || luck > 20 {
            return Err(ErrorCode::IncorrectParam.into());
        }
        if strenth < 2 || magic < 2 || vitality < 2 || spirit < 2 || luck < 2 {
            return Err(ErrorCode::IncorrectParam.into());
        }
        
        ctx.accounts.user_account.attribute = attribute;
        // ctx.accounts.user_account.life_origins = life_origins;
        match life_origins {
            0 => ctx.accounts.user_account.life_origins = LifeOrigins::Heaven,
            1 => ctx.accounts.user_account.life_origins = LifeOrigins::Hell,
            2 => ctx.accounts.user_account.life_origins = LifeOrigins::Earth,
            _ => return Err(ErrorCode::IncorrectParam.into())
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user_account: Account<'info, Charactor>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[account]
#[derive(Default)]
pub struct Charactor {
    pub attribute: Attribute,
    pub life_origins: LifeOrigins
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct Attribute {
    pub strenth: u64,
    pub magic: u64,
    pub vitality: u64,
    pub spirit: u64,
    pub luck: u64,
}

#[derive(Clone, Copy)]
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub enum LifeOrigins {
    // #[default]
    Heaven,
    Hell,
    Earth
}

impl Default for LifeOrigins {
    fn default() -> Self {LifeOrigins::Heaven}
}

#[error]
pub enum ErrorCode {
    #[msg("Values are not validated")]
    IncorrectParam
}