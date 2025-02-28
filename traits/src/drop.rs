/// The Drop trait in Rust provides a way to customize the cleanup process when an object goes out of scope.
/// It works similarly to a destructor in other languages, ensuring that resources are freed correctly.
/// Note that while Rust automatically calls drop when a value is no longer needed, you can also force early cleanup
/// using the standard library's drop function. However, you cannot call the drop method directly on an instance.
struct Creature {
    name: String,
}

impl Creature {
    /// Creates a new Creature with the specified name and prints a message indicating its entry into the game.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the creature.
    ///
    /// # Returns
    ///
    /// A Creature instance with the given name.
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    /// Automatically called when a Creature goes out of scope.
    ///
    /// This method is responsible for cleaning up resources.
    /// In this example, it simply prints a message indicating that the creature is dead.
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

/// Demonstrates the behavior of the Drop trait in Rust.
///
/// This function creates a Creature and then explicitly
/// drops it using Rust's standard library drop function. Although
/// values are generally dropped automatically when they go out of scope,
/// calling `drop` manually can be useful to free resources sooner.
/// Note that after calling `drop`, the variable `goblin` is no longer valid.
pub fn drop_demo() {
    println!("Drop:");

    let goblin = Creature::new("Jeff");
    println!("Game proceeds");
    // Although Rust automatically drops variables when they go out of scope,
    // we can explicitly drop the Creature early by passing it to the `drop` function.
    drop(goblin)
}
