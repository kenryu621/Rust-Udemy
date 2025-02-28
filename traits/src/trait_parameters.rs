//! This module demonstrates the use of trait parameters in Rust.
//!
//! In Rust, traits define shared behavior that can be used for polymorphism.
//! The `Shape` trait in this module provides a common interface with an `area` method,
//! which is implemented by different types such as `Circle` and `Square`.
//!
//! Functions can accept parameters constrained by traits, allowing them to work with any type
//! that meets the required interface. For example, the function `print_info` requires its parameter
//! to implement both the `Shape` and `Debug` traits. This enables the function to print debug information
//! and calculate the area of various shapes in a type-safe manner.

#![allow(dead_code)]
use std::fmt::Debug;

trait Shape {
    fn area(&self) -> f64;
}

// We can get the compiler to derive the default implementation of Debug trait
#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// We can write functions that take parameters implementing traits.
// There are various syntaxes to specify trait bounds:
// 1. Using `impl Trait` syntax, e.g., fn print_info(shape: impl Shape + Debug)
// 2. Using generics with trait bounds, e.g., fn print_info<T: Shape + Debug>(shape: T)
// 3. Using a `where` clause for additional clarity when multiple constraints are needed.
fn print_info<T>(shape: T)
where
    T: Shape + Debug,
{
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

/// Demonstrates trait parameters by creating a `Circle` and printing its details.
pub fn trait_parameters() {
    println!("Trait as parameters:");
    let c = Circle { radius: 2.0 };
    print_info(c);
}
