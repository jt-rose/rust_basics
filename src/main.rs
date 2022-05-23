#![warn(clippy::all, clippy::pedantic)]
use std::io::stdin;

fn main() {
    // hello world
    println!("Hello, Rust!");

    // capture user input
    let mut my_name = String::new();
    stdin()
        .read_line(&mut my_name)
        .expect("failed to read line");

    println!("{}", my_name);

    // string interpolation
    println!("Hello {}!", "Jeff");
    println!("Here are named variables x: {x} and y: {y}", x=99, y=0);
    // print with debugging info (for example, show format of variable such as array ()
    println!("debug: {:?}", (3,4));
    // print while defining order of arguments from 0,1,2...
    println!("First is {1} and Second is {0}", "B", "A");

    // declare variables
    // immutable by default (yay!), opt into mutability with "mut" keyword
    let x = 7;
    let y = 9;
    let mut z = 10;
    z = 200;

    // shadow variables - can overwrite without mut by referencing original
    let m = 10;
    let m = m + 1;

    // shadow variables can change type
    let abc = "ABC";
    let abc = abc.len();

    // constants - require explicit type
    const PI: f32 = 3.14;

    // types
    // signed integers - i8, i16, i32, i64, i128, isize
    // unsigned integers - u8, u16, u32, u64, u128, usize
    // isize and usize default to the processor's amount
    let x64: i64 = 123456;

    // floats - f32 and f64, determined automatically at f64 if not specified
    let a = 2.0;
    let b: f32 = 1.0;

    // single chars with single quotes
    let c = 'C';

    // str and String, similar to Java's primitives vs Wrapper classes
    let mut my_string = String::from("Hello");
    my_string.push(' ');
    my_string.push_str("world");
    my_string.insert(5, ',');
    my_string.is_empty();
    my_string.len();

    // tuples - different types but fixed length
    let tup: (i32, f64, u8, f32) = (500, 6.4, 1, 29.29);
    // type inference works
    let tup2 = (5.5, 200);

    // destructuring
    let (f, g) = tup2;

    // arrays, same type and fixed length
    let arr = [1,2,3,4,5];
    // create array with length of three and default all to 0
    let emptyArr = [0; 3];

    // variable declarations can include multiline calculations with {}
    // by leaving a ; off of the final line it will automatically return the value
    let calculated_var = {
        let num1 = 2;
        num1 * num1
    };


    // check types

    // check for equality

    // if / else
    let victory = true;
    if victory == true {
        println!("Congrats");
    } else {
        println!("Oh, too bad...");
    }

    // declare function
    fn my_func(name: &str) {
        println!("my name is {}", name);
    }
    my_func("Jeff");

    // if function returns a value, declare the return value
    fn double(num: i32) -> i32 {
        num * 2
    }

    // recursion with matching
    fn factorial(num: u64) -> u64 {
        match num {
            0 => 1,
            1 => 1,
            _ => factorial(num - 1) * num,
        }
    }

    // currying
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    let add5 = move |x| add(5, x);

    // declare Rust equivalent of a class / object

    // for loops / iterators
    let nums = [1, 2, 3];

    // ---> explicit amount of rounds 0 through 9 (stop at 10)
    for i in 0..3 {
        println!("{}", nums[i]);
    }

    // ---> use length to set up dynamic iteration
    for i in 0..nums.len() {
        println!("{}", nums[i]);
    }

    // ---> use shorthand when index number isn't needed
    for n in nums {
        println!("{}", n);
    }

    // functional streams

    // arrays / vectors
    // ---> arrays, fixed length and store same type
    let my_numbers = [1, 2, 3];

    // array CRUD

    // ...and more
}
