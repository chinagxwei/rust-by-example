struct S;

struct GenericVal<T>(T);

impl GenericVal<f32> {}

impl GenericVal<S> {}

impl<T> GenericVal<T> {}

struct Val { val: f64 }

struct GenVal<T> { gen_val: T }

impl Val {
    fn value(&self) -> &f64 { &self.val }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T { &self.gen_val }
}

fn example_14_2_impl() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    let z = GenVal::<char> { gen_val: 'a' };

    println!("{}, {}, {}", x.value(), y.value(), z.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_14_2_impl() {
        example_14_2_impl();
    }
}