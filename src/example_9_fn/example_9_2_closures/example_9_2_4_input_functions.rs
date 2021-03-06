fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_9_2_4_input_functions() {
        let closure = || println!("I'm a closure!");

        call_me(closure);
        call_me(function);
    }
}