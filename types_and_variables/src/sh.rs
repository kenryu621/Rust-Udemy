#![allow(dead_code)]
#![allow(unused_variables)]
use std::mem;

struct Point {
    // this struct takes up 16 bytes because it stored two float 64 bits
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin()); // This would take up 8 bytes because an address is 64 bits
    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));
    let p3 = *p2; // The star sign could let the variable follow the address
}
