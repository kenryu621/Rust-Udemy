#![allow(dead_code)]
use std::io;
mod arrays;
mod enumerations;
mod generics;
mod option;
mod pm;
mod structures;
mod tuples;
mod unions;

fn main() {
    println!("Choose a data structure to demo:");
    println!("1. Structures");
    println!("2. Enumerations");
    println!("3. Unions");
    println!("4. Option<T>");
    println!("5. Arrays");
    println!("6. Multi-dimensional Arrays");
    println!("7. Slices");
    println!("8. Tuples");
    println!("9. Pattern Matching");
    println!("10. Generics");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let input_result: i32 = input.trim().parse().expect("Input not an integer");

    match input_result {
        1 => structures::structures(),
        2 => enumerations::enums(),
        3 => unions::unions(),
        4 => option::option_t(),
        5 => arrays::arrays(),
        6 => arrays::multi_dimenstional_arrays(),
        7 => arrays::slices(),
        8 => tuples::tuples(),
        9 => pm::pattern_matching(),
        10 => generics::generics(),
        _ => println!("Not an valid input."),
    }
}
