use std::process::{Command, Stdio};
use std::error::Error;
use std::io::{Write, Read};

static PANGRAM: &'static str = "the quick brown fox jumped over the lazy dog \n";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_20_5_1_pipe() {
        let process = match Command::new("wca")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn() {
            Ok(process) => process,
            Err(e) => panic!("couldn't spawn wc: {}", e.description())
        };

        match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
            Ok(_) => println!("sent pangram to wc"),
            Err(e) => panic!("couldn't write to wc stdin: {}", e.description())
        }

        let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Ok(_) => println!("wc responded wiith:\n{}", s),
            Err(e) => panic!("couldn't read wc stdout: {}", e.description())
        }
    }
}