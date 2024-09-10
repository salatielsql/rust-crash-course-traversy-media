// structs - used to create custom data types

struct Color {
    r: u8,
    g: u8,
    b: u8
}

// tuple struct
struct PrintableColor (u8, u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
    age: Option<u8>,
}

impl Person {
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age: Default::default()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn name_to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut red = Color {
        r: 255,
        g: 0,
        b: 0,
    };
    red.b = 1;

    // Color
    println!("red (rgb): ({}, {}, {})", red.r, red.g, red.b);
    println!("red (hex): ({:x}{:x}{:x})", red.r, red.g, red.b);

    // PrintableColor
    let mut black = PrintableColor(0, 0, 0, 100);
    println!("black (CMYK): ({}, {}, {}, {})", black.0, black.1, black.2, black.3);

    black.0 = 100;
    black.1 = 100;
    black.2 = 100;
    println!("full black (CMYK): ({}, {}, {}, {})", black.0, black.1, black.2, black.3);

    // Person
    let mut p = Person::new("John", "Do");

    println!("person {}, {}, {:?}, fullname: {}", p.first_name, p.last_name, p.age, p.full_name());

    p.set_last_name("Scott");
    println!("updated: {}", p.full_name());
    println!("full name tuples {:?}", p.name_to_tuple());
}