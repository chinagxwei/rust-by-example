mod example_17_1_syntax;
mod example_17_2_dry;
mod example_17_3_dsl;
mod example_17_4_variadics;

/// item: 条目，比如函数、结构体、模组等。
/// block: 区块(即由花括号包起的一些语句加上/或是一项表达式)。
/// stmt: 语句
/// pat: 模式
/// expr: 表达式
/// ty: 类型
/// ident: 标识符
/// path: 路径 (例如 foo, ::std::mem::replace, transmute::<_, int>, …)
/// meta: 元条目，即被包含在 #[…]及#![…]属性内的东西。
/// tt: 标记树

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_17_macros(){
        say_hello!();
    }
}
