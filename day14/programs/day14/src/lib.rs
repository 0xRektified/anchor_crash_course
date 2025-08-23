use anchor_lang::prelude::*;

declare_id!("EQbVsDeTkiZzj8YZr8jnppFZrcdQqJ9qfVLrypUj5ipK");

// NOTE: Replace with your wallet's public key
const OWNER: &str = "azAq71cZVjpiQvd7c4qAicrq1zQNM8Eb84xR8Y81w5o";

#[program]
pub mod day14 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let the_signer1: &mut Signer = &mut ctx.accounts.signer1;
        msg!("The signer1: {:?}", *the_signer1);
        let the_signer2: &mut Signer = &mut ctx.accounts.signer2;
        msg!("The signer2: {:?}", *the_signer2);
        Ok(())
    }

    pub fn mutlisigner(ctx: Context<Initialize>) -> Result<()> {
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