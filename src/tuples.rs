pub fn run() {
  let person: (&str, &str, i8) = ("Salatiel", "Brazil", 27);

  println!("{} is from {} and is {}", person.0, person.1, person.2);

  // MAX 12
  println!("{:?}", (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));
}