
fn used_function(){}

#[allow(dead_code)]
fn unused_function(){}

fn noisy_unused_function(){}

fn example_13_1_unused(){
    used_function();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_13_1_unused() {
        example_13_1_unused();
    }
}