macro_rules! find_min {
    ($x:expr) => {$x};
    ($x:expr, $($y:expr),+) => {
        std::cmp::min($x, find_min!($($y),+))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_17_1_3_repeat() {
        println!("{}", find_min!(1u32));
        println!("{}", find_min!(1u32 + 2, 4u32));
        println!("{}", find_min!(5u32, 2u32 * 3, 7u32));
    }
}