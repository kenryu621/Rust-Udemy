/*
Demonstrates Rust iterator usage and common patterns.

Iterators provide a safe, efficient way to work with collections by:
- Allowing lazy chaining of operations (map, filter, etc)
- Avoiding index bounds checks
- Supporting immutable, mutable, and consuming access patterns

This demo shows:
- Basic iteration with references vs. iter()
- Using iterator adapters like rev()
- Mutable iteration with iter_mut()
- Consuming iteration with into_iter()
- The extend() method that consumes an iterator
*/
pub fn demo() {
    let mut vec = vec![3, 2, 1];

    // ordinary iteration (for x in vec) causes a move
    for x in &vec {
        println!("{}", *x);
    }

    // iter() = a bunch of immutable references
    for x in vec.iter() {
        println!("we got {}", x);
        // cannot modify things!
        // x += 1;
    }

    // iter adapter methods
    for x in vec.iter().rev() {
        println!("in reverse: {}", x);
    }

    // iter_mut() - mutable references, requires
    //              the vector to be declared mut
    for x in vec.iter_mut() {
        *x += 2;
    }
    println!("{:?}", vec);

    // into_iter() - move operation that transforms the collection into a by-value iterator
    //               not the same as ordinary iteration!
    //               useful when you need values but not the collection itself
    // extend() - automatically calls into_iter() to move elements from one collection to another
    let mut vec2 = vec![1, 2, 3];
    vec2.extend(vec);
    println!("{:?}", vec2);
}
