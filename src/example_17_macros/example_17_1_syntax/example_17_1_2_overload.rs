macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    }
}


fn example_17_1_2_overload(){
    test!((1i32 + 1 == 2i32); and 2i32 * 2 == 4i32);
    test!(true; or false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_17_1_2_overload() {
        example_17_1_2_overload();
    }
}