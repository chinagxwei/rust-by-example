fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_15_4_7_lifetime_coercion() {
        let first = 2;
        println!("{:p}", &first);
        {
            let second = 3;

            println!("The product is {}", multiply(&first, &second));
            println!("{} is the first", choose_first(&first, &second));
            println!("{:p}", &choose_first(&first, &second));
        }

        println!("{:p}", &first);
    }
}