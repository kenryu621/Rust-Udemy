#![allow(dead_code)]
use std::mem;
mod pm;

// Structure
struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    println!("Structures: ");
    let p = Point { x: 3.0, y: 4.0 };
    println!("Point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };
    println!(
        "My line start from ({}, {}) and end at ({}, {})",
        myline.start.x, myline.start.y, myline.end.x, myline.end.y
    );
}

// Enumerations
enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8), // Tuple
    CMYK {
        // Struct within enumeration
        cyan: u8,
        magneta: u8,
        yellow: u8,
        black: u8,
    },
}

fn enums() {
    println!("Enumerations: ");
    let c: Color = Color::CMYK {
        cyan: 1,
        magneta: 0,
        yellow: 0,
        black: 10,
    };
    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        Color::Blue => println!("B"),
        Color::RGBColor(0, 0, 0)
        | Color::CMYK {
            cyan: _,
            magneta: _,
            yellow: _,
            black: 255,
        } => println!("Black"),
        Color::RGBColor(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
        Color::CMYK {
            cyan: c,
            magneta: m,
            yellow: y,
            black: k,
        } => println!("CMYK({}, {}, {}, {})", c, m, y, k),
    }
}

/*
Union

Let's say the programmer want to allocate 32 bits of memory but didn't explicitly specify the
data type, instead you specify all the possibilities with Union.

Essentially, unions are primarily used for interoperating with C, C++, or similar languages.
It is not particularly convenient because an union does not carry the information about which
data type is carried inside. If this information is needed, just use enumerations, but you will
lose the compatibility to interoperate with some third party API that is written in C or C++.
*/
union IntOrFloat {
    i: i32,
    f: f32,
}

fn unions() {
    println!("Unions: ");
    // You can assign either integer or float point within the curly bracket
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234; // This can modify the value of the union
    let value = unsafe { iof.i };
    // We will need to use unsafe block to get the value of the union,
    // because the value in the union could be a different data type, and that could
    // raise type error. Therefore the unsafe block tells the compiler that the programmer
    // will going to assume the responsibility for anything.
    println!("iof.i = {}", value);

    fn process_value(iof: IntOrFloat) {
        unsafe {
            match iof {
                IntOrFloat { i: 42 } => {
                    println!("Meaning of life value")
                }
                IntOrFloat { f } => {
                    println!("Value = {}", f)
                }
            }
        }
    }
    process_value(IntOrFloat { i: 42 });
    process_value(IntOrFloat { f: 42.0 });
    process_value(IntOrFloat { i: 5 });
    // Since this integer of 5 didn't match with the first case in the match statement,
    // the compiler will attempt to recast the bits of the integer into a float point.
    // Thus we are treating the integer of 5 as a f32 value, and the last case in the match
    // statement will catch it.
}

// Option<T>, which is a fundamental building block of Rust
fn option_t() {
    println!("Option<T>: ");
    // Let's say we are performing a division
    let x = 3.0;
    let y = 0.0;
    // Dividing by 0 does not provide us any meaningful result.
    //
    // When we perform the calculation, we can use Option type to indicate whether or not
    // the result actually succeed.
    //
    // Option has two possible value, either you have a Some where you specify some value
    // that you actually got, or if your operation didn't work and no result to yield, you
    // will return none.
    // Option -> Some(v) | None
    let result = if y != 0.0 { Some(x / y) } else { None };
    // We can use match statement as a way to test the Option type
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Cannot divide by zero"),
    }
    // We can also use if let keyword for testing, this statement will execute if
    // the Some value match with the result, otherwise if result is None, then it wouldn't execute
    //
    // The let statement allows the programmer to check whether or not the right hand side can
    // be assigned to the left hand side. If it can be assigned, then the let statement will return True
    //
    // You can also use that technique and create while let statements
    if let Some(z) = result {
        println!("Result = {}", z)
    }
}

