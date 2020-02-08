fn give_princess(gift: &str) {
    if gift == "snake" { panic!("AAAaaaaaa!!!!!!") }

    println!("I love {}s!!!!!!", gift);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_18_1_panic() {
        give_princess("teddy bear");
//    give_princess("snake");
    }
}