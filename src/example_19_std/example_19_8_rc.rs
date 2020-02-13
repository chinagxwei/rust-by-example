#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn example_19_8_rc() {
        let rc_example = "Rc example".to_string();

        {
            println!("--- rc_a is create ---");

            let rc_a = Rc::new(rc_example);
            println!("Reference Count of  rc_a: {}", Rc::strong_count(&rc_a));

            {
                println!("--- rc_a is cloned to rc_b ---");

                let rc_b = Rc::clone(&rc_a);

                println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
                println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

                println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

                println!("Length of the value inside rc_a: {}", rc_a.len());
                println!("Value of rc_b: {}", rc_b);

                println!("--- rc_b is dropped out of scope ---");
            }

            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            println!("--- rc_a is dropped out of scope ---");
        }

//        println!("rc_examples: {}", rc_examples);
    }
}