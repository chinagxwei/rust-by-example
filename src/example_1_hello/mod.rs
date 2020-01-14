pub mod comment_example;

pub mod print_example;

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
