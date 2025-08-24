use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("4r2jtfrZjWkHdohhQhxi3jZMoKK1XfraZwHT8njVMKRZ");

#[program]
pub mod example_map {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, key: u64) -> Result<()> {
        Ok(())
    }

    pub fn set(_ctx: Context<Set>, value: u64) -> Result<()>{
        _ctx.accounts.val.value = value;
        Ok(())
    }

    pub fn initialize_dex(_ctx: Context<Dexs>, key: u64, key2: u64) -> Result<()> {
        Ok(())
    }

    pub fn set_dex(_ctx: Context<SetDex>, value: u64) -> Result<()>{
        _ctx.accounts.dex.liquidity = value;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct Set<'info>{
    #[account(mut)]
    val: Account<'info, Val>
}

#[derive(Accounts)]
#[instruction()]
pub struct SetDex<'info>{
    #[account(mut)]
    dex: Account<'info, Dex>
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Val>() + 8,
        seeds=[
            &key.to_le_bytes().as_ref()
        ],
        bump
    )]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(which_map: u64, key: u64)]
pub struct Dexs<'info> {
    #[account(
        init,
        payer = signer,
        space = size_of::<Dex>() + 8,
        seeds=[
            &which_map.to_le_bytes().as_ref(),
            &key.to_le_bytes().as_ref(),
        ],
        bump
    )]
    dex: Account<'info, Dex>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>
}

#[account]
pub struct Val {
    value: u64
}

#[account]
pub struct Dex {
    liquidity: u64
}