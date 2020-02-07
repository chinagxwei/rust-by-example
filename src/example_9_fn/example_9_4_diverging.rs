fn foo() -> ! {
    panic!("This call never return.");
}

fn some_fn() {
    ()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_9_4_diverging() {
        let a = some_fn();

        println!("This function returns and you can see this line.");

//    let b = foo();
//
//    println!("You will never see this line!");

        fn sum_odd_numbers(up_to: u32) -> u32 {
            let mut acc = 0;
            for i in 0..up_to {
                let addition = match i % 2 == 1 {
                    true => i,
                    false => continue,
                };
                println!("{}", i);
                acc += addition;
            }
            acc
        }
        println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
    }
}