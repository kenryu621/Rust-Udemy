/*
Traits in Rust define shared behavior that types can implement, similar to interfaces
in other languages. They enable ad-hoc polymorphism by specifying method signatures, often
accompanied by default implementations to offer baseline behavior. This approach fosters
code reuse, facilitates abstraction, and ensures a clear separation between behavior and data.

Beyond the basics, traits in Rust also provide:
- The ability to implement multiple traits for a single type, enabling a form of multiple inheritance.
- Associated types, which allow you to specify placeholder types that implementors must define,
  enhancing type relationships and reducing verbose generic annotations.
- Generic constraints (trait bounds), ensuring compile-time type safety and enabling powerful
  compile-time polymorphism.
- Dynamic dispatch via trait objects (using &dyn Trait or Box<dyn Trait>), which allows you to handle
  different types uniformly at runtime.
- Extension methods that add functionality to types not originally defined in your control.

These features make traits a cornerstone of Rust's type system, empowering developers to build modular,
flexible, and maintainable code.
*/

trait Animal {
    fn name(&self) -> &'static str;
    // Provides a default implementation for talking.
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
    // Factory method for creating an instance of the implementing type.
    fn create(name: &'static str) -> Self;
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name);
    }

    fn create(name: &'static str) -> Human {
        Human { name: name }
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name);
    }

    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }
}

// Demonstrates how to extend functionality to types we don't own.
trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for num in self {
            result += *num;
        }
        return result;
    }
}

pub fn traits() {
    println!("Traits:");
    let h = Human { name: "Kenry" };
    h.talk();

    let c = Cat { name: "Meeku" };
    c.talk();

    // Using the creation method to instantiate a Human.
    let h_2 = Human::create("Kenny");
    h_2.talk();

    // Explicitly specifying the type allows use of Animal's creation function.
    let h_3: Human = Animal::create("Kenji");
    h_3.talk();

    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum());
}
