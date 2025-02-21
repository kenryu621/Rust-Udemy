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

pub fn enums() {
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
