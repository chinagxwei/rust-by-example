fn example_8_5_2_guard() {
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("Thede are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd."),
        _ => println!("No correlation..."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_8_5_2_guard() {
        example_8_5_2_guard();
    }
}