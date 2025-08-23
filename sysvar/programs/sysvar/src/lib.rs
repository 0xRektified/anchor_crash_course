use anchor_lang::prelude::*;
// https://rareskills.io/post/solana-sysvar
declare_id!("6avhC8TG5YKTq82FGqFvxWsNtau8nKSxDSegBJAgpcu2");

#[program]
pub mod sysvar {
    use super::*;
    use chrono::*;
    use anchor_lang::solana_program::sysvar::{
        instructions, fees::Fees, recent_blockhashes::RecentBlockhashes
    };

    pub fn initialize(
        _ctx: Context<Initialize>,
        number: u32
    ) -> Result<()> {
        let clock: Clock = Clock::get()?;
        msg!("Block timestamp: {}", clock.unix_timestamp);
        msg!("clock: {:?}", clock);

        let epoch_schedule = EpochSchedule::get()?;
        msg!("epoch schedule: {:?}", epoch_schedule);

        let rent_var = Rent::get()?;
        msg!("rent var: {:?}", rent_var);

        // Sysvar from public address
        // create array to store stake history account
        let arr = [_ctx.accounts.stake_history.clone()];
        let accounts_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(accounts_iter)?;
        let stake_history = StakeHistory::from_account_info(sh_sysvar_info);
        msg!("stake_history: {:?}", stake_history);

        let arr = [_ctx.accounts.last_restart_slot.clone()];
        let accounts_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(accounts_iter)?;
        
        // LastRestartSlot contains a u64 value (slot number)
        let data = sh_sysvar_info.try_borrow_data()?;
        let last_restart_slot = u64::from_le_bytes(
            data[0..8].try_into().unwrap()
        );
        msg!("last_restart_slot: {}", last_restart_slot);

        // Sysvar from account
        let arr = [_ctx.accounts.instruction_sysvar.clone()];
        let account_info_iter = &mut arr.iter();
        let instructions_sysvar_account = next_account_info(account_info_iter)?;

        let instruction_details = instructions::load_instruction_at_checked(
            0, instructions_sysvar_account
        )?;
        msg!(
            "Instruction details of this transaction: {:?}",
            instruction_details
        );
        msg!("Number is: {}", number);
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
pub struct Initialize<'info>{
    /// CHECK:
    pub stake_history: AccountInfo<'info>,
    /// CHECK:
    pub recent_blockhashes: AccountInfo<'info>,
    /// CHECK:
    pub instruction_sysvar: AccountInfo<'info>,
    /// CHECK:
    pub last_restart_slot: AccountInfo<'info>,
}
