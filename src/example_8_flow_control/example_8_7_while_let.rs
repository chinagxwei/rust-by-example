#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_8_7_while_let() {
        let mut optional = Some(0);

        while let Some(e) = optional {
            if e > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`e` is `{:?}`. Try again.", e);
                optional = Some(e + 1);
            }
        }
    }
}