fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn try_division(dividend: i32, divisor: i32) {
    match checked_division(dividend, divisor) {
        Some(quotient) => println!("{} / {} = {}", dividend, divisor, quotient),
        None => println!("{} / {} failed!", dividend, divisor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_19_4_option() {
        try_division(4, 2);
        try_division(1, 0);

        let none: Option<i32> = None;

        let equivalent_none = None::<i32>;

        let option_float = Some(0f32);

        println!("{:?} unwraps to {:?}", option_float, option_float.unwrap());

//        println!("{:?} unwraps to {:?}", none, none.unwrap());
    }
}