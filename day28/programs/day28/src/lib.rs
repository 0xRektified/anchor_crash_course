use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("79HMndPVWggXkcU9NQroFsZmWjvgKQ8krged2J765B4p");

#[program]
pub mod day28 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set(_ctx: Context<Set>, amount: u32) -> Result<()> {
        _ctx.accounts.pda.pda = amount;
        Ok(())
    }

    pub fn always_fail(_ctx: Context<Set>) -> Result<()> {
        return err!(Error::AlwaysFails);
        Ok(())
    }
}

#[error_code]
pub enum Error {
    #[msg("AlwaysFails")]
    AlwaysFails,
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
pub struct Set <'info> {
    #[account(mut)]
    pub pda: Account<'info, MyPda>,
}

#[account]
pub struct MyPda{
    pda: u32,
}
