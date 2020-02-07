mod example_9_2_1_capture;
mod example_9_2_2_input_parameters;
mod example_9_2_3_anonymity;
mod example_9_2_4_input_functions;
mod example_9_2_5_output_parameters;
mod example_9_2_6_closure_examples;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_9_2_closures() {
        fn function(i: i32) -> i32 { i + 1 }
        let closure_annotated = |x: i32| { x + 1 };
        let closure_inferred = |x| x + 1;

        let i = 1;

        println!("function {}", function(i));
        println!("closure_annotated: {}", closure_annotated(i));
        println!("closure_inferred: {}", closure_inferred(i));

        let one = || 1;
        println!("closure returning one: {}", one());
    }
}