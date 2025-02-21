/*
A tuple is simply several values taken together by put them in round brackets
*/
fn sum_and_the_product(x: i32, y: i32) -> (i32, i32) {
    // This function will return a tuple of sum and product
    // This is almost the same as returning an array, but you can have different data type
    (x + y, x * y)
}

pub fn tuples() {
    println!("Tuples:");
    // Let's suppose we have two numbers X and Y, and we want to get their sum and product
    let x = 3;
    let y = 4;
    let sp = sum_and_the_product(x, y);
    println!("sp = {:?}", sp);
    // You would use dot and number to extract specific data from tuples
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // Destructuring
    // This essentially means that I want to destruct the tuple sp,
    // assuming the structure of the sp matches the structure of the left hand side
    // Assigning the first element to a, and second element to b
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    // tuple of tuple
    let sp2 = sum_and_the_product(4, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    // Destructuring such multi-dimensional tuples is not so convenient
    println!("Last element = {}", (combined.1).1); // You can't just go combined.1.1
    let ((_c, _d), (_e, _f)) = combined;

    // Again, you can have mix match data types in a tuple
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // What if you want a tuple of a single element?
    let meaning = (42,); // You just leave the comma in the tuple
    println!("{:?}", meaning);
}
