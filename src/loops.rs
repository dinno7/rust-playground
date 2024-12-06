pub fn run() {
    let mut count = 1;
    loop {
        println!("count value: {}", count);
        if count >= 20 {
            break;
        }
        count += 1;
    }

    println!("The loop done");

    println!("-----------FizzBuzz-----------");
    let mut count = 1;
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }
        count += 1;
    }

    // > For range loop
    for x in 0..101 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}
