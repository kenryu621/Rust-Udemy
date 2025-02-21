// Structure
struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

pub fn structures() {
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
