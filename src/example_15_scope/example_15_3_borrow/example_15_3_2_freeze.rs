fn example_15_3_2_freeze() {
    let mut mutable = 7i32;

    {
        let large_integer = &mutable;

//        mutable = 50;

        println!("Immutably borrowed {}", large_integer);
    }

    mutable = 3;

    println!("mutably {}", mutable);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_15_3_2_freeze() {
        example_15_3_2_freeze();
    }
}