#[derive(Debug)]
struct Point { x: i32, y: i32, z: i32 }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_15_3_3_alias() {
        let mut point = Point {
            x: 1,
            y: 2,
            z: 3,
        };

        {
            let borrowed_point = &point;
            let another_borrow = &point;

            //可变状态被冻结
//        let mutable_borrow = &mut point;

            println!("Point has coordinates: ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);
        }//离开块后可变状态恢复

        {
            let mutable_borrow = &mut point;
            mutable_borrow.x = 4;
            mutable_borrow.y = 5;
            mutable_borrow.z = 6;

            println!("Point has coordinates: ({}, {}, {})", mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);
        }

        let borrowed_point = &point;

        println!("Point now has coordinates: ({}, {}, {})", borrowed_point.x, borrowed_point.y, borrowed_point.z);
    }
}