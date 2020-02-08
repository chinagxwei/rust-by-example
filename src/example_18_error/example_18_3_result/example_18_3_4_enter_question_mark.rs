use std::num::ParseIntError;

fn multiply_v1(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn multiply_2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = r#try!(first_number_str.parse::<i32>());
    let second_number = r#try!(second_number_str.parse::<i32>());

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_3_4_enter_question_mark() {
        print(multiply_v1("10", "2"));
        print(multiply_2("t", "2"));
    }
}