/*
    Although static dispatch is optimized at compile time for speed and efficiency, it requires knowing
    the concrete types in advance. Dynamic dispatch, on the other hand, gives us the flexibility to work
    with heterogeneous collections of types that implement a common trait. In this example, an array of
    &dyn Shape trait objects allows us to handle both Circle and Square instances uniformly. This is
    crucial when the exact types may vary or be unknown at compile time.
*/

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

trait Shape {
    /// Calculates and returns the area of the shape.
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

pub fn why_dyn_dispatch_demo() {
    println!("Why Dynamic Dispatch:");

    // Here we create an array of trait objects (&dyn Shape) containing both circles and squares.
    // Since the concrete types vary, the method calls for calculating the area are resolved at runtime.
    let shapes: [&dyn Shape; 4] = [
        &Circle { radius: 1.0 },
        &Square { side: 3.0 },
        &Circle { radius: 2.0 },
        &Square { side: 4.0 },
    ];

    // Iterate over the shapes, using dynamic dispatch to call each shape's area method.
    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {}", i, shape.area());
    }
}
