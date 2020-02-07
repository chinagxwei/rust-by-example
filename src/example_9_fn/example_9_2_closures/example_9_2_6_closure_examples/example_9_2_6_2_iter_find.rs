#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_9_2_6_2_iter_find() {
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];

        let mut iter1 = vec1.iter();
        let mut into_iter1 = vec2.into_iter();

        println!("Find 2 in vec1: {:?}", iter1.find(|&&x| { x == 2 }));
        println!("Find 2 in vec2: {:?}", into_iter1.find(|&x| { x == 2 }));

        let array1 = [1, 2, 3];
        let array2 = [4, 5, 6];

        let mut iter2 = array1.iter();
        let mut into_iter2 = array2.into_iter();

        println!("Find 2 in array1: {:?}", iter2.find(|&&x| { x == 2 }));
        println!("Find 2 in array2: {:?}", into_iter2.find(|&&x| { x == 2 }))
    }
}