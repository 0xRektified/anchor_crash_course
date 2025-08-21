use anchor_lang::prelude::*;

declare_id!("4WRKNNzDcmoSVXb5gujkAqmWMJJ7wDjY1pxHngJWGCnW");

#[program]
pub mod deploy_tutorial {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("test Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
