/*
String Formatting in Rust

The format! macro provides a powerful way to create formatted strings, similar to println! but returns a String instead of printing to stdout.

Key Features:
- Positional arguments: {} for sequential arguments, {0} {1} for indexed arguments
- Named arguments: {name} for named parameters
- Mixed formatting: Combine positional and named arguments
- Compiler validation: Ensures all arguments are used and types match

The format! macro is type-safe and checked at compile time, preventing common string formatting errors.
*/

pub fn string_formatting() {
    println!("String Formatting Examples:");

    // Basic formatting with single argument
    let name = "Kenry";
    let greeting = format!("Hi, I'm {}, nice to meet you!", name);
    println!("Basic: {}", greeting);

    // Multiple arguments with sequential placeholders
    let hello = "Hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("Multiple Arguments: {}", hello_rust);

    // Positional indexing with {0}, {1} syntax
    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}!", run, forest);
    println!("Positional Indexing: {}", rfr);

    // Named arguments for clearer formatting
    let info = format!(
        "The surname is {last}. {first} {last}",
        first = "Kenry",
        last = "Yu"
    );
    println!("Named Arguments: {}", info);

    // Mixed formatting combining positional and named arguments
    let mixed = format!("{1} {} {0} {} {data}", "alpha", "beta", data = "delta");
    println!("Mixed Formatting: {}", mixed);

    /*
    Compiler Safety:
    - The Rust compiler will catch unused arguments
    - Example: Adding "gamma" to the mixed format! call would trigger:
      "argument never used" warning
    - This prevents common formatting errors and ensures type safety
    */
}
