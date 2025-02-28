/*
    In Rust, a lifetime defines the scope during which a reference is valid.
    Lifetimes are a compile-time feature that guarantees references do not outlive the data they point to.
    For example, the 'static lifetime signifies that the data is available for the entire duration of the program.
    In addition to 'static, you can define custom lifetimes to tie the scope of references to specific variables.
*/
#![allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    // This method demonstrates lifetime elision in Rust.
    // Lifetime elision is a feature where Rust automatically infers lifetimes for references in function signatures.
    // Here, since the method takes '&self', the compiler infers that the returned reference (&String)
    // has the same lifetime as 'self'. In other words, the output is valid for as long as the Person instance exists.
    fn get_ref_name(&self) -> &String {
        &self.name
    }
}

/// The Company struct uses explicit lifetime annotations to ensure memory safety.
/// The lifetime parameter 'z for the ceo reference guarantees that the ceo outlives the Company instance.
/// This explicit linkage prevents the ceo from being dropped while still in use by the Company.
#[derive(Debug)]
struct Company<'z> {
    name: String,
    ceo: &'z Person,
    /*
       Specifying a lifetime for the ceo reference prevents scenarios where the referenced Person might
       be dropped before the Company, which would otherwise lead to a dangling reference.
       Rust's compile-time checks enforce that the lifetimes are compatible.
    */
}

pub fn lifetime_demo() {
    println!("Lifetime Demo:");

    let boss = Person {
        name: String::from("Elon Musk"),
    };
    let tesla = Company {
        name: String::from("Tesla"),
        ceo: &boss,
    };

    println!("Tesla: {:?}", tesla);

    // The following code demonstrates a lifetime violation.
    // The function get_ref_name returns a reference to a Person's name, but if the Person is dropped
    // (as it would be when declared inside an inner block), the reference becomes invalid.
    // This code is commented out to prevent a compile-time error.
    /*
    #[allow(unused_mut)]
    let mut z: &String;
    {
        let p = Person {
            name: String::from("Alex"),
        };
        // Due to lifetime elision, get_ref_name is seen as having the same lifetime as 'p'.
        // Since 'p' is dropped after this block, 'z' would point to invalid memory,
        // which Rust correctly disallows.
        z = p.get_ref_name();
    }
    println!("{:?}", z);
    */
}
