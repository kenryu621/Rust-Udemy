/*
Higher Order Functions (HOFs) in Rust are functions that either:
1. Take one or more functions as arguments, or
2. Return a function as their result, or
3. Both

This powerful concept enables functional programming patterns and allows for
more expressive and reusable code. Rust's support for closures makes HOFs
particularly useful for creating flexible and composable operations.

Key characteristics of HOFs in Rust:
- Can accept function pointers or closures as parameters
- Can return closures using the `impl Fn` trait bound
- Enable functional programming patterns like mapping, filtering, and folding
- Allow for lazy evaluation through returned closures
- Can capture and manipulate their environment through closures

This example demonstrates both traditional and functional approaches to
calculating the sum of all even squares below a given limit.
*/

/// Checks if a number is even
/// # Arguments
/// * `x` - The number to check (u32)
/// # Returns
/// true if the number is even, false otherwise (bool)
fn is_even(x: u32) -> bool {
    x % 2 == 0
}

/// Creates a closure that checks if a number is greater than a given limit
/// # Arguments
/// * `limit` - The threshold value (u32)
/// # Returns
/// A closure that takes a u32 and returns bool
fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |y| y > limit
}

/// Demonstrates higher order functions in Rust
pub fn higher_order_functions() {
    println!("Higher order functions:");

    // Traditional approach using loops and HOFs
    let limit = 500;
    let mut sum = 0;
    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i * i;
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("Sum = {}", sum);

    // Functional approach using Rust's iterator combinators
    let sum2 = (0..)
        .map(|x| x * x) // Square each number
        .take_while(|&x| x < limit) // Stop when we reach the limit
        .filter(|x: &u32| is_even(*x)) // Keep only even numbers
        .fold(0, |sum, x| sum + x); // Accumulate the sum

    println!("Higher order function sum = {}", sum2);
}
