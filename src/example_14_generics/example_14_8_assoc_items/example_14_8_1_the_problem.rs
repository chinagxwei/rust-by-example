struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, num_1: &i32, num_2: &i32) -> bool {
        (&self.0 == num_1) && (&self.1 == num_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<A, B, C>(container: &C) -> i32
    where C: Contains<A, B>
{
    container.last() - container.first()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_14_8_1_the_problem() {
        let num_1 = 3;
        let num_2 = 10;

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