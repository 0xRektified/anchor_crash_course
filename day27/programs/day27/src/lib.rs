use anchor_lang::prelude::*;
use std::mem::size_of;
use anchor_lang::system_program;

declare_id!("7eveRCyuZT7yxfZxKUSGPHQjfk9hQfbFzSvS5g3xsFux");

#[program]
pub mod day27 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn drain_lamports(_ctx: Context<DrainLamports>) -> Result<()> {
        let lamports = _ctx.accounts.my_pda.to_account_info().lamports();
        _ctx.accounts.my_pda.sub_lamports(lamports)?;
        _ctx.accounts.signer.add_lamports(lamports)?;
        Ok(())
    }

    pub fn give_to_system_program(_ctx: Context<GiveToSystemProgram>) -> Result<()> {
        let account_info = &mut _ctx.accounts.my_pda.to_account_info();
        account_info.assign(&system_program::ID);
        account_info.realloc(0, false)?;
        Ok(())
    }

    pub fn erase(_ctx: Context<Erase>) -> Result<()> {
        _ctx.accounts.my_pda.realloc(0, false)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Erase<'info> {
    /// CHECK: we are going to erase the account
    #[account(mut)]
    pub my_pda: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<MyPda>() +8,
        seeds = [],
        bump
    )]
    pub my_pda: Account<'info, MyPda>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DrainLamports<'info> {
    #[account(mut)]
    pub my_pda: Account<'info, MyPda>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct GiveToSystemProgram<'info> {
    #[account(mut)]
    pub my_pda: Account<'info, MyPda>,
}

#[account]
pub struct MyPda {}