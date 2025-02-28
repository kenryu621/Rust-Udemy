#[allow(unused_mut)]
/*
    Borrowing in Rust allows you to reference data without taking ownership.
    This means you can temporarily "borrow" a value through a reference,
    leaving the original owner in full control of its data. This mechanism
    is fundamental for ensuring memory safety and enabling concurrency.
*/
pub fn borrowing_demo() {
    println!("Borrowing:");

    let print_vector = |x: &Vec<i32>| {
        println!("Print vector: {:?}", x);
    };

    let v = vec![1, 2, 3];
    print_vector(&v);
    println!("v: {:?}", v); // 'v' remains accessible after being borrowed.

    // Demonstrating mutable borrowing:
    // With non-lexical lifetimes (NNL), a mutable borrow automatically ends after its last use,
    // removing the need for an extra scope to limit the borrow's lifetime.
    let mut a = 40;
    let b = &mut a; // Mutable borrow begins here.
    *b += 2; // The borrow of 'a' by 'b' ends here after this use.
    println!("a = {}", a); // 'a' is usable again after the mutable borrow is over.

    let mut z = vec![3, 2, 1];
    for i in &z {
        println!("i = {}", i);
        // z.push(5); // Uncommenting this line would result in a compile-time error
        // because modifying 'z' while iterating over it is disallowed.
    }
}
