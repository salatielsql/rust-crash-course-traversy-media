pub fn run() {
    let mut count_me: u8 = 0;

    // infinite loop
    loop {
        count_me += 1;
        println!("count is on: {}", count_me);
        if count_me == 10 {
            break;
        }        
    }

    // while loop
    let mut count = 0;
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz")
        } else if count % 3 == 0 {
            println!("fizz")
        } else if count % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count)
        }

        count += 1;
    }

    // range loop
    for n in 0..10 {
        println!("range {}", n)
    }
}