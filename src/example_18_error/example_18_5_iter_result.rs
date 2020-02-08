fn use_iter_map() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|x| x.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}

fn use_filter_map() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|x| x.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    println!("Results: {:?}", numbers)
}

fn use_collect() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|x| x.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);
}

fn use_partition_v1() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|x| x.parse::<i32>())
        .partition(Result::is_ok);
    println!("Results: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

fn use_partition_v2() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|x| x.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Results: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_5_iter_result() {
        use_iter_map();
        use_filter_map();
        use_collect();
        use_partition_v1();
        use_partition_v2();
    }
}