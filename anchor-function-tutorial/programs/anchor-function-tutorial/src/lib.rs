use anchor_lang::prelude::*;

declare_id!("7NwApojpzMwb8m25nmHQpRahWZsd9C4EUbfNFZnTLvcE");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<NonEmptyAccountExample>, a:u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn add(ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        msg!("Sum is {}", sum);  
          Ok(())
    }
    
    pub fn sub(ctx: Context<Empty>, a: u64, b: u64) -> Result<()> {
    let difference = a - b;
    msg!("Difference is {}", difference);  
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Empty {}


#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}
