fn example_8_1_if_else() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n)
    }

    let big_n = if -10 < n && n < 10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, half the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_8_1_if_else() {
        example_8_1_if_else();
    }
}