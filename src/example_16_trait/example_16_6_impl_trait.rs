use std::iter::{Cycle, Chain};
use std::vec::IntoIter;


fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| { x + y }
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item=i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}

// 复杂的返回类型描述
fn combine_vecs_explicit_return_type<'a>(
    v: Vec<i32>, u: Vec<i32>,
)
    -> Cycle<Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 简单返回类型描述
fn combine_vecs<'a>(
    v: Vec<i32>, u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn example_16_6_impl_trait() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    let numbers = vec![1, 2, 3];
    println!("numbers before {:?}", &numbers);
    let mut iter = double_positives(&numbers);
    println!("numbers after {:?}", &numbers);
    println!("iter is {:?}", iter.next());
    println!("iter is {:?}", iter.next());
    println!("iter is {:?}", iter.next());
    println!("iter is {:?}", iter.next());

    let v_1 = vec![1i32, 2];
    let v_2 = vec![3i32, 4];
    let mut cycle_vec = combine_vecs_explicit_return_type(v_1, v_2);

    println!("complicated is {:?}", cycle_vec.next());
    println!("complicated is {:?}", cycle_vec.next());
    println!("complicated is {:?}", cycle_vec.next());
    println!("complicated is {:?}", cycle_vec.next());
    println!("complicated is {:?}", cycle_vec.next());

    let v_1 = vec![1i32, 2];
    let v_2 = vec![3i32, 4];
    let mut cycle_vec = combine_vecs(v_1, v_2);

    println!("simple is {:?}", cycle_vec.next());
    println!("simple is {:?}", cycle_vec.next());
    println!("simple is {:?}", cycle_vec.next());
    println!("simple is {:?}", cycle_vec.next());
    println!("simple is {:?}", cycle_vec.next());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_16_6_impl_trait() {
        example_16_6_impl_trait();
    }
}