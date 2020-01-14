pub static LANGUAGE: &'static str = "Rust";
pub const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_example() {
        let n = 16;

        println!("This is {}", LANGUAGE);
        println!("The Threshold is {}", THRESHOLD);
        println!("{} is {}", n, if is_big(n) { "big" } else { "small" })
    }
}