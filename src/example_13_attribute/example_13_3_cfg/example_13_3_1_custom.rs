//#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn example_13_3_1_custom() {
    conditional_function();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_13_3_1_custom() {
        example_13_3_1_custom();
    }
}