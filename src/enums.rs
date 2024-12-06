enum Movements {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movements) {
    match m {
        Movements::Up => println!("The avatar moved Up"),
        Movements::Down => println!("The avatar moved Down"),
        Movements::Left => println!("The avatar moved Left"),
        Movements::Right => println!("The avatar moved Right"),
    }
}
pub fn run() {
    move_avatar(Movements::Up);
    move_avatar(Movements::Right);
}
