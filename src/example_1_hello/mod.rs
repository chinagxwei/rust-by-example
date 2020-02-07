mod example_1_1_comment;
mod example_1_2_print;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_hello() {
        println!("Hello World!");
    }
}
