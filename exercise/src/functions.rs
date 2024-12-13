pub fn run() {
    greeting("Hello", "Taha");
    println!("{}", add(10, 20));

    // > Closure
    let offset = 5;
    let add_nums = |a: i32, b: i32| a + b + offset;
    println!("Closure sum {}", add_nums(23, 10));
}

fn greeting(greet: &str, name: &str) {
    println!("{greet} {name}, nice to meet you!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
