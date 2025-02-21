/*
Union

Let's say the programmer want to allocate 32 bits of memory but didn't explicitly specify the
data type, instead you specify all the possibilities with Union.

Essentially, unions are primarily used for interoperating with C, C++, or similar languages.
It is not particularly convenient because an union does not carry the information about which
data type is carried inside. If this information is needed, just use enumerations, but you will
lose the compatibility to interoperate with some third party API that is written in C or C++.
*/
union IntOrFloat {
    i: i32,
    f: f32,
}

pub fn unions() {
    println!("Unions: ");
    // You can assign either integer or float point within the curly bracket
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234; // This can modify the value of the union
    let value = unsafe { iof.i };
    // We will need to use unsafe block to get the value of the union,
    // because the value in the union could be a different data type, and that could
    // raise type error. Therefore the unsafe block tells the compiler that the programmer
    // will going to assume the responsibility for anything.
    println!("iof.i = {}", value);

    fn process_value(iof: IntOrFloat) {
        unsafe {
            match iof {
                IntOrFloat { i: 42 } => {
                    println!("Meaning of life value")
                }
                IntOrFloat { f } => {
                    println!("Value = {}", f)
                }
            }
        }
    }
    process_value(IntOrFloat { i: 42 });
    process_value(IntOrFloat { f: 42.0 });
    process_value(IntOrFloat { i: 5 });
    // Since this integer of 5 didn't match with the first case in the match statement,
    // the compiler will attempt to recast the bits of the integer into a float point.
    // Thus we are treating the integer of 5 as a f32 value, and the last case in the match
    // statement will catch it.
}
