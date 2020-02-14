static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs::File;
    use std::error::Error;
    use std::io::Write;

    #[test]
    fn example_20_4_2_create() {
        let path = Path::new("out/lorem_ipsum.txt");
        let display = path.display();

        // 报错请在项目目录创建 out 文件夹
        let mut file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => panic!("couldn't create {}: {}", display, e.description())
        };

        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Ok(_) => println!("successfully wrote to {}", display),
            Err(e) => panic!("couldn't write to {}: {}", display, e.description())
        }
    }
}