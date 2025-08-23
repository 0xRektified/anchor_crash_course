use anchor_lang::prelude::*;

declare_id!("E2CwkZmjHPHXathgKu2FQjFVt3B1kU9JQmqUMGz7uSPR");

#[program]
pub mod basic_storage {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn set(_ctx: Context<Set>, new_x: i64) -> Result<()> {
        let my_storage = &mut _ctx.accounts.my_storage;
        my_storage.x = new_x;
        Ok(())
    }
    pub fn print_x(_ctx: Context<PrintX>) -> Result<()> {
        let x = _ctx.accounts.my_storage.x;
        msg!("the new value is {}", x);
        Ok(())
    }

    pub fn add_one(_ctx: Context<Set>, new_x: i64) -> Result<()> {
        let my_storage = &mut _ctx.accounts.my_storage;
        my_storage.x = new_x + 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PrintX<'info> {
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space=size_of::<MyStorage>() + 8,
        seeds = [],
        bump
    )]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: i64,
    y: i64,
}
