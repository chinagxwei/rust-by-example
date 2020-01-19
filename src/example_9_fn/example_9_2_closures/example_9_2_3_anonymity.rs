fn apply<F>(f: F) where F: FnOnce() {
    f();
}

fn example_9_2_3_anonymity() {
    let x = 7;

    let print = || println!("x is {}", x);

    apply(print);

//    println!("{}", x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_9_2_3_anonymity() {
        example_9_2_3_anonymity();
    }
}