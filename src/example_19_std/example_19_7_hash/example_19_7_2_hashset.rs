#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn example_19_7_2_hashset() {
        let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
        let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

        assert!(a.insert(4));
        assert!(a.contains(&4));

//        assert!(b.insert(4),"Value 4 is already in set B!");

        b.insert(5);

        println!("A: {:?}", a);
        println!("B: {:?}", b);

        println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

        // a 集合中与 b 不相同的部分
        println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

        // a 、b 集合中相同的部分
        println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

        // a 、b 集合中不相同的部分
        println!("Symmetric Difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
    }
}