mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_10_2_struct_visibility() {
        let open_box = my::OpenBox { contents: "public information" };

        println!("The open box contains: {}", open_box.contents);

        // 带有私有字段的公有结构体不能使用字段名来构造。
        // 报错！`ClosedBox` 含有私有字段。
//    let closed_box = my::ClosedBox { contents: "classified information" };
        // 试一试 ^ 取消此行注释

        let _closed_box = my::ClosedBox::new("classified information");

        // 并且一个结构体中的私有字段不能访问到。
        // 报错！`content` 字段是私有的。
        //println!("The closed box contains: {}", _closed_box.contents);
        // 试一试 ^ 取消此行注释
    }
}