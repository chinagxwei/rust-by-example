#[derive(Debug)]
struct Borrowed<'a> { x: &'a i32 }

impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_15_4_5_trait() {
        let b: Borrowed = Default::default();
        println!("b is {:?}", b);
    }
}