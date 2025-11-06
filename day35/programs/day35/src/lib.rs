use anchor_lang::prelude::*;

declare_id!("3qMAaxRvVWDhbZFqmMkPFrs7W5mT4nT6BxxZfHmL3aKf");

#[program]
pub mod day35 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
