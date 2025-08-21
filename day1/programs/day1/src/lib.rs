use anchor_lang::prelude::*;

declare_id!("8mCuZuJV4sUNejhUXPa9sNh25DBHhm7QrvMP1DXBuWaF");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
