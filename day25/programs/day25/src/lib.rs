use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("1a7To5apg9ySFrHH42MhLkETAqECgoKkE3iAizM2tbQ");

#[program]
pub mod day25 {
    use super::*;

    pub fn initialize_pda(ctx: Context<InitializePDA>) -> Result<()> {
        Ok(())
    }

    pub fn initialize_key_pair_account(ctx: Context<InitializeKeypairAccount>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePDA<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<MyPDA>() + 8,
        seeds = [],
        bump,
    )]
    pub store_pda: Account<'info, MyPDA>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeKeypairAccount<'info>{
    #[account(
        init,
        payer = signer,
        space = size_of::<MyPDA>() + 8
    )]
    pub store_key: Account<'info, MyPDA>,

    #[account(mut)]
    pub signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[account]
pub struct MyPDA {
    x: u64,
}
