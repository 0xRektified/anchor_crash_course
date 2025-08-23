use anchor_lang::prelude::*;

declare_id!("6avhC8TG5YKTq82FGqFvxWsNtau8nKSxDSegBJAgpcu2");

#[program]
pub mod sysvar {
    use super::*;
    use chrono::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        let clock: Clock = Clock::get()?;
        msg!("Block timestamp: {}", clock.unix_timestamp);
        Ok(())
    }

    pub fn get_day_of_week(
        _ctx: Context<Initialize>
    ) -> Result<()>{
        let clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp;

        let date_time = chrono::NaiveDateTime::from_timestamp_opt(
            time_stamp,
            0
        ).unwrap();
        let day_of_the_week = date_time.weekday();

        msg!("Week day is: {}", day_of_the_week);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
