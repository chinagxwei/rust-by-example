fn apply<F>(f: F) where F: FnOnce() {
    f();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_9_2_3_anonymity() {
        let x = 7;

        let print = || println!("x is {}", x);

        apply(print);

//    println!("{}", x);
    }
}