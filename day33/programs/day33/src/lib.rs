use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("8cS25W65T8isi3WoGTX6SNdN5wZjbd8EzvjJPrTQKiFK");

#[program]
pub mod day33 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Data Account initialized: {}", _ctx.accounts.day33_data_account.key());
        Ok(())
    }

    pub fn add_to_store(
        _ctx: Context<AddStore>,
        x: u64,
        y: u64,
    ) -> Result<()> {
        let res = x + y;
        _ctx.accounts.day33_data_account.result = res; 
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
    pub day33_data_account: Account<'info, Store>,

    #[account(mut)]
    pub signer: Signer<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddStore<'info> {
    #[account(mut)]
    pub day33_data_account: Account<'info, Store>,
}

#[account]
pub struct Store{
    pub result: u64,
}