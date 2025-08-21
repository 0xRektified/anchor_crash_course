use anchor_lang::prelude::*;

declare_id!("FU9QpiKWrkDRTr4LhbZh8J9iHtthQsJiZQMGXdy1euSA");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(
        _ctx: Context<Initialize>,
        a: u64,
        b: u64,
        message: String
    ) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You sent: {} and {}",a , b);
        Ok(())
    }

    pub fn array(
        _ctx: Context<Initialize>,
        arr: Vec<u64>,
    ) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn test_overflow(
        _ctx: Context<Initialize>,
        a: u64,
        b: u64,
    )-> Result<()>{
        //let xSafe: u64 = a - b;
        let xSafe: u64 = a.checked_sub(b).unwrap();
        msg!("result {}", xSafe);
        Ok(())
    }

    pub fn math_op(
        _ctx: Context<Initialize>,
        a: u64,
        b: u32,
    ) -> Result<()>{
        let pow = a.pow(b);
        msg!("pow of {}**{} is {}",a ,b ,pow);

        let cbrt = (b as f32).cbrt();
        msg!("cube root of {} is {}", b , cbrt);
        Ok(())
    }

    pub fn calc(
        _ctx: Context<Initialize>,
        a: u64,
        b: u64,
        op: String,
    ) -> Result<()> {
        let result = match op.as_str() {
            "+" => a.checked_add(b).ok_or(CalculatorError::ArithmeticOverflow)?,
            "-" => a.checked_sub(b).ok_or(CalculatorError::ArithmeticOverflow)?,
            "*" => a.checked_mul(b).ok_or(CalculatorError::ArithmeticOverflow)?,
            "/" => {
                if b == 0 {
                    return Err(CalculatorError::DivisionByZero.into());
                }
                a.checked_div(b).ok_or(CalculatorError::ArithmeticOverflow)?
            }
            "sqrt" => a.isqrt(),
            "log10" => {
                if a == 0 {
                    return Err(CalculatorError::LogarithmOfZero.into());
                }
                a.ilog10().into()
            }
            _ => return Err(CalculatorError::InvalidOperation.into()),
        };
        
        msg!("Calculation: {} {} {} is {}", a, op, {
            if op != "sqrt" && op != "log10" {
                b.to_string()
            } else {
                "".to_string()
            }
        }, result);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum CalculatorError {
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Cannot calculate logarithm of zero")]
    LogarithmOfZero,
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
    #[msg("Invalid operation")]
    InvalidOperation,
}
