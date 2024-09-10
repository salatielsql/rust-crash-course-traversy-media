// https://doc.rust-lang.org/rust-by-example/primitives.html
//
// Scalar Types
//     Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
//     Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
//     Floating point: f32, f64
//     char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
//     bool either true or false
//     The unit type (), whose only possible value is an empty tuple: ()
//
// Compound Types
//     Arrays like [1, 2, 3]
//     Tuples like (1, true)


pub fn run() {
    // defaults to i32
    let x = 1;

    // defaults to f64
    let y = 2.5;

    let z: i64 = 4545454545;

    // find max size
    println!("max i32: {}", std::i32::MAX);
    println!("max i64: {}", std::i64::MAX);
    println!("max f64: {}", std::f64::MAX);

    // bolean
    let is_active = true;

    // get boolean from expression
    let is_greater = 5 > 10;

    // char
    let a1 = 'a';
    let face = '\u{1F600}'; // smile emoji

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}