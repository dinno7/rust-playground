use std::mem;
pub fn run() {
    let numbers: [i32; 7] = [1, 23, 4, 5, 6, 712, 74];
    println!("{:?}", numbers);

    // > Single value
    println!("{}", numbers[3]);

    // > Re-assign
    let mut numbers2: [i32; 5] = [1, 23, 4, 5, 6];
    numbers2[2] = 323;
    println!("{}", numbers2[2]);

    // > Array length
    println!("{}", numbers.len());

    // > Stack allocated
    println!(
        "The numbers array is occupied {} Bytes",
        mem::size_of_val(&numbers)
    );

    let slice: &[i32] = &numbers[2..5];
    println!("a slice of numbers {:?}", slice);

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];
    // println!("{:?}", ys);

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

}
