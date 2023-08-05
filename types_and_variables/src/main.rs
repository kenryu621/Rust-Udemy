#[allow(dead_code)]
#[allow(unused_variables)]
use std::mem; // to determine the size of variables
mod sh;

fn main() {
    println!("Core variables: ");
    core_variables();
    println!("Operators: ");
    operators();
    println!("Constants: ");
    constants();
    println!("Stack and heap:");
    sh::stack_and_heap();
}

fn core_variables() {
    let a: u8 = 123; // u = unsigned, 8 bits (0 - 255), immutable (cannot be changed)
    println!("a = {}", a);
    // a = 199; would raise an error

    // u = unsigned, 0 ... 2^n-1
    // i = signed, -2^(n-1) ... 2^(n-1)-1
    let mut b: i8 = 122; // -128 to 127
    println!("b = {} before", b);
    b = 127;
    println!("b = {} now", b);

    // the compiler set the variable type to integer 32 bits by default
    let c = 123456789; // signed integer 32 bits, 32 bits = 4 bytes
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c)); // pass a pointer to the size_of_val macro

    // Available number variable types:
    // u8, u16, u32, u64, i8, i16, i32, i64
    // usize and isize
    // It would be really make sense to declare a variable
    // which is native to that processor and operating system.

    // char:
    let d: isize = 422;
    let size_of_d = mem::size_of_val(&d);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        d,
        size_of_d,
        size_of_d * 8
    );

    let e: char = 'x';
    println!("e = '{}', takes up {} bytes", e, mem::size_of_val(&e));

    // float point: f32 f64 SIGNED!
    let f: f32 = 2.5; // f64 is the default type for float point
    println!("f = {}, takes up {} bytes", f, mem::size_of_val(&f));

    // boolean: bool
    let g: bool = true; // or false
    println!("g = {}, takes up {} bytes", g, mem::size_of_val(&g));
}

fn operators() {
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);
    a = a + 1; // -- ++
    a -= 2; // -= += *= /= %=

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {:.2}, {}^pi = {:.2}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2;
    // | OR, & AND, ^ XOR, ! NOR
    // 01 | 10 = 11, 11 = 3 in decimal
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10; // shift operator >>, <<
    println!("2^10 = {}", two_to_10);

    // logical
    // >, <, >=, <=, ==
    let pi_less_than_4 = std::f64::consts::PI < 4.0; // true
    println!("pi < 4 = {}", pi_less_than_4);
    let x = 5;
    let x_is_5 = x == 5;
    println!("x == 5 is {}", x_is_5);
}

// Declaring and using constants
const MEANING_OF_LIFE: u8 = 42; // no fixed address
static mut Z: i32 = 26;
// A mutable variable that is accessible throughout the duration of the program
// could cause synchronization problem in running
// Different threads could write and read the variable in the same time

// In most cases, if the programmer only needs an immutable constant,
// the const is much better than static

fn constants() {
    println!("The meaning of life is {}", MEANING_OF_LIFE);
    // By declaring a unsafe scope means that the programmer understood
    // the possibility of unsafe operations
    unsafe {
        Z = 888;
        println!("The number of Z is {}", Z);
    }
}
