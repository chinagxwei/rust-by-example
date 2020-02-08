mod example_18_3_1_result_map;
mod example_18_3_2_result_alias;
mod example_18_3_3_early_returns;
mod example_18_3_4_enter_question_mark;

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();

    first_number * second_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_3_result() {
        let twenty = multiply("2", "10");
        println!("double is {}", twenty);

//    let tt = multiply("tt", "10");
//    println!("double is {}", tt);
    }
}