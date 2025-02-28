/*
    Arc (Atomic Reference Counting) is a thread-safe reference counting pointer that enables shared ownership
    of immutable data across multiple threads. Unlike Rc, which is limited to single-threaded scenarios, Arc uses
    atomic operations to safely track the number of references, making it ideal for multi-threaded environments.
*/
/*
    When sharing mutable data between threads, Arc alone is insufficient because it does not prevent race conditions.
    To safely allow mutation, we wrap the data in a Mutex. A Mutex (mutual exclusion) is a synchronization primitive
    that ensures only one thread can access the contained data at a time. When a thread locks the Mutex, other threads
    attempting to lock it will block until the Mutex is released. This guarantees that concurrent modifications do not
    corrupt the data.
*/

use std::{
    sync::{Arc, Mutex},
    thread,
};

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>,
}

impl Person {
    /// Creates a new Person with an Arc-wrapped name and state protected by a Mutex.
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person {
        Person { name, state }
    }

    /// Greets by printing the Person's name along with a state that is updated in a thread-safe manner.
    fn greet(&self) {
        // Lock the Mutex to obtain exclusive access to the state.
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");

        println!("Hi, my name is {} and I am {}", self.name, state.as_str());
    }
}

/// Demonstrates thread-safe sharing of data using Arc and Mutex.
///
/// This function wraps a string in an Arc and spawns a new thread to invoke the greeting method on a Person instance.
/// It showcases how Arc enables safe sharing of immutable data between threads, while Mutex ensures that mutable data
/// is accessed by only one thread at a time.
pub fn arc_var_and_mutex_demo() {
    println!("Atomic Reference-Counted Variables:");

    let name = Arc::new("Alex".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(name.clone(), state.clone());

    // Spawn a new thread to demonstrate that Arc can be shared between threads.
    let t = thread::spawn(move || {
        person.greet();
    });

    // Using Arc ensures that 'name' can be shared safely across threads.
    println!(
        "Name = {}, state = {}",
        name,
        state.lock().unwrap().as_str()
    );
    t.join().unwrap();
}
