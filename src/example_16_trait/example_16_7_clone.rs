#[derive(Copy, Clone, Debug)]
struct Nil;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn example_16_7_clone() {
    let nil = Nil;
    let copied_nil = nil;

    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // 只有Clone trait 的情况下，一般赋值行为还是值移动，原变量作废
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);
//    println!("original: {:?}", pair);

    let clone_pair = moved_pair.clone();
    drop(moved_pair);

    println!("clone: {:?}", clone_pair);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_16_7_clone() {
        example_16_7_clone();
    }
}