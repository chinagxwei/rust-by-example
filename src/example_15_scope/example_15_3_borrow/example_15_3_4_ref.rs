#[derive(Copy, Clone, Debug)]
struct Point { x: i32, y: i32 }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_15_3_4_ref() {
        let c = 'Q';

        let ref ref_c1 = c;
        let ref_c2 = &c;

        println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

        let point = Point { x: 1, y: 2 };

        let copy_of_x = {
            let Point { x: ref ref_to_x, y: _ } = point;
            *ref_to_x
        };

        let mut mutable_point = point;

        {
            let Point { x: _, y: ref mut ref_to_y } = mutable_point;

            *ref_to_y = 3;
        }

        println!("point is ({:?})", point);

        println!("mutable_point is ({:?})", mutable_point);

        let mut mutable_tuple = (Box::new(3u32), 5u32);

        {
            let (_, ref mut last) = mutable_tuple;

            *last = 7u32;
        }

        println!("tuple is {:?}", mutable_tuple);
    }
}