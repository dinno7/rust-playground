// Values of different types
// Max is 12 elements
pub fn run() {
    let person: (&str, &str, i16) = ("Taha", "developer", 23);

    println!(
        "Person name is {} and his job is {} and he is {} years old.",
        person.0, person.1, person.2
    );
    println!("The person tuple is: {:?}", person);

    // > But long Tuples (more than 12 elements) cannot be printed.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);
    // Uncomment the above 2 lines to see the compiler error

    // > Tuples can be destructured to create bindings.
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{} {} {} {}", a, b, c, d);
}

pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_pair, bool_pair) = pair;

    (bool_pair, int_pair)
}
