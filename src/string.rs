pub fn run() {
    let mut hello = String::from("Hello ");

    println!("{}", hello);
    
    // get length
    println!("length {}", hello.len());

    // push char
    hello.push('W');

    // push string
    hello.push_str("orld!");

    // get capacity in bytes
    println!("capacity {}", hello.capacity());

    // is empty
    println!("is empty {}", hello.is_empty());

    // contains
    println!("contains world {}", hello.contains("World"));

    // replace
    println!("replaced {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // create string with capacity
    let mut max_string: String = String::with_capacity(1);
    max_string.push('a');
    max_string.push('b');
    println!("s: {}, c: {}", max_string, max_string.capacity());

    // assertion testing
    assert_eq!(2, max_string.len());
    assert_eq!(1, max_string.capacity());

}