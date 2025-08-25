use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("CxuKqyQnSTJA6xt2kbiq3P9AWRsimpYZx2oi83xTEcZ7");

#[program]
pub mod day30 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        _ctx.accounts.pda.signer = _ctx.accounts.signer.key();
        Ok(())
    }

    pub fn delete(_ctx: Context<Delete>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(
        init,
        payer = signer,
        space = size_of::<MyPda>() + 8,
        seeds = [],
        bump
    )]
    pub pda: Account<'info, MyPda>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Delete <'info>{
    #[account(
        mut,
        close = signer,
        has_one = signer @ Error::NotAuthorized,
    )]
    pub pda: Account<'info, MyPda>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum Error{
    #[msg("Not authorized")]
    NotAuthorized
}

#[account]
pub struct MyPda {
    signer: Pubkey,
}