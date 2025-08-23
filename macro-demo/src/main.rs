use macro_demo::*;

// An attribute-like macro takes in a struct and can completely rewrite it.

// A derive macro augments a struct with additional functions.

#[foo_bar_attribute]
struct MyStruct {
    baz: i32,
}

// Derive maccro
#[derive(Debug)]
struct Foo{
    bar: i32,
}

pub fn test_derive(){
    let foo = Foo {bar: 3};
    println!("{:?}", foo);
}

fn main() {
    let demo = MyStruct::default();

    println!("struct is {:?}", demo);

    let double_foo = demo.double_foo();

    println!("double foo: {}", double_foo);

    test_derive();
}