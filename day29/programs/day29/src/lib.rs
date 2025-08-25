use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("6s26X8cev112oR96FZcWkkvtNEYBPPV9A5QeanShuPhR");

#[program]
pub mod day29 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(
        init,
        payer = signer,
        space = size_of::<Store>() + 8,
        seeds = [],
        bump
    )]
    pub store: Account<'info, Store>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Store {}
