fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let x = 12;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_15_4_1_explicit() {
        let (four, nine) = (4, 9);

        print_refs(&four, &nine);

        failed_borrow();
    }
}