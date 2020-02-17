mod example_20_5_1_pipe;
mod example_20_5_2_wait;


#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn example_20_5_process() {
        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            println!("rustc succeeded and stdout was:\n{}", s);
        } else {
            let s = String::from_utf8_lossy(&output.stderr);
            println!("rustc failed and stderr was:\n{}", s);
        }
    }
}