pub mod example_1_1_comment;

pub mod example_1_2_print;

pub fn hello_example() {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        hello_example();
    }
}
