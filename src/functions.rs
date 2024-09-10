pub fn run() {
    greeting("Hello", "Salatiel");
    
    // bind function values to variables
    let sum = add(15, 45);
    println!("{}", sum);

    // closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("{}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{}! {}", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}