struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_6_2_to_string_and_from_str() {
        let circle = Circle { radius: 6 };
        println!("{}", circle.to_string());

        let parsed: i32 = "5".parse().unwrap();
        let turbo_parsed = "10".parse::<i32>().unwrap();
        let sum = parsed + turbo_parsed;
        println!("Sum: {:?}", sum);
    }
}