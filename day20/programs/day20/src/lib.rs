use anchor_lang::prelude::*;
use anchor_lang::solana_program::rent as rent_module;


declare_id!("6kS5V6Pfvi8PL646ZPZ7m4rHDb3XW9HeV7ed9wccLMRz");

#[program]
pub mod day20 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let cost_of_empty_acc = rent_module::ACCOUNT_STORAGE_OVERHEAD as f64 * 
        rent_module::DEFAULT_LAMPORTS_PER_BYTE_YEAR as f64 *
        rent_module::DEFAULT_EXEMPTION_THRESHOLD;
        msg!("cost to create an empty account: {}", cost_of_empty_acc);

        let cost_for_32_bytes = cost_of_empty_acc + 
        32 as f64 * 
        rent_module::DEFAULT_LAMPORTS_PER_BYTE_YEAR as f64 *
        rent_module::DEFAULT_EXEMPTION_THRESHOLD;

        msg!("cost to create a 32 byte account: {}", cost_for_32_bytes);
// 1,113,600 lamports
        Ok(())
    }

    pub fn increase_account_size(_ctx: Context<IncreaseAccountSize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct IncreaseAccountSize<'info>{
    #[account(
        mut,
        realloc = size_of::<Val>() + 1000,
        realloc::payer = signer,
        realloc::zero = false,
        seeds = [],
        bump
    )]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        init,
        payer = signer,
        space = size_of::<Val>() + 8,
        seeds = [],
        bump
    )]
    val: Account<'info, Val>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>
}

#[account]
pub struct Val {
    val: u64,
}