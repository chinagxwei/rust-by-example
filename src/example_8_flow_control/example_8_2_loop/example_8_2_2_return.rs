fn example_8_2_2_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_8_2_2_return() {
        example_8_2_2_return();
    }
}