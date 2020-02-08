use std::num::ParseIntError;
use std::fmt::{Formatter, Error};

#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    Parse(ParseIntError),
}

type Result<T> = std::result::Result<T, DoubleError>;

impl std::fmt::Display for DoubleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
            DoubleError::Parse(ref e) => e.fmt(f)
        }
    }
}

impl std::error::Error for DoubleError {
    fn description(&self) -> &str {
        match *self {
            DoubleError::EmptyVec => "empty vector not allowed",
            DoubleError::Parse(ref e) => e.description()
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            DoubleError::EmptyVec => None,
            DoubleError::Parse(ref e) => Some(e)
        }
    }
}

impl From<ParseIntError> for DoubleError {
    fn from(e: ParseIntError) -> DoubleError {
        DoubleError::Parse(e)
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(DoubleError::EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(r) => println!("The first doubled is {}", r),
        Err(e) => println!("Error: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_4_5_wrap_error() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}