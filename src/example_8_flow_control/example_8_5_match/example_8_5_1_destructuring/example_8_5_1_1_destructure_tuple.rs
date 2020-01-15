fn example_8_5_1_1_destructure_tuple() {
    let pair = (0, -2);

    match pair {
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _ => println!("It doesn't matter what they are"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_8_5_1_1_destructure_tuple() {
        example_8_5_1_1_destructure_tuple();
    }
}