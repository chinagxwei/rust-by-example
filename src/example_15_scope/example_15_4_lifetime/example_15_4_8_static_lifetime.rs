static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_15_4_8_static_lifetime() {
        {
            let static_string = "I'm in read-only memory";
            println!("static_string is: {}", static_string);
        }

        {
            let lifetime_num = 5;

            let coerce_static = coerce_static(&lifetime_num);

            println!("coerce_static is: {}", coerce_static);
        }

        println!("NUM: {} stays accessible!", NUM);
    }
}