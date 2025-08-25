use anchor_lang::prelude::*;
use anchor_lang::system_program;
use std::mem::size_of;
use std::str::FromStr;

declare_id!("SSKVmACiHxQ1h2Sj5FYV1JoRD1vBhqXPxNW2pjaVs71");

#[program]
pub mod day26 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let _initialize_pda = &mut _ctx.accounts.pda;
        msg!("Greetings from: {:?}", _ctx.program_id);
        Ok(())
    }

    pub fn donate(_ctx: Context<Donate>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            _ctx.accounts.system_program.to_account_info(),
            system_program::Transfer{
                from: _ctx.accounts.signer.to_account_info(),
                to: _ctx.accounts.pda.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;
        Ok(())
    }

    pub fn withdraw(_ctx: Context<Withdraw>, amount: u64) -> Result<()> {
       let _ = _ctx.accounts.pda.sub_lamports(amount);
       let _ = _ctx.accounts.signer.add_lamports(amount + 1);
        Ok(())
    }
}

#[account]
pub struct MyPda {}

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
pub struct Donate <'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pda: Account<'info, MyPda>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw <'info>{
    #[account(mut, address = Pubkey::from_str("azAq71cZVjpiQvd7c4qAicrq1zQNM8Eb84xR8Y81w5o").unwrap())]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub pda: Account<'info, MyPda>,
}