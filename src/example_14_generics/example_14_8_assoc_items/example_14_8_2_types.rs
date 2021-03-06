struct Container(i32, i32);

trait Contains {
    type A;
    type B;
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, num_1: &Self::A, num_2: &Self::B) -> bool {
        (&self.0 == num_1) && (&self.1 == num_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_14_8_2_types() {
        let num_1 = 30;
        let num_2 = 100;

        let container = Container(num_1, num_2);
        println!(
            "Does container contain {} and {}: {}",
            &num_1,
            &num_2,
            container.contains(&num_1, &num_2)
        );
        println!("First number: {}", container.first());
        println!("Last number: {}", container.last());

        println!("The difference is: {}", difference(&container));
    }
}