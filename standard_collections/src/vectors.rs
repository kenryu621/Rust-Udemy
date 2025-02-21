pub fn vectors() {
    println!("Vector:");
    // Initialize a new Vec
    let mut a = Vec::new();
    // Add new elements using push method
    a.push(1);
    a.push(2);
    a.push(3);
    // Printing the Vec using the debug output
    println!("a = {:?}", a);
    // We can call the element of the vector using indices
    println!("a[0] = {:?}", a[0]);
    // The number in the bracket is actually a size variable
    // usize (unsigned) / isize (signed)
    // let idx:i32 = 0; wont work because first memory addresses cannot be signed
    // Second our system is running on 64 bit.
    let idx: usize = 2;
    // If the usize is bigger than the actual length of the vector, we will run into
    // index out of bound error. We can use the get function to prevent this
    a[idx] = 312;
    println!("a[2] = {:?}", a[idx]);
    // The get function return an Option type
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("No such element"),
    }

    // Iterating the vector
    for x in &a {
        println!("{}", x)
    }

    // Removing the element
    a.push(404);
    println!("a = {:?}", a);
    // The pop function will attempt to remove and return the last element
    // The pop function will return an Option type first, just like get function
    // If the vector is empty therefore nothing to pop, it will return None
    let last_elem = a.pop();
    match last_elem {
        Some(x) => println!("last elem is {}, a = {:?}", x, a),
        None => println!("Nothing to pop"),
    }

    // The following will continue to run until the pop doesn't return some
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
