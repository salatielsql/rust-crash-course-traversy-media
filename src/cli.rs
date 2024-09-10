use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("debug args: {:?}", args);

    if command == "ping" {
        println!("pong")
    } else {
        println!("'{}' is not a valid command", command)
    }
}
