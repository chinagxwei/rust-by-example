use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
pub struct Matrix(f32, f32, f32, f32);

pub fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

pub fn transpose(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({},{})\n({},{})", self.0, self.1, self.2, self.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuples() {
        let long_tuple = (
            1u8,
            2u16,
            3u32,
            4u64,
            -1i8,
            -2i16,
            -3i32,
            -4i64,
            0.1f32,
            0.2f64,
            'a',
            true
        );

        println!("long tuple first value :{}", long_tuple.0);
        println!("long tuple second value :{}", long_tuple.1);

        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

        println!("tuple of tuples:{:?}", tuple_of_tuples);

        let pair = (1, true);
        println!("pair is {:?}", pair);
        println!("the reversed pair is {:?}", reverse(pair));

        println!("one element tuple: {:?}", (5u32, ));
        println!("just an integer: {:?}", (5u32));

        let tuple = (1, "example_1_hello", 4.5, true);

        let (a, b, c, d) = tuple;

        println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{:?}", matrix);
        println!("Matrix:\n{}", matrix);
        println!("Transpose:\n{}", transpose(matrix));
    }
}