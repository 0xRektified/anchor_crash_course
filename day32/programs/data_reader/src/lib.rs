use anchor_lang::prelude::*;

declare_id!("AbMY9r5CusEkEaE3ffDJo2mrUMmudXZthBKrRtv71GYX");

#[program]
pub mod data_reader {
    use super::*;

    pub fn read_other_data(_ctx: Context<ReadOtherData>) -> Result<()> {
        let data_account = &_ctx.accounts.other_data;
        if data_account.data_is_empty(){
            return err!(MyError::NoData);
        }
        let mut data_slice: &[u8] = &data_account.data.borrow();
        let data_struct: Storage = AccountDeserialize::try_deserialize(
            &mut data_slice
        )?;
        msg!("The value of x is {}", data_struct.can_be_anything);
        Ok(())
    }
}

#[error_code]
pub enum MyError {
    #[msg("no data")]
    NoData,
}

#[derive(Accounts)]
pub struct ReadOtherData<'info> {
    /// CHECK: We're manually deserializing this account
    other_data: UncheckedAccount<'info>,
}

#[account]
pub struct Storage { // Here only the structure name matter for the discriminator
    can_be_anything: u32
}
