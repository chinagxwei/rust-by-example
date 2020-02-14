use std::fs::File;
use std::path::Path;
use std::io::BufRead;

type IOResult = std::io::Result<std::io::Lines<std::io::BufReader<File>>>;

fn read_lines<P>(filename: P) -> IOResult where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(std::io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_20_4_3_read_lines() {
        if let Ok(result) = read_lines("./hosts.toml") {
            for line in result {
                if let Ok(ip) = line {
                    println!("{}", ip);
                }
            }
        }
    }
}