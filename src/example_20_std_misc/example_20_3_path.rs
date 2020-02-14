#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn example_20_3_path() {
        let path = Path::new(".");

        let display = path.display();

        let new_path = path.join("a").join("b");

        match new_path.to_str() {
            Some(v) => println!("new path is {}", v),
            None => panic!("new path is not a valid UTF-8 sequence")
        }
    }
}