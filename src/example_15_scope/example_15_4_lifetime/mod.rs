mod example_15_4_1_explicit;
mod example_15_4_2_fn;
mod example_15_4_3_methods;
mod example_15_4_4_struct;
mod example_15_4_5_trait;
mod example_15_4_6_lifetime_bounds;
mod example_15_4_7_lifetime_coercion;
mod example_15_4_8_static_lifetime;
mod example_15_4_9_elision;

fn example_15_4_lifetime() {

    // i 生命周期开始
    let i = 3;

    {
        // borrow_1 生命周期开始
        let borrow_1 = &i;

        println!("borrow_1: {}", borrow_1);

        // borrow_1 生命周期结束
    }

    {
        // borrow_2 生命周期开始
        let borrow_2 = &i;

        println!("borrow_2: {}", borrow_2);

        // borrow_2 生命周期结束
    }

    // i 生命周期结束
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_15_4_lifetime() {
        example_15_4_lifetime();
    }
}