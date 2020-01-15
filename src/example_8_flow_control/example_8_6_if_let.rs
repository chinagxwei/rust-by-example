#[derive(Debug)]
enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn example_8_6_if_let_1() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(e) = number {
        println!("Matched {:?}", e);
    }

    if let Some(e) = letter {
        println!("Matched {:?}", e)
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(e) = emoticon {
        println!("Matched {:?}", e);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emotion :)!");
    }
}

fn example_8_6_if_let_2() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(e) = c {
        println!("c is {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_8_6_if_let() {
        example_8_6_if_let_1();
        example_8_6_if_let_2();
    }
}