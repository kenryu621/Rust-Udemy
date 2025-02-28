/// The `Person` struct demonstrates the usage of lifetime annotations in a struct that holds references.
/// Lifetime annotations ensure that the data referenced within the struct remains valid for as long as the struct is in use.
struct Person<'a> {
    /// The `name` field is a string slice that must live at least as long as lifetime `'a`.
    name: &'a str,
}

/// Implementation of methods for `Person`. The lifetime specifier ensures that any reference used within
/// the instance is valid when accessed.
impl<'a> Person<'a> {
    /// The `talk` method prints a greeting including the person's name.
    fn talk(&self) {
        println!("Hi, my name is {}.", self.name)
    }
}

/// Demonstrates lifetime annotations in a structure.
/// This function creates a `Person` instance with a string literal (which has a `'static` lifetime)
/// and calls its `talk` method.
pub fn lifetime_in_struct_demo() {
    println!("Lifetime in Structure Implementation:");
    let person = Person { name: "Derek" };
    person.talk();
}
