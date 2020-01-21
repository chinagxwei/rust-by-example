use std::marker::PhantomData;

mod example_14_9_1_testcase_units;

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

fn example_14_9_phantom() {
    let _tuple_1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple_2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);
    let _struct_1: PhantomStruct<char, f32> = PhantomStruct { first: 'Q', phantom: PhantomData };
    let _struct_2: PhantomStruct<char, f64> = PhantomStruct { first: 'Q', phantom: PhantomData };

    // 编译期错误！类型不匹配，所以这些值不能够比较：
//    println!("_tuple1 == _tuple2 yields: {}",
//             _tuple_1 == _tuple_2);

    // 编译期错误！类型不匹配，所以这些值不能够比较：
//    println!("_struct1 == _struct2 yields: {}",
//              _struct_1 == _struct_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_14_9_phantom() {
        example_14_9_phantom();
    }
}