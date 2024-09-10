// Variables hold primite data or references to data
// Variables are immutable by default (can't re-assign)
// Rust is a block-scoped language (a variable created within a func that variables exists only within that scope)

pub fn run() {
    let name = "Salatiel";
    // add keyword mut to indicate this variable can be mutated
    let mut age = 27;

    age = 28;

    println!("My name is {} and I'm {}", name, age);

    // consts
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Multiple assignment
    let (my_name, my_age) = ("Salatiel", 27);
    println!("My name is {} and I'm {}", my_name, my_age);
}