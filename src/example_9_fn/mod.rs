mod example_9_1_methods;
mod example_9_2_closures;
mod example_9_3_hof;
mod example_9_4_diverging;

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for i in 1..(n + 1) {
        fizzbuzz(i);
    }
}

fn example_9_fn(){
    fizzbuzz_to(51);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_9_fn() {
        example_9_fn();
    }
}