fn how_many(x: i32) -> &'static str {
    // this function will return a static string
    match x {
        0 => "no",
        1 | 2 => "one or two", // The vertial bar means or
        12 => "a dozen",
        _z @ 9..=11 => "lots of", // The two dots with equal sign means an inclusive range
        // You can also give the range a name like the above with @
        _ if (x % 2 == 0) => "some", // You can also use logical statement for the catch all cases
        _ => "a few",
    }
}

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8),
    CMYK {
        cyan: u8,
        magneta: u8,
        yellow: u8,
        black: u8,
    },
}

pub fn pattern_matching() {
    // Suppose we want to print out the number of aranges we have
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (0, 0);
    match point {
        // You can also use variables while pattern matching
        // You can also add keyword at front of the variable
        // For example, ref mut x to make x a reference mutable variable
        (0, 0) => println!("Origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("x = {}, y axis", x),
        (x, y) => println!("x = {}, y = {}", x, y),
    }

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
        // We can use two dots to indicate that we only care about the black value
        Color::RGBColor(0, 0, 0) | Color::CMYK { black: 255, .. } => println!("Black"),
        Color::RGBColor(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
        Color::CMYK {
            cyan: c,
            magneta: m,
            yellow: y,
            black: k,
        } => println!("CMYK({}, {}, {}, {})", c, m, y, k),
    }
}
