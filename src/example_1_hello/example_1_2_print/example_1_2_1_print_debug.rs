
#[derive(Debug)]
pub struct Structure(i32);

#[derive(Debug)]
pub struct Deep(Structure);

#[derive(Debug)]
pub struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        // 使用 `{:?}` 打印和使用 `{}` 类似。
        println!("{:?} months in a year.", 12);
        println!("{1:?} {0:?} is the {actor:?} name.",
                 "Slater",
                 "Christian",
                 actor = "actor's");

        // `Structure` 也可以打印！
        println!("Now {:?} will example_1_hello.example_1_2_print!", Structure(3));

        // 使用 `derive` 的一个问题是不能控制输出的形式。
        // 假如我只想展示一个 `7` 怎么办？
        println!("Now {:?} will example_1_hello.example_1_2_print!", Deep(Structure(7)));

        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };

        println!("{:#?}", peter)
    }
}