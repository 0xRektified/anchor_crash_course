use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("2S1CxRcgZbwGggAkbz24awqmxof8u1842nzPbsJ4DM55");

const STARTING_POINTS: u32 = 10;

#[program]
pub mod day24 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn setup_game(_ctx: Context<PointInit>) -> Result<()> {
        _ctx.accounts.player.points = STARTING_POINTS;
        _ctx.accounts.player.authority = _ctx.accounts.signer.key();
        Ok(())
    }

    pub fn transfer_points(
        _ctx: Context<TransferPoints>,
        amount: u32,
    ) -> Result<()> {
        _ctx.accounts.from.points -= amount;
        _ctx.accounts.to.points += amount;
        Ok(())
    }

    pub fn update_value(
        _ctx: Context<UpdateValue>,
        new_value: u64,
    ) -> Result<()> {
        _ctx.accounts.store.x = new_value;
        Ok(())
    }
}



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = fren,
        space = size_of::<MyStorage>() + 8,
        seeds = [],
        bump,
    )]
    pub store: Account<'info, MyStorage>,

    #[account(mut)]
    pub fren: Signer<'info>,

    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)]
pub struct UpdateValue<'info>{
    #[account(
        mut, seeds =[], bump
    )]
    pub store: Account<'info, MyStorage>,

    #[account(mut)]
    pub fren: Signer<'info>,
}

#[account]
pub struct MyStorage {
    x: u64,
}

#[derive(Accounts)]
pub struct PointInit <'info>{
    #[account(
        init,
        payer = signer,
        space = size_of::<Player>() + 8,
        seeds = [&(signer.as_ref().key().to_bytes())],
        bump
    )]
    player: Account<'info, Player>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}



#[error_code]
pub enum Errors{
    #[msg("SignerIsNotAuthority")]
    SignerIsNotAuthority,
    #[msg("InsufficientsPoints")]
    InsufficientsPoints
}

#[derive(Accounts)]
#[instruction(amount: u32)]
pub struct TransferPoints<'info> {
    #[account(
        mut,
        has_one = authority @ Errors::SignerIsNotAuthority,
        constraint = from.points >= amount @ Errors::InsufficientsPoints
    )]
    from: Account<'info, Player>,

    #[account(mut)]
    to: Account<'info, Player>,

    #[account(mut)]
    authority: Signer<'info>,
}

#[account]
pub struct Player {
    points: u32,
    authority: Pubkey,
}