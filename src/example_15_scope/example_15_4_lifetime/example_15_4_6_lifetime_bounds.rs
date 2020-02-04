use std::fmt::Debug;

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print<T>(t: T) where T: Debug {
    println!("`print`: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T) where T: Debug + 'a {
    println!("`print_ref`: t is {:?}", t);
}

fn example_15_4_6_lifetime_bounds() {
    let x = 5;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_15_4_6_lifetime_bounds() {
        example_15_4_6_lifetime_bounds();
    }
}