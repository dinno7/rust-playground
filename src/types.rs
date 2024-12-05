/*
Primitive types:
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    Floats: f32, f64
    Boolean: bool
    Characters: char
    Tuples: (... , ...)
    Arrays

*/
pub fn run() {
    // The constant vars must have type explicitly
    const APP_NAME : &str = "Dinno-App";
    println!("{}", APP_NAME);

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);


    let boolean: bool = true;
    println!("{:?}", boolean);
}