fn arrays() {
    // An array is a data structure where you know the size, in other words,
    // how many elements you want in advance.
    //
    // Initializing an array of five integers
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);

    // Changing the element in the array
    a[0] = 321;
    println!("a[0] = {}", a[0]);

    // Debugging the array in a simpler way, the colon and question mark will
    // represent a debug kind of output for the array
    println!("{:?}", a);

    // We can also check whether the array has a particular value
    if a != [1, 2, 3, 4, 5] {
        println!("Does not match");
    }

    // A way to bulk fill an array with the same value:
    // This will initialize an array of ten elements that is equal to 1
    let b = [1; 10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }
    // Printing out the size of the array
    println!("The array b took up {} bytes", mem::size_of_val(&b));
    // We can also specify the data types to reduce the size of array
    // The u16 specified the 1 will be 16-bit unsigned integer
    // This is the same as let c:[u16;10] = [1;10];
    let c = [1u16; 10];
    println!("The array c took up {} bytes", mem::size_of_val(&c));
    // An array cannot be resized, so you cannot make it smaller or larger,
    // you always have a fixed size.
}

fn multi_dimenstional_arrays() {
    // Define a multi-dimensional array to have three columns and two rows
    let mda: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mda);
    // Let's suppose that we want to print out all the diagonal values
    // Using iterators and loops will be a bit complicated
    for i in 0..mda.len() {
        for j in 0..mda[i].len() {
            if i == j {
                // The value is at the diagonal line when i equals j
                println!("mda[{}][{}] = {}", i, j, mda[i][j]);
            }
        }
    }
}

/*
A slice is essentially a pattern of an array.
But unlike array, its actual size is not known at the compile time.
The ampersand and the square bracket are basically borrowing a part of
an array as a slice.
*/
fn use_slice(slice: &mut [i32]) {
    println!("First element = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321; // Changing the element of the slice will also effect the original array
}

fn slices() {
    println!("Slices:");
    let mut data = [1, 2, 3, 4, 5]; // the compiler knows there's five element
                                    // Now we can feed the use_slice function with a chunk of data
    use_slice(&mut data[1..4]);
    println!("{:?}", data); // The changes from use_slice will effect the original array
}

/*
A tuple is simply several values taken together by put them in round brackets
*/
fn sum_and_the_product(x: i32, y: i32) -> (i32, i32) {
    // This function will return a tuple of sum and product
    // This is almost the same as returning an array, but you can have different data type
    (x + y, x * y)
}

fn tuples() {
    println!("Tuples:");
    // Let's suppose we have two numbers X and Y, and we want to get their sum and product
    let x = 3;
    let y = 4;
    let sp = sum_and_the_product(x, y);
    println!("sp = {:?}", sp);
    // You would use dot and number to extract specific data from tuples
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // Destructuring
    // This essentially means that I want to destruct the tuple sp,
    // assuming the structure of the sp matches the structure of the left hand side
    // Assigning the first element to a, and second element to b
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    // tuple of tuple
    let sp2 = sum_and_the_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    // Destructuring such multi-dimensional tuples is not so convenient
    println!("Last element = {}", (combined.1).1); // You can't just go combined.1.1
    let ((_c, _d), (_e, _f)) = combined;

    // Again, you can have mix match data types in a tuple
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // What if you want a tuple of a single element?
    let meaning = (42,); // You just leave the comma in the tuple
    println!("{:?}", meaning);
}

/*
Generics
If we don't want to specify explicitly what the type of the coordinate is,
we can add a generic parameter called T
Instead of using the f64 for coordinates, it is going to use the type T
You can also add another type parameter, like GenericPoint<T, V>
*/
struct GenericPoint<T> {
    x: T,
    y: T,
}

struct GenericLine<T> {
    // The T in the line must match with the T in the points
    // Which essentially means the points and the lines need to have the same type
    // So we cannot create a line with a point with float point value and another point with
    // integer value
    start: GenericPoint<T>,
    end: GenericPoint<T>,
}

fn generics() {
    println!("Generic:");
    // Now we can create a point with any type X and Y
    let a = GenericPoint { x: 0.0, y: 4f64 };
    let b = GenericPoint { x: 1.2, y: 3.4 };
    let myline = GenericLine { start: a, end: b };
    println!(
        "The line starts from ({},{}) to ({},{})",
        myline.start.x, myline.start.y, myline.end.x, myline.end.y
    );
}

fn main() {
    structures();
    enums();
    unions();
    option_t();
    arrays();
    multi_dimenstional_arrays();
    slices();
    tuples();
    pm::pattern_matching();
    generics();
}
