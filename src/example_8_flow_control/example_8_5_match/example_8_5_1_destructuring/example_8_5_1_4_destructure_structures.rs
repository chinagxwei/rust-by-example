struct Foo {
    x: (u32, u32),
    y: u32,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_8_5_1_4_destructure_structures() {
        let foo = Foo { x: (1, 2), y: 3 };

        let Foo { x: (a, b), y } = foo;
        println!("a = {}, b = {}, y = {}", a, b, y);

        let Foo { x: i, y: j } = foo;
        println!("i = {:?}, j = {:?}", i, j);

        let Foo { y, .. } = foo;
        println!("y = {}", y);
    }
}