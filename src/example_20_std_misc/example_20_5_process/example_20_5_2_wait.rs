#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn example_20_5_2_wait() {
        let mut child = Command::new("sleep")
            .arg("5")
            .spawn()
            .unwrap();
        let result = child.wait().unwrap();

        println!("reached end of main");
    }
}