fn vectors() {
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

use std::collections::HashMap;

fn hash_map() {
    println!("Hash Map:");
    let mut shapes = HashMap::new();
    shapes.insert(String::from("dot"), 1);
    shapes.insert(String::from("line"), 2);
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    shapes.insert(String::from("diamond"), 6);

    // The program could crash if the key doesn't exist in the hash map
    println!("a square has {} sides", shapes["square".into()]);

    // Iterating the hash map using for loop
    for (key, value) in &shapes {
        println!("{}: {}", key, value);
    }

    // Overriding content of the map
    shapes.insert("diamond".into(), 4);
    println!("Corrected diamond value, shapes = {:?}", shapes);

    // Use entry function if we want to check a key exist in the map
    // Then use or_insert function if the key doesn't exist
    shapes.entry("circle".into()).or_insert(1);
    // We can also use the entry function to return a reference to the value
    // Then we can modify the reference using assignment
    {
        let circle_ref = shapes.entry("circle".into()).or_insert(2);
        *circle_ref = 0;
    }
    println!("{:?}", shapes);
}

use std::collections::HashSet;
use std::hash::Hash;

fn hash_set() {
    println!("Hash Set:");
    // Initializing new Hash Set
    let mut greeks = HashSet::new();
    // Use insert function to add values
    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("beta");
    println!("{:?}", greeks);
    // Nothing will happen if we try to insert a value that already exist
    // The insert function will return a boolean value to indicate if the insertion was succeed
    let added_result = greeks.insert("vega");
    if added_result {
        println!("New value added!");
    } else {
        println!("Nothing added");
    }
    // Use contains function to check if a value exist
    if !greeks.contains("kappa") {
        println!("We don't have kappa");
    }
    // Use remove function to delete a value
    // Just like insert, it will return a bool for the operation result
    let removed = greeks.remove("delta");
    if removed {
        println!("We removed delta");
    }
    println!("{:?}", greeks);

    // Next we will talk about comparison between sets
    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // subset, every single element in a set exist in another set
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );

    // disjoint, two sets does not have common element
    println!(
        "is {:?} a disjoint from {:?}? {}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    // Union, the combined set of two sets
    println!(
        "the union of {:?} and {:?}: {:?}",
        _1_5,
        _6_10,
        _1_5.union(&_6_10)
    );
    // intersection, the common elements between two sets
    println!(
        "intersection between {:?} and {:?}: {:?}",
        _2_8,
        _1_5,
        _2_8.intersection(&_1_5)
    );
    // difference, items that are in the first set but not in the second set
    println!(
        "difference between {:?} and {:?}: {:?}",
        _1_5,
        _2_8,
        _1_5.difference(&_2_8)
    );
    // symmetric_difference, items that are not in common between the two sets, union - intersection
    println!(
        "symmetric difference between {:?} and {:?}: {:?}",
        _1_5,
        _2_8,
        _1_5.symmetric_difference(&_2_8)
    );
}

fn main() {
    vectors();
    hash_map();
    hash_set();
}
