// let cpi_ctx = CpiContext::new(
//     _ctx.accounts.day33_program.to_account_info(),
//     AddStore {
//         day33_data_account: _ctx.accounts.day33_data_account.to_account_info(),
//     }
// );

use anchor_lang::prelude::*;
use day33::program::Day33;
use day33::Store;
use day33::cpi::accounts::AddStore;

declare_id!("9xgoWvbQV1voGBGeqTttuUpuvwyJECeqzRFyxqY5NYpE");

#[program]
pub mod alice {
    use super::*;

    pub fn ask_day33_to_add(
        _ctx: Context<Day33Op>,
        a: u64,
        b: u64,
    ) -> Result<()> {
        let res = day33::cpi::add_to_store(_ctx.accounts.add_function_ctx(), a, b);

        if !res.is_ok() {
            return err!(Errors::CPIToDay33Failed);
        }
        Ok(())
    }
}

impl<'info> Day33Op<'info> {
    pub fn add_function_ctx(&self) ->
        CpiContext<'_,'_,'_,'info, AddStore<'info>>
        {
        let cpi_program = self.day33_program.to_account_info();

        let cpi_account = AddStore {
            day33_data_account: self.day33_data_account.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_account)
    }
}

#[derive(Accounts)]
pub struct Day33Op<'info> {
    #[account(mut)]
    pub day33_data_account: Account<'info, Store>,

    pub day33_program: Program<'info, Day33>,
}

#[error_code]
pub enum Errors{
    #[msg("cpi to day33 failed")]
    CPIToDay33Failed
}