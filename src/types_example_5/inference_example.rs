fn inference_example() {
    let elem = 5u8;

    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inference_example() {
        inference_example();
    }
}