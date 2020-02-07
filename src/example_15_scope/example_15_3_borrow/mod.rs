mod example_15_3_1_mut;
mod example_15_3_2_freeze;
mod example_15_3_3_alias;
mod example_15_3_4_ref;

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains: {}", boxed_i32);
}

fn borrow_i32(borrowed: &i32) {
    println!("This int is: {}", borrowed);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_15_3_borrow() {
        let boxed_i32 = Box::new(5i32);
        let stacked_i32 = 6_i32;

        borrow_i32(&boxed_i32);
        borrow_i32(&stacked_i32);

        {
            let ref_to_i32 = &boxed_i32;
//        eat_box_i32(boxed_i32);
            borrow_i32(ref_to_i32);
        }

        eat_box_i32(boxed_i32);
    }
}