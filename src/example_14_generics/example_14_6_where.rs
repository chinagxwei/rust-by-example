use std::fmt::Debug;

///
/// ```
/// trait ATrait {}
/// trait BTrait {}
/// trait CTrait {}
/// trait MyTrait<A, D> {}
/// struct YourType;
///
/// impl<A: ATrait + BTrait, D: ATrait + CTrait> MyTrait<A, D> for YourType {}
///
/// impl<A, D> MyTrait<A, D> for YourType
///    where A: ATrait + BTrait,
///          D: ATrait + CTrait
/// {}
/// ```
///

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
    where Option<T>: Debug
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn example_14_6_where() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_14_6_where() {
        example_14_6_where();
    }
}