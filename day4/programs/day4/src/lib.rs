use anchor_lang::prelude::*;

declare_id!("BhLpsimykd2FfAGYSM3CyA48QxoLf5U2KmMjjvQcXSZ5");

#[program]
pub mod day4 {
    use super::*;

    pub fn limit_range(
        ctx: Context<Initialize>,
        a: u64,
    ) -> Result<()> {
        if a < 10 {
            return err!(Day4Error::AisTooSmall);
        }
        if a > 100 {
            return err!(Day4Error::AisTooBig);
        }
        msg!("Result {}", a);
        Ok(())
    }

    pub fn limit_range_require(
        ctx: Context<Initialize>,
        a: u64,
    ) -> Result<()> {
        require!(a > 10, Day4Error::AisTooSmall);
        require!(a < 100, Day4Error::AisTooBig);
        msg!("Result {}", a);
        Ok(())
    }

    pub fn func(ctx: Context<Initialize>) -> Result<()> {
        msg!("Will this print?");
        return err!(Day4Error::AlwaysErrors);
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum Day4Error {
    #[msg("a is too big")]
    AisTooBig,
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("Always errors")]
    AlwaysErrors,
}