pub fn run() {
    // print line
    println!("Hello from print.rs file");

    // Basic formatting
    println!("Number: {}", 123);
    println!("{} is from {}", "Salatiel", "Brazil");

    // Positional parameters
    println!("{0} is from {1} and {0} likes to {2}", "Salatiel", "Brazil", "code");

    // Named arguments
    println!("{name} likes to play {activity}", name = "Salatiel", activity = "videogame");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "Hey!"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}