fn example_15_2_1_mut() {
    let immutable_box = Box::new(5u32);
    println!("immutable_box contains: {}", immutable_box);
    let mut mutable_box = immutable_box;
    println!("mutable_box contains: {}", mutable_box);
    *mutable_box = 4;
    println!("mutable_box now contains: {}", mutable_box);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_15_2_1_mut() {
        example_15_2_1_mut();
    }
}