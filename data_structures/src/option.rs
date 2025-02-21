// Option<T>, which is a fundamental building block of Rust
pub fn option_t() {
    println!("Option<T>: ");
    // Let's say we are performing a division
    let x = 3.0;
    let y = 0.0;
    // Dividing by 0 does not provide us any meaningful result.
    //
    // When we perform the calculation, we can use Option type to indicate whether or not
    // the result actually succeed.
    //
    // Option has two possible value, either you have a Some where you specify some value
    // that you actually got, or if your operation didn't work and no result to yield, you
    // will return none.
    // Option -> Some(v) | None
    let result = if y != 0.0 { Some(x / y) } else { None };
    // We can use match statement as a way to test the Option type
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Cannot divide by zero"),
    }
    // We can also use if let keyword for testing, this statement will execute if
    // the Some value match with the result, otherwise if result is None, then it wouldn't execute
    //
    // The let statement allows the programmer to check whether or not the right hand side can
    // be assigned to the left hand side. If it can be assigned, then the let statement will return True
    //
    // You can also use that technique and create while let statements
    if let Some(z) = result {
        println!("Result = {}", z)
    }
}
