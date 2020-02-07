#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_5_3_inference() {
        let elem = 5u8;

        let mut vec = Vec::new();
        vec.push(elem);
        println!("{:?}", vec);
    }
}