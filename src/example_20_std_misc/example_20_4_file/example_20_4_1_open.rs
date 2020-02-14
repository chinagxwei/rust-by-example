#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs::File;
    use std::io::Read;
    use std::error::Error;

    #[test]
    fn example_20_4_1_open() {
        let path = Path::new("hello.txt");

        let display = path.display();

        let mut file = match File::open(&path) {
            Ok(v) => v,
            Err(e) => panic!("couldn't open {}: {}", display, e.description())
        };

        let mut s = String::new();

        match file.read_to_string(&mut s) {
            Ok(_) => println!("{} contains:\n{}", display, s),
            Err(e) => panic!("couldn't read {}: {}", display, e.description())
        }
    }
}