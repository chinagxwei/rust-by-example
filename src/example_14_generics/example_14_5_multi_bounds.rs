use std::fmt::{Debug, Display};

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_prints_2<T>(t: &T) where T: Debug + Display {
    println!("2 Debug: `{:?}`", t);
    println!("2 Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}`", t);
    println!("u: `{:?}`", u)
}

fn example_14_5_multi_bounds() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    compare_prints_2(&string);
//    compare_prints(&array);

    compare_types(&array, &vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_14_5_multi_bounds() {
        example_14_5_multi_bounds();
    }
}