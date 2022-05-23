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

    // types
    // str and String, similar to Java's primitives vs Wrapper classes

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
