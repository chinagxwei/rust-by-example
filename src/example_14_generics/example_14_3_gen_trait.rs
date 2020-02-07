struct Empty;

struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_14_3_gen_trait() {
        let empty = Empty;
        let null = Null;
        empty.double_drop(null);

//    empty;
//    null;
    }
}