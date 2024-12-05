// Values of different types
// Max is 12 elements
pub fn run() {
    let person: (&str, &str, i16) = ("Taha", "developer", 23);

    println!(
        "Person name is {} and his job is {} and he is {} years old.",
        person.0, person.1, person.2
    );
    println!("The person tuple is: {:?}", person);
}
