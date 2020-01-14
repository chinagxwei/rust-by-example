pub mod literals_example;
pub mod tuples_example;
pub mod array_example;

pub fn primitive_example() {
// 变量可以给出类型说明。
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 常规说明
    let an_integer = 5i32; // 后缀说明

    // 否则会按默认方式决定类型。
    let default_float = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    inferred_type = 4294967296i64;

    // 可变的（mutable）变量，其值可以改变。
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // 报错！变量的类型并不能改变。
//    mutable = true;

    // 但可以用掩蔽（shadow）来覆盖前面的变量。
    let mutable = true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive() {
        primitive_example();
    }
}