#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_4_1_mut() {
        let _immutable_binding = 1;
        let mut mutable_binding = 1;

        println!("Before mutation: {}", mutable_binding);

        mutable_binding += 1;

        println!("After mutation: {}", mutable_binding);

//    _immutable_binding += 1;
    }
}