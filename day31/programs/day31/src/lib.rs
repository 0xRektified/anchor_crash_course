use anchor_lang::prelude::*;

declare_id!("EjUbHAPzXZwUZvxRbDnZD5Dsw7GXWWGjovr7e2cFtie");

#[program]
pub mod day31 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // @audit interesting
    pub fn uncheck_read(_ctx: Context<Foo>) -> Result<()> {
        let data = &_ctx.accounts.uncheck_read.try_borrow_data()?;
        msg!("{:?}", data);
        Ok(())
    }

    pub fn hello(_ctx: Context<Hello>) -> Result <()> {
        let lamports = _ctx.accounts.signer.lamports();
        let address = &_ctx.accounts.signer.signer_key().unwrap();
        msg!("hello {:?} you have {} lamports", address, lamports);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    some_account: Account<'info, SomeAccount>,
}

#[account]
pub struct SomeAccount{}

#[derive(Accounts)]
pub struct Foo<'info> {
    /// CHECK : we are printing data
    uncheck_read: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Hello<'info>{
    pub signer: Signer<'info>,
} 
