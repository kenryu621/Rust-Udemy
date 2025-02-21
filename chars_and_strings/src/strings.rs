/*
Rust has two main string types: String and &str
Both are UTF-8 encoded sequences of characters, ensuring proper handling of Unicode text.

Key differences:
- String: Owned, growable, heap-allocated string
- &str: Immutable string slice reference, often static or borrowed
*/

pub fn strings() {
    let s: &'static str = "Hello there!";
    /*
    &str is a string slice - a view into a string, similar to array/vector slices.
    The 'static lifetime means this string is embedded directly in the compiled binary,
    with a fixed memory location that persists for the program's entire duration.

    Why we can't index directly into &str:
    - UTF-8 is a variable-width encoding (1-4 bytes per character)
    - Indexing would access individual bytes, not characters
    - This could lead to invalid UTF-8 sequences or incorrect character access
    - Rust prevents this to maintain string safety guarantees

    Instead, we use the chars() iterator to safely access Unicode characters
    */
    // Iterate through characters in the string slice using chars() iterator
    for c in s.chars() {
        println!("{}", c);
    }

    // Safely access the first character using nth() on chars() iterator
    // nth() returns an Option<char> to handle empty strings gracefully
    if let Some(first_char) = s.chars().nth(0) {
        println!("First letter is {}", first_char);
    }

    // Demonstrating String manipulation - a growable, heap-allocated string
    let mut letters = String::new();
    let mut a = 'a' as u8;
    // Build a string with alphabet letters separated by commas
    while a <= ('z' as u8) {
        letters.push(a as char); // Append character
        letters.push_str(","); // Append string slice
        a += 1;
    }
    println!("Generated letters: {}", letters);

    // Conversion between String and &str
    let _u: &str = &letters; // Deref conversion: String to &str
    let mut abc = String::from("Hello world"); // &str to String using from()
    let mut _abc2 = "Hello world".to_string(); // &str to String using to_string()

    // String manipulation examples
    abc.remove(0); // Remove first character
    abc.push_str("!!!"); // Append string slice
    println!("Modified string: {}", abc.replace("ello", "goodbye"));

    // String concatenation examples
    // String + &str concatenation (letters is String, "abc" is &str)
    let _z = letters + "abc";
    // Note: The following is commented out as it would consume letters twice
    // let _z_string = letters + &letters; // String + &String (via deref coercion)
}
