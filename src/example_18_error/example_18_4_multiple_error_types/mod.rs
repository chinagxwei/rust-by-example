mod example_18_4_1_option_result;
mod example_18_4_2_multiple_error_types;
mod example_18_4_3_boxing_errors;
mod example_18_4_4_reenter_question_mark;
mod example_18_4_5_wrap_error;

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_4_multiple_error_types() {
        let numbers = vec!["42", "93", "18"];

//        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        println!("The first double is {}", double_first(numbers));

//        println!("The first double is {}", double_first(empty));

//        println!("The first double is {}", double_first(strings));
    }
}