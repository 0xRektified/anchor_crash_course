use anchor_lang::prelude::*;

declare_id!("a72MPsKrzJEkd3K8NuYXk5KNzBeQL9suQ6fTbdQ7Dzv");

const MEANING_OF_LIFE_AND_EXISTENCE: u64 = 42;

#[program]
pub mod tryrust {
    use super::*;
    use std::collections::HashMap;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!(
            "Greetings from: {:?}, {}",
            ctx.program_id,
            MEANING_OF_LIFE_AND_EXISTENCE
        );
        Ok(())
    }

    pub fn age_checker(
        _ctx: Context<Initialize>,
        age: u32,
    ) -> Result<()> {
        if age >= 18 {
            msg!("you are over 18")
        } else {
            msg!("you are less than 18")
        }
        Ok(())
    }

    // Ternary operator doesn't exist
    pub fn age_checker_eq_ternary(
        _ctx: Context<Initialize>,
        age: u32,
    ) -> Result<()> {
        let result = if age >= 18 {
            "you are over 18"
        } else {
            "you are less than 18"
        };
        msg!(result);
        Ok(())
    }

    // Ternary operator doesn't exist
    pub fn age_checker_match(
        _ctx: Context<Initialize>,
        age: u32,
    ) -> Result<()> {
        match age {
            1 => {
                // exact match
                msg!("the age is 1");
            },
            2 | 3 => {
                msg!("the age is either 2 or 3");
            },
            4..=6 => {
                msg!("the age is between 4 and 6");
            },
            ..=18 => {
                msg!("the age is between 0 and 18 but won't be catch for 4 to 6");
            },
            _=> {
                msg!("the age is something else");
            }, 
        };
        Ok(())
    }

    pub fn fn_loop(
        _ctx: Context<Initialize>,
    ) -> Result<()> {
        let mut value = 0;
        for mut _i in (0..10).step_by(2){
            value += 1
        };
        msg!("value is {}", value);
        Ok(())
    }

    pub fn fn_array(
        _ctx: Context<Initialize>,
    ) -> Result<()> {
        // declare with fix size
        let my_array: [u32;5] = [10,20,30,40,50];

        // access element
        let first_element = my_array[0];

        let mut mutable_array: [u32; 3] = [100, 200, 300];

        mutable_array[1] = 250;

        Ok(())
    }

    pub fn fn_dyn_array(
        _ctx: Context<Initialize>,
    ) -> Result<()> {
        let mut dyn_array: Vec<u32> = Vec::new();

        dyn_array.push(10);
        dyn_array.push(20);
        dyn_array.push(30);

        let first_element = dyn_array[0];

        msg!("first element element = {}", first_element);

        let len = dyn_array.len();

        let another_var = 5;

        let len_plus_an_other_var = len as u64 + another_var;
        msg!("len + var is {}", len_plus_an_other_var);
        Ok(())
    }

    pub fn hash_map(
        _ctx: Context<Initialize>,
        key: String,
        value: String,
    ) -> Result<()> {
        let mut my_map = HashMap::new();

        my_map.insert(key.to_string(), value.to_string());
        msg!("my name is {}", my_map[&key]);

        Ok(())
    }

    pub fn fn_struct(
        _ctx: Context<Initialize>,
        name: String,
        age: u64,
    ) -> Result<()> {
        struct Person {
            my_name: String,
            my_age: u64,
        }

        let mut person1: Person = Person {
            my_name: name,
            my_age: age,
        };

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        person1.my_name = "bob".to_string();
        person1.my_age = 18;
        msg!("{} is {} years old", person1.my_name, person1.my_age);

        Ok(())
    }

    pub fn exercise(
        _ctx: Context<Initialize>,
        v: Vec<u64>
    ) -> Result<()>{
        let mut nv: Vec<u64> = Vec::new();
        for &num in v.iter() {
            if num % 2 == 0 {
                nv.push(num);
            }
        };
        msg!("{:#?}", nv);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
