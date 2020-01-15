fn example_8_3_while() {
// 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时循环
    while n < 51 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加 1
        n += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_8_3_while() {
        example_8_3_while();
    }
}