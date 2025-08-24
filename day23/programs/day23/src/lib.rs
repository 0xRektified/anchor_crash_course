use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("8uikMNgFp5B4k3j6aEfgGLFMETosXxEHGztA5k1DQ743");

#[program]
pub mod day23 {
    use super::*;

    pub fn send_sol<'a, 'b, 'c, 'info>(
        _ctx: Context<'a, 'b, 'c,'info, SendSol<'info>>,
        amount: u64
    ) -> Result<()> {
        let amount_each_gets = amount / _ctx.remaining_accounts.len() as u64;
        let system_program = &_ctx.accounts.system_program;

        for recipient in _ctx.remaining_accounts {
            let cpi_accounts = system_program::Transfer {
                from: _ctx.accounts.signer.to_account_info(),
                to: recipient.to_account_info(),
            };

            let cpi_program = system_program.to_account_info();
            let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
            let res = system_program::transfer(cpi_context, amount_each_gets);
            if !res.is_ok(){
                return err!(Errors::TransferFailed)
            }
        };
        Ok(())
    }
}

#[error_code]
pub enum Errors {
    #[msg("transfer fail")]
    TransferFailed,
}

#[derive(Accounts)]
pub struct SendSol<'info> {

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
}
