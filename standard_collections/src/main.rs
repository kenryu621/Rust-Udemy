mod hashmap;
mod hashset;
mod iterators;
mod vectors;
use std::io;

fn main() {
    println!("Choose a data structure to demo:");
    println!("1. Vectors");
    println!("2. HashMaps");
    println!("3. HashSet");
    println!("4. HashSet: Mathematic");
    println!("5. Iterators");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let input_result: i32 = input.trim().parse().expect("Input not an integer");

    match input_result {
        1 => vectors::vectors(),
        2 => hashmap::hashmap(),
        3 => hashset::hashset(),
        4 => hashset::set_math(),
        5 => iterators::demo(),
        _ => println!("Not an valid input."),
    }
}
