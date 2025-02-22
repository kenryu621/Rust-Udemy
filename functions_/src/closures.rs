/*
Closures in Rust are anonymous functions that can capture their environment. They are similar to
lambda expressions in other languages. Key characteristics:

1. Syntax: Defined using vertical bars for parameters and curly braces for body
   - `|params| { body }`
2. Type Inference: Parameter and return types can often be inferred
3. Capturing Environment: Can capture variables from their surrounding scope
4. Ownership: Can capture variables by:
   - Value (moves ownership)
   - Reference (&)
   - Mutable reference (&mut)
5. Flexibility: Can be stored in variables, passed as arguments, or returned from functions
6. Traits: Implement Fn, FnMut, or FnOnce traits depending on how they capture variables
*/

/// A simple function that prints "hello!"
fn say_hello() {
    println!("hello!");
}

/// Demonstrates various aspects of closures in Rust
pub fn closures() {
    println!("Closures:");

    // Storing a function pointer in a variable
    let sh = say_hello;
    sh(); // Invoke the function through the variable

    // Basic closure with explicit types
    let plus_one = |x: i32| -> i32 { x + 1 };
    let a = 7;
    println!("{} + 1 = {}", a, plus_one(a));

    // Closure with type inference
    let plus_two = |x| {
        let mut z = x;
        z += 2;
        z // Implicit return
    };
    println!("{} + 2 = {}", 7, plus_two(7));

    // Closure capturing environment variable
    let mut three = 3;
    {
        // Captures 'three' by immutable reference
        let plus_three = |x| {
            let mut z = x;
            z += three;
            z
        };
        println!("{} + 3 = {}", 3, plus_three(3));
    } // plus_three goes out of scope here

    // Now we can mutate 'three' since the closure that borrowed it is gone
    let _borrow_three = &mut three;

    // Closure with mutable reference parameter
    let plus_four = |x: &mut i32| *x += 4;
    let mut f = 12;
    plus_four(&mut f);
    println!("f = {}", f);
}
