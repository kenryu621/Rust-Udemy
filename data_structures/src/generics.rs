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

pub fn generics() {
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
