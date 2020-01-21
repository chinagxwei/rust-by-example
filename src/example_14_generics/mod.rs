mod example_14_1_gen_fn;
mod example_14_2_impl;
mod example_14_3_gen_trait;
mod example_14_4_bounds;
mod example_14_5_multi_bounds;
mod example_14_6_where;
mod example_14_7_new_types;
mod example_14_8_assoc_items;
mod example_14_9_phantom;

struct A;

struct Single(A);

struct SingleGen<T>(T);

fn foo<T>(arg: T) -> T { arg }

fn example_14_generics() {
    let s = Single(A);

    let char: SingleGen<char> = SingleGen('a');

    let t = SingleGen(A);
    let is_i32 = SingleGen(6);
    let is_char = SingleGen('a');

    println!("foo is: {}", foo::<&str>("str"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_14_generics() {
        example_14_generics();
    }
}