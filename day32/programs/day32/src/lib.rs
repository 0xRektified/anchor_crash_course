use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("CJwKvFawAjeCXzhEsQMSZwPRWKbZsNBenkKTB79VaJep");

#[program]
pub mod day32 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.storage.x = 10;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Storage>() + 8,
        seeds = [],
        bump,
    )]
    pub storage: Account<'info, Storage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Storage {
    x: u64
}