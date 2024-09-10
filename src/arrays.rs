use std::mem;

// Fixed-length where elements are the same data types
pub fn run() {
    let mut numbers: [i32;5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // get single value
    println!("index 0: {}", numbers[0]);

    // change value
    numbers[0] = 20;

    println!("index 0: {}", numbers[0]);

    // length
    println!("length: {}", numbers.len());

    // arrays are stack allocated
    println!("memory size: {} bytes", mem::size_of_val(&numbers));

    // array slices
    let sliced: &[i32] = &numbers[1..2];
    println!("slice: {:?}", sliced);
}