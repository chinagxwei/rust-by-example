use std::num::ParseIntError;

fn double_first_v1(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first()
        .map(|first| {
            first.parse::<i32>()
                .map(|x| 2 * x)
        })
}

fn double_first_v2(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    vec.first()
        .map(|first| {
            first.parse::<i32>()
                .map(|x| 2 * x)
        })
        .map_or(Ok(None), |x| x.map(Some))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_4_1_option_result() {
        let numbers = vec!["42", "93", "18"];

        let empty = vec![];

        let strings = vec!["tofu", "93", "18"];

        println!("The first double is {:?}", double_first_v1(numbers.to_owned()));

        println!("The first double is {:?}", double_first_v1(empty.to_owned()));

        println!("The first double is {:?}", double_first_v1(strings.to_owned()));

        println!("The first double is {:?}", double_first_v2(numbers));

        println!("The first double is {:?}", double_first_v2(empty));

        println!("The first double is {:?}", double_first_v2(strings));
    }
}