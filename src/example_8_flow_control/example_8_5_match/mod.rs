mod example_8_5_1_destructuring;
mod example_8_5_2_guard;
mod example_8_5_3_binding;

fn example_8_5_1_destructuring() {
    let number = 13;
    println!("Tell me about {}", number);

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime."),
        b @ 13..=19 => println!("A teen. `{}`", b),
        c @ _ => println!("Ain't special. `{}`", c)
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1
    };

    println!("{} -> {}", boolean, binary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_8_5_1_destructuring() {
        example_8_5_1_destructuring();
    }
}