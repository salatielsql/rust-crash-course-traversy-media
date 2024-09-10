// Vectors - Resizeble arrays

use std::mem;

// Fixed-length where elements are the same data types
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers.push(6);
    numbers.push(7);

    println!("{:?}", numbers);

    numbers.pop();
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

    // loop
    for x in numbers.iter() {
        println!("number: {}", x)
    }

    // loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("numbers multipled by 2 {:?}", numbers)
}