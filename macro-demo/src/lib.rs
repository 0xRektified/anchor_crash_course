extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]

pub fn foo_bar_attribute(_metadata: TokenStream, _input: TokenStream)
    -> TokenStream {
    // Parse the input TokenStream into an AST node representing a struct
    let input = parse_macro_input!(_input as ItemStruct);
    let struct_name = &input.ident; // get the name of the struct

    TokenStream::from(quote!{
        #[derive(Debug)]

        struct #struct_name {
            foo: i32,
            bar: i32,
        }

        impl Default for #struct_name{
            fn default() -> Self {
                #struct_name {foo: 10, bar: 20}
            }
        }

        impl #struct_name {
            fn double_foo(&self)->i32 {
                self.foo * 2
            }
        }
    })


}