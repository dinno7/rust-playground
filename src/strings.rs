use core::prelude;

pub fn run() {
    let my_str = "Hello there";
    println!("This is my str -> {}", my_str);

    let mut changeable_str = String::from("Hello ");
    changeable_str.push('W');
    changeable_str.push_str("orld!");

    println!("The length {}", changeable_str.len());
    println!("The length {}", my_str.len());

    println!("The capacity {}", changeable_str.capacity());

    println!(
        "Is my_str empty? {}",
        if my_str.is_empty() { "Yes" } else { "No" }
    );
    println!(
        "Is changeable_str empty? {}",
        if changeable_str.is_empty() {
            "Yes"
        } else {
            "No"
        }
    );

    println!("Is my_str contains World? {}", my_str.contains("World"));
    println!(
        "Is changeable_str contains World? {}",
        changeable_str.contains("World")
    );

    // > Replacing...
    println!(
        "\"there\" replaced by \"World!\" in my_str-> {}",
        my_str.replace("there", "World!")
    );

    // > loop over string by whitespaces:
    println!("----------------");
    for word in changeable_str.split_whitespace() {
        println!("words -> {}", word);
    }
    println!("-------With index---------");
    for (index, word) in changeable_str.split_whitespace().enumerate() {
        println!("word number {index} -> {}", word);
    }
    println!("----------------");

    // > String with capacity
    let mut cap_str = String::with_capacity(10);
    cap_str.push_str("hello");
    println!("str: {}, cap: {}",cap_str, cap_str.capacity());

    println!("my_str: {} , changeable_str: {}", my_str, changeable_str);
}
