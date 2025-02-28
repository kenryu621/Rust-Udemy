/*
    This example demonstrates two techniques for storing heterogeneous objects (i.e., objects of different types)
    in a single vector in Rust.

    1. Using an enum: We wrap different types (e.g., Human and Cat) in an enum (Creature). This approach
       requires pattern matching to extract and work with the inner values.

    2. Using dynamic dispatch with trait objects: By storing Box<dyn Animal> in a vector, any type implementing
       the Animal trait can be added directly. This method simplifies code by allowing direct method calls
       without needing to unwrap an enum.
*/

trait Animal {
    fn name(&self) -> &'static str;

    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("Hello, my name is {}", self.name());
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
        println!("{} says meow", self.name());
    }
}

/// Enum wrapper for different Animal implementations.
/// This allows us to store both Human and Cat in a single vector using explicit variants.
enum Creature {
    Human(Human),
    Cat(Cat),
}

pub fn vectors_of_different_objects_demo() {
    println!("Vectors storing heterogeneous objects:");

    // Direct vector creation with a concrete type is inflexible:
    let mut first_vector = Vec::new();
    first_vector.push(Human { name: "Kenry" });
    // The following would fail because first_vector is inferred as Vec<Human>:
    // first_vector.push(Cat { name: "Meeku" });

    // Approach 1: Using an enum to wrap different types.
    let mut enum_vector = Vec::new();
    enum_vector.push(Creature::Human(Human { name: "Kary" }));
    enum_vector.push(Creature::Cat(Cat { name: "Meeku" }));

    for creature in enum_vector {
        match creature {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk(),
        }
    }

    // Approach 2: Using dynamic dispatch with trait objects.
    // This allows us to store any type that implements Animal without the need for an enum.
    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human { name: "Kary" }));
    animals.push(Box::new(Cat { name: "Meeku" }));

    for animal in animals.iter() {
        animal.talk();
    }
}
