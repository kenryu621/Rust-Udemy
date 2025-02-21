use std::collections::HashMap;

pub fn hashmap() {
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
