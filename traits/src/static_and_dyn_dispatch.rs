/*
    Dispatch in Rust refers to the mechanism by which the compiler determines which implementation
    of a function or trait method to invoke. There are two primary strategies:

    1. Static Dispatch:
       With static dispatch, the compiler uses generics to resolve function calls at compile time via
       monomorphisation. This process generates concrete versions of functions for each specific type used,
       enabling optimizations like inlining, though it may result in larger binaries due to code duplication.

    2. Dynamic Dispatch:
       Dynamic dispatch defers method resolution to runtime using trait objects (e.g., &dyn Trait). Instead of
       compile-time resolution, the necessary method is looked up through a virtual table (vtable), introducing
       a minor runtime cost for pointer indirection. This approach offers flexibility when working with heterogeneous types.
*/

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("String: {}", *self)
    }
}

// Example of Dynamic Dispatch:
// The function below demonstrates dynamic dispatch using a trait object.
// When invoked, the formatting method is resolved at runtime via the vtable.
fn print_dynamic_dispatch(z: &dyn Printable) {
    println!("{}", z.format());
}

/// Static Dispatch via Monomorphisation:
/// This generic function, print_static_dispatch, works with any type implementing Printable.
/// For each unique type passed, Rust generates a specific instance of this function at compile time,
/// enabling efficient inlined code without runtime dispatch overhead.
fn print_static_dispatch<T: Printable>(z: T) {
    println!("{}", z.format());
}

pub fn static_and_dyn_dispatch_demo() {
    println!("Static Dispatch Demonstration:");
    let a = 123;
    let b = "Hello".to_string();

    // Using static dispatch
    print_static_dispatch(a);
    print_static_dispatch(b.clone());

    // Using dynamic dispatch
    print_dynamic_dispatch(&a);
    print_dynamic_dispatch(&b);
}
