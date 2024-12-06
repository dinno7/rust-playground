use std::fmt::{self, Display, Formatter};

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn run() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
    println!("----------Transposed----------");
    println!("{}", transpose(matrix));
}

fn transpose(m: Matrix) -> Matrix {
    let Matrix(a, b, c, d) = m;
    Matrix(a, c, b, d)
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "( {}\t{} )\n( {}\t{} )", self.0, self.1, self.2, self.3)
    }
}
