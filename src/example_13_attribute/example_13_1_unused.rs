fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_13_1_unused() {
        used_function();
    }
}