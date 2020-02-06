mod example_17_1_syntax;
mod example_17_2_dry;
mod example_17_3_dsl;
mod example_17_4_variadics;

/// block
/// expr 用于表达式
/// ident 用于变量名或函数名
/// item
/// pat (模式 pattern)
/// path
/// stmt (语句 statement)
/// tt (标记树 token tree)
/// ty (类型 type)

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn example_17_macros(){
    say_hello!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_17_macros() {
        example_17_macros();
    }
}