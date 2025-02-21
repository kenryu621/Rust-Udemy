/*
A set is simply a bunch of unique values which are stored
in some sort of container, and you can verify whether an element
is stored in that container.
It is like a vector except that all the elements are unique and
no guarantees about their order.
*/

use std::collections::HashSet;

pub fn hashset() {
    println!("HashSet:");
    let mut greeks = HashSet::new();
    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    // Nothing will happens if we try to insert delta again
    println!("Nothing will happens if we try to insert delta again:");
    greeks.insert("delta");
    println!("{:?}", greeks);

    // We can actually detect if an insertion succeed, the insert() will return a Boolean result
    let added_vega: bool = greeks.insert("vega");
    if added_vega {
        println!("We added vega to the set!")
    }
    let added_delta: bool = greeks.insert("delta");
    if added_delta {
        println!("We just added delta?")
    } else {
        println!("Failed to add delta to the set.")
    }

    // We can also check if the HashSet contains a certain value
    println!("set.contains():");
    if !greeks.contains("kappa") {
        println!("We don't have kappa in the set.");
    }

    // We can also remove a value from the set, and it will also return a Boolean result
    let removed_delta = greeks.remove("delta");
    if removed_delta {
        println!("Delta is removed from the set: {:?}", greeks);
    }
}

/*
We can also perform typical set operations that we have in mathematics.
We are going to start with comparisons, things like subsets, supersets, etc.
*/
pub fn set_math() {
    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    // Subset
    // Remember that by default, elements in set are in random order
    println!(
        "Is {:?} a subset of {:?}? {}",
        _2_8,
        _1_10,
        _2_8.is_subset(&_1_10)
    );

    // Disjoint: No common element
    println!(
        "Is {:?} a disjoint of {:?}? {}",
        _1_5,
        _6_10,
        _1_5.is_disjoint(&_6_10)
    );

    // Union = elements from both set
    println!(
        "Items in either {:?} and {:?} are {:?}",
        _2_8,
        _6_10,
        _2_8.union(&_6_10)
    );
    // intersection = elements that present in both set
    println!(
        "Items in both {:?} and {:?} are {:?}",
        _2_8,
        _6_10,
        _2_8.intersection(&_6_10)
    );
    // difference = elements in the first set but not in the second
    println!(
        "Difference between {:?} and {:?} are {:?}",
        _1_10,
        _6_10,
        _1_10.difference(&_6_10)
    );
    // symmetric_difference = elements that are in the union but not in the intersection (union - intersection)
    println!(
        "Symmetric difference between {:?} and {:?} are {:?}",
        _2_8,
        _1_5,
        _2_8.symmetric_difference(&_1_5)
    );
}
