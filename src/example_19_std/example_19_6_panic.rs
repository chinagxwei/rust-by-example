fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("division by zero")
    } else {
        dividend / divisor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_19_6_panic() {
        let x = Box::new(0i32);

        division(3, 1);

        println!("This point won't be reached!")
    }
}