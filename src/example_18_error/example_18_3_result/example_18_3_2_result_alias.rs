use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>()
        .and_then(|first_number|
            second_number_str.parse::<i32>()
                .map(|second_number| first_number * second_number)
        )
}

fn print(result: AliasedResult<i32>) {
    match result {
        Ok(i) => println!("i is {}", i),
        Err(e) => println!("Error: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_3_2_result_alias() {
        print(multiply("2", "10"));
        print(multiply("tt", "10"));
    }
}