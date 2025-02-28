/*
    Rust's unique approach to memory management is built on the
    principles of ownership, borrowing, and lifetimes. In Rust, every
    value has a single owner responsible for cleanup, which helps prevent
    data races and memory leaks. This demo illustrates:
      - How ownership is transferred between variables.
      - How closures can consume values by taking ownership.
      - The difference between heap-allocated types (which enforce moves)
        and primitive types (which implement the Copy trait).
      - Using Box to explicitly allocate data on the heap.
*/

pub fn ownership_demo() {
    println!("Ownership Demo:");

    // Example: 'v' initially owns a vector allocated on the heap.
    let v = vec![1, 2, 3];

    // Transferring ownership: 'v2' takes ownership from 'v'.
    // This move ensures that there is only one valid owner at all times.
    let v2 = v;
    println!("v2 now owns the vector: {:?}", v2);

    // Ownership transfer in closures: The closure 'foo' consumes its argument.
    // After this move, 'v2' can no longer be used.
    let foo = |consumed_vec: Vec<i32>| {
        println!("Closure 'foo' has consumed the vector: {:?}", consumed_vec);
    };
    foo(v2);

    // Primitive types implement the Copy trait, so they are duplicated rather than moved.
    let u = 1;
    let _u2 = u;
    println!("Primitive integer 'u' remains accessible (by Copy): {}", u);

    // Using Box to allocate data on the heap explicitly:
    // 'u3' owns a heap-allocated integer. Moving it to '_u4' invalidates 'u3'.
    let u3 = Box::new(3);
    let _u4 = u3;
    println!("Heap-allocated Box has been moved successfully.");

    // Closure returning ownership:
    // 'print_vector' takes ownership of its vector parameter, prints it,
    // and then returns it, enabling the caller to keep using the vector.
    let v3 = vec![1, 2, 3];
    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("Inside closure, received vector: {:?}", x);
        x
    };
    let vv = print_vector(v3);
    println!("Returned vector from closure: {:?}", vv);
}
