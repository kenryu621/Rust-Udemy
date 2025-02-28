/*
    Rust’s ownership and borrowing system, while powerful, can sometimes make it
    challenging to share data across different parts of your code. A common solution
    to this challenge is using Reference Counting.

    Reference Counting (using Rc in Rust) allows multiple parts of a program to hold
    a shared reference to the same data. Once the last reference is dropped, the data
    is automatically deallocated.
*/
use std::rc::Rc;

/// The Person struct demonstrates how to store a shared, reference counted value.
/// By wrapping the name in an Rc<String>, multiple owners can safely share and manage
/// the same data without violating Rust's ownership rules. The underlying string
/// is deallocated when the last reference is dropped.
struct Person {
    // The name field is wrapped in an Rc to enable shared ownership.
    name: Rc<String>,
}

impl Person {
    /// Creates a new Person instance with a reference counted name.
    fn new(name: Rc<String>) -> Person {
        Person { name }
    }

    /// Greets by printing the person's name.
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

/// Demonstrates reference counting with Rc.
/// Cloning an Rc increases its reference count, enabling shared ownership.
pub fn ref_counted_var_demo() {
    println!("Reference Counted Variables:");

    let name = Rc::new("Alex".to_string());

    // At this point, 'name' has a single strong reference.
    println!(
        "Name = {}, name has {} strong pointers",
        name,
        Rc::strong_count(&name)
    );

    {
        // Cloning the Rc increments the reference count.
        let person = Person::new(name.clone());
        println!(
            "Name = {}, name has {} strong pointers",
            name,
            Rc::strong_count(&name)
        );
        person.greet();
    }

    // Leaving the block drops the cloned reference, reducing the count.
    println!(
        "Name = {}, name has {} strong pointers",
        name,
        Rc::strong_count(&name)
    );
    println!("Name = {}", name); // This works since Rc allows shared ownership.
}
