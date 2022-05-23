#![warn(clippy::all, clippy::pedantic)]

fn main() {
    // hello world
    println!("Hello, Rust!");

    // capture user input
    let mut my_name = String::new();

    // string interpolation
    println!("Hello {}!", "Jeff");

    // declare variables

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
    let nums = [1,2,3];

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
    let my_numbers = [1,2,3];



    // array CRUD

    // ...and more
}
