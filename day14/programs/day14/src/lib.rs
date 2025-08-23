use anchor_lang::prelude::*;

declare_id!("EQbVsDeTkiZzj8YZr8jnppFZrcdQqJ9qfVLrypUj5ipK");

// NOTE: Replace with your wallet's public key
const OWNER: &str = "azAq71cZVjpiQvd7c4qAicrq1zQNM8Eb84xR8Y81w5o";

#[program]
pub mod day14 {
    use super::*;

    pub fn initialize(
        _ctx: Context<Initialize>
    ) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        msg!("The signer1: {:?}", *the_signer1);
        Ok(())
    }

    pub fn mutlisigner(
        _ctx: Context<Initialize>
    ) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        msg!("The signer1: {:?}", *the_signer1);
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;
        msg!("The signer2: {:?}", *the_signer2);
        let the_signer3: &mut Signer = &mut ctx.accounts.signer3;
        msg!("The signer3: {:?}", *the_signer3);
        Ok(())
    }

    #[access_control(check(&ctx))]
    pub fn access(ctx: Context<OnlyOwner>) -> Result<()>{
        msg!("I am the owner");
        Ok(())
    }

    pub fn compute_unit(
        _ctx: Context<Initialize>
    ) -> Result<()>{
        // this costs 600 CU (type defaults to Vec<i32>)
        let mut a = Vec::new();
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);

        // this costs 618 CU
        let mut a: Vec<u64> = Vec::new();
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);

        // this costs 600 CU (same as the first one but the type was explicitly denoted)
        let mut a: Vec<i32> = Vec::new();
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);

        // this costs 618 CU (takes the same space as u64)
        let mut a: Vec<i64> = Vec::new();
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);

        // this costs 459 CU
        let mut a: Vec<u8> = Vec::new();
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);
        a.push(1);

        Ok(())
    }
}

fn check(ctx: &Context<OnlyOwner>) -> Result<()>{
    require_keys_eq!(
        ctx.accounts.signer_account.key(),
        OWNER.parse::<Pubkey>().unwrap(),
        OnlyOwnerError::NotOwner
    );
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer1: Signer<'info>,
    #[account(mut)]
    pub signer2: Signer<'info>,
    #[account(mut)]
    pub signer3: Signer<'info>,
}

#[derive(Accounts)]
pub struct OnlyOwner<'info> {
    signer_account: Signer<'info>,
}

#[error_code]
pub enum OnlyOwnerError {
    #[msg("Only owner can call this function!")]
    NotOwner,
}