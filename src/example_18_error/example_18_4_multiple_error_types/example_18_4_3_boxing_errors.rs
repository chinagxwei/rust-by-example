use std::fmt::{Formatter, Error};

#[derive(Clone, Debug)]
struct EmptyVec;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl std::fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl std::error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|x| {
            x.parse::<i32>()
                .map_err(|x| x.into())
                .map(|x| 2 * x)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(r) => println!("The first double is {}", r),
        Err(e) => println!("Error: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_4_3_boxing_errors() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}