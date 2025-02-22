/*
Methods in Rust are functions associated with a struct or enum that operate on instances of that type.
Key concepts:

1. Implementation Blocks: Methods are defined within an `impl` block for a specific type
2. Self Parameter: The first parameter is always `self`, representing the instance
   - `&self` for immutable reference
   - `&mut self` for mutable reference
   - `self` for ownership transfer
3. Associated Functions: Functions without `self` parameter (like constructors)
4. Method Chaining: Methods can return `Self` to enable chaining
5. Namespacing: Methods are called using the dot notation on instances

This example demonstrates basic method implementation for calculating line length.
*/

/// Represents a point in 2D space
struct Point {
    x: f64,
    y: f64,
}

/// Represents a line segment between two points
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    /// Calculates the length of the line segment
    /// # Returns
    /// The Euclidean distance between start and end points (f64)
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Demonstrates method usage in Rust
pub fn methods() {
    println!("Methods:");

    // Create points and line
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let myline = Line { start: p, end: p2 };

    // Call method using dot notation
    println!("Length of my line: {}", myline.len());
}
