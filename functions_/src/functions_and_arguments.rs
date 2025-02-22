/*
Functions in Rust are reusable blocks of code that perform specific tasks. They are defined using
the `fn` keyword and can take parameters and return values. Key concepts:

1. Parameters: Functions can accept input values through parameters with explicit types
2. Return values: Functions can return values using the `->` syntax
3. Ownership: By default, Rust uses move semantics for function arguments
4. References: Use `&` to pass references instead of transferring ownership
5. Mutability: Use `&mut` for mutable references to modify values outside the function
6. Expression-based: The last expression in a function is automatically returned (no semicolon)
*/

/// Prints a given integer value to the console
/// # Arguments
/// * `x` - The integer value to print (i32)
fn print_value(x: i32) {
    println!("Value = {}", x);
}

/// Increments an integer value by 1 using a mutable reference
/// # Arguments
/// * `x` - A mutable reference to the integer to increment (&mut i32)
fn increment(x: &mut i32) {
    *x += 1;
}

/// Calculates the product of two integers
/// # Arguments
/// * `x` - First integer (i32)
/// * `y` - Second integer (i32)
/// # Returns
/// The product of x and y (i32)
fn product(x: i32, y: i32) -> i32 {
    x * y // Expression-based return (no semicolon)
}

/// Demonstrates various function concepts in Rust
pub fn functions() {
    println!("Functions:");
    // Calling a function with a value argument
    print_value(33); // The value 33 is passed by value (copied)

    // Demonstrating mutable references
    let mut z = 1;
    increment(&mut z); // Pass a mutable reference to modify z
    println!("Increased value = {}", z);

    // Using a function with a return value
    let a = 3;
    let b = 5;
    let c = product(a, b); // a and b are passed by value
    println!("{} * {} = {}", a, b, c);
}
