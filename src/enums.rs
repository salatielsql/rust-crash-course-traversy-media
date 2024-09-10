// enums are types which have a few definite values

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Animal {
    Dog(String),
    Cat { name: String, weight: f64 },
}

fn move_player(d: Direction) {
    match d {
        Direction::Up => println!("Move player {:?}", d),
        Direction::Down => println!("Move player {:?}", d),
        Direction::Left => println!("Move player {:?}", d),
        Direction::Right => println!("Move player {:?}", d),
    }
}

pub fn run() {
    move_player(Direction::Up);
    move_player(Direction::Down);
    move_player(Direction::Left);
    move_player(Direction::Right);

    let dog = Animal::Dog(String::from("Aslan"));
    let cat = Animal::Cat {
        name: String::from("Lorelai"),
        weight: 6.1,
    };
    print!("My dog is {:?} and my cat is {:?}", dog, cat);
}
