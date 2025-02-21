use std::mem;

pub fn arrays() {
    // An array is a data structure where you know the size, in other words,
    // how many elements you want in advance.
    //
    // Initializing an array of five integers
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);

    // Changing the element in the array
    a[0] = 321;
    println!("a[0] = {}", a[0]);

    // Debugging the array in a simpler way, the colon and question mark will
    // represent a debug kind of output for the array
    println!("{:?}", a);

    // We can also check whether the array has a particular value
    if a != [1, 2, 3, 4, 5] {
        println!("Does not match");
    }

    // A way to bulk fill an array with the same value:
    // This will initialize an array of ten elements that is equal to 1
    let b = [1; 10];
    for i in 0..b.len() {
        println!("{}", b[i]);
    }
    // Printing out the size of the array
    println!("The array b took up {} bytes", mem::size_of_val(&b));
    // We can also specify the data types to reduce the size of array
    // The u16 specified the 1 will be 16-bit unsigned integer
    // This is the same as let c:[u16;10] = [1;10];
    let c = [1u16; 10];
    println!("The array c took up {} bytes", mem::size_of_val(&c));
    // An array cannot be resized, so you cannot make it smaller or larger,
    // you always have a fixed size.
}

pub fn multi_dimenstional_arrays() {
    // Define a multi-dimensional array to have three columns and two rows
    let mda: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mda);
    // Let's suppose that we want to print out all the diagonal values
    // Using iterators and loops will be a bit complicated
    for i in 0..mda.len() {
        for j in 0..mda[i].len() {
            if i == j {
                // The value is at the diagonal line when i equals j
                println!("mda[{}][{}] = {}", i, j, mda[i][j]);
            }
        }
    }
}

/*
A slice is essentially a pattern of an array.
But unlike array, its actual size is not known at the compile time.
The ampersand and the square bracket are basically borrowing a part of
an array as a slice.
*/
fn use_slice(slice: &mut [i32]) {
    println!("First element = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321; // Changing the element of the slice will also effect the original array
}

pub fn slices() {
    println!("Slices:");
    let mut data = [1, 2, 3, 4, 5]; // the compiler knows there's five element
                                    // Now we can feed the use_slice function with a chunk of data
    use_slice(&mut data[1..4]);
    println!("{:?}", data); // The changes from use_slice will effect the original array
}
