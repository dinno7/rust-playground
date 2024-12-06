use std::fmt::{self, Formatter, Result};

// > Traditional structs
// #[derive(Debug)]
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }
// > Tuple structs
#[derive(Debug)]
struct RGBColor(i32, i32, i32);
impl fmt::Display for RGBColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "R({})\tG({})\tB({})", self.0, self.1, self.2)
    }
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: i32,
}

impl Person {
    fn new(first: &str, last: &str, age: i32) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
            age,
        }
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, last: &str) {
        self.first_name = last.to_string();
    }

    fn name_2_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut color = RGBColor(255, 0, 0);
    color.2 = 121;
    println!("{}", color);

    let mut person = Person::new("Dinno", "Dlrb", 23);
    println!("{:?}", person);
    person.set_first_name("Taha");
    println!("{}", person.get_full_name());
    println!("{:?}", person.name_2_tuple());
}
