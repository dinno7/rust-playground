pub fn run(){
    println!("Hello this is from src/print.rs");
    println!("Number {}", 7);

    println!("My name is {1} and i am a {0}, i have {2} years old.","developer", "taha", 23);
    println!("My name is {name} and i am a {job}, i have {age} years old.",job="developer", name="taha", age=23);

    println!("-----------------------");
    let num = 7124;
    println!("Double {}\nBinary {:0>8b}\nHex 0x{:x}\nOctal {:o}", num, num , num, num);
    println!("-----------------------");

    // Debug trait
    println!("{:?}", (123, "hello", true, 3.4));
}
