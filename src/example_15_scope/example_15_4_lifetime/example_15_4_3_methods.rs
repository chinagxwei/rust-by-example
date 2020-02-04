struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn example_15_4_3_methods() {
    let mut owner = Owner(10);

    owner.add_one();
    owner.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_15_4_3_methods() {
        example_15_4_3_methods();
    }
}