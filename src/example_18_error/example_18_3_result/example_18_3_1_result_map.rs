use std::num::ParseIntError;

fn multiply_v1(first_number_str: &str, second_number_str: &str)
               -> Result<i32, ParseIntError>
{
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e)
        },
        Err(e) => Err(e)
    }
}

fn multiply_v2(first_number_str: &str, second_number_str: &str)
               -> Result<i32, ParseIntError>
{
    first_number_str.parse::<i32>()
        .and_then(|first_number|
            second_number_str.parse::<i32>()
                .map(|second_number| first_number * second_number)
        )
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(i) => println!("i is {}", i),
        Err(e) => println!("Error: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_3_1_result_map() {
        let twenty = multiply_v1("2", "10");

        print(twenty);

//    let tt = multiply_v1("t","10");
        let tt = multiply_v2("t", "10");

        print(tt);
    }
}