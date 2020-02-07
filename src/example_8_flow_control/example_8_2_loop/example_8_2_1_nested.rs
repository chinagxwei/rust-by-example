#![allow(unreachable_code)]


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_8_2_1_nested() {
        'outer: loop {
            println!("Entered the outer loop");
            'inner: loop {
                println!("Entered the inner loop");

                // the break is stop `'inner` loop
//            break;

                // the break is stop `'outer` loop
                break 'outer;
            }
            println!("This point will never b reached");
        }
        println!("Exited the outer loop")
    }
}