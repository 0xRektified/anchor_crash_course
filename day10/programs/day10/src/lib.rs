use anchor_lang::prelude::*;
pub mod calculate;
declare_id!("i29zmLniHRRN1eVYhqpbbf8eNbA9Uso2x13MQPUYc1e");

#[program]
pub mod day10 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let u = get_a_num();
        msg!("Greetings from: {:?}", ctx.program_id);
        some_internal_function::internal_function();
        Ok(())
    }

    pub mod some_internal_function {
        pub fn internal_function() {
            // internal function logic
        }

        // Private won't compile
        // pub(in crate::day10) fn private_function(){

        // }
    }

    pub fn add_two_number(_ctx: Context<Initialize>, x: u64, y: u64) -> Result<()>{
        let result = calculate::add(x, y);
        msg!("{} + {} = {}", x, y, result);
        Ok(())
    }
}

mod do_something {
    // Import func_visibility module
    use crate::day10;

    pub fn some_func_here() {
        //Call the internal_function from outside its parent module
        day10::some_internal_function::internal_function();
       
        // Private won't compile
       // day10::some_internal_function::private_function();
    }
}

// Could be ocnsider as external
fn get_a_num() -> u64 {
    2
}

#[derive(Accounts)]
pub struct Initialize {}
