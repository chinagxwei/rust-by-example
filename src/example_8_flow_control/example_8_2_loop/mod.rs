mod example_8_2_1_nested;
mod example_8_2_2_return;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_8_2_loop() {
        let mut count = 0u32;
        println!("Let's count until infinity!");
        loop {
            count += 1;
            if count == 3 {
                println!("three");
                continue;
            }
            println!("{}", count);
            if count == 5 {
                println!("OK, that's enough");
                break;
            }
        }
    }
}