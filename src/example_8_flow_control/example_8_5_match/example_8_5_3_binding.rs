fn get_age() -> i32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn example_8_5_2_guard_1() {
    println!("Tell me type of person you are");

    match get_age() {
        0 => println!("I;m not born yet I guess"),
        n @ 1...12 => println!("I'm a child of age {:?}", n),
        n @ 13...19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an ole person of age {:?}", n)
    }
}

fn example_8_5_2_guard_2() {
    match some_number() {
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_8_5_2_guard() {
        example_8_5_2_guard_1();
        example_8_5_2_guard_2();
    }
}