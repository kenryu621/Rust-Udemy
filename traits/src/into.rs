#![allow(dead_code)]

//! This module demonstrates the use of the Into trait in Rust.
//!
//! The Into trait is a powerful feature that allows for automatic type conversion.
//! It lets you write functions that can accept multiple types (e.g., &str, String)
//! and convert them into a desired type seamlessly. In this example, the Person
//! struct leverages the Into trait to provide flexible constructors.
//!
//! Key functions:
//! - new: Creates a Person from a &str.
//! - new_into: Uses the Into trait for automatic conversion of types that can be turned into a String.
//! - new_where: Shows an alternative syntax using a where clause to apply the Into trait bound.

use std::fmt::Debug;

#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    /// Creates a new Person from a string slice.
    fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Creates a new Person using the Into trait for type conversion.
    ///
    /// The generic parameter S must implement Into<String>, which allows this function
    /// to accept types like &str or String. This flexibility enables automatic conversion
    /// to a String when initializing a Person.
    fn new_into<S: Into<String>>(name: S) -> Person {
        Person { name: name.into() }
    }

    /// Creates a new Person using a where clause for the Into trait bound.
    ///
    /// This function demonstrates an alternative syntax using a where clause to specify
    /// that S must implement Into<String>, achieving the same flexibility as new_into.
    fn new_where<S>(name: S) -> Person
    where
        S: Into<String>,
    {
        Person { name: name.into() }
    }
}

/// Demonstrates the usage of flexible constructors that leverage the Into trait.
pub fn into() {
    println!("Into:");

    let alex = Person::new("Alex");
    println!("{:?}", alex);

    let name: String = "James".to_string();
    let james = Person::new(name.as_ref());
    println!("{:?}", james);
    // Instead of converting a String to &str manually, we can use the Into trait
    // to allow automatic conversion.

    let steph = Person::new_into("Steph");
    println!("{:?}", steph);

    let harden = Person::new_where("Harden");
    println!("{:?}", harden);
}
