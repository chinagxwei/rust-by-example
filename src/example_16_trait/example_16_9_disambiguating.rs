trait UsernameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct From {
    username: String,
    age: u8,
}

impl UsernameWidget for From {
    fn get(&self) -> String {
        println!("use UsernameWidget `get`");
        self.username.to_owned()
    }
}

impl AgeWidget for From {
    fn get(&self) -> u8 {
        println!("use AgeWidget `get`");
        self.age
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_16_9_disambiguating() {
        let from = From { username: "wayne".to_string(), age: 24 };

        // 报错，不知道要使用哪个 get 方法
//    println!("{}",from.get());

        let username = <From as UsernameWidget>::get(&from);
        assert_eq!("wayne".to_owned(), username);
        let age = <From as AgeWidget>::get(&from);
        assert_eq!(age, 24);
    }
}