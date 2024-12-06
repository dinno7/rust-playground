pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);


    let slice: &[i32] = &numbers[1..4];
    println!("Slice of vector {:?}", slice);

    println!("Index of 2 is {}", numbers[2]);

    println!("-------------------------------");
    for (index, value) in numbers.iter().enumerate() {
        println!("Index -> {index} | value -> {value}");
    }

    // >>  Loop and mutate
    for value in numbers.iter_mut() {
        *value *= *value;
    }

    println!("-------------------------------");

    for (index, value) in numbers.iter().enumerate() {
        println!("Index -> {index} | value -> {value}");
    }

}
