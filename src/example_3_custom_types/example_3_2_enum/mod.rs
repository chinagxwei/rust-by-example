#![allow(dead_code)]

mod example_3_2_2_c_like;
mod example_3_2_1_enum_use;
mod example_3_2_3_test_case_linked_list;
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("loaded page"),
        WebEvent::PageUnload => println!("unloaded page"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => println!("clicked at x:{}, y:{}", x, y)
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_3_2_enum() {
        let pressed = WebEvent::KeyPress('y');
        let pasted = WebEvent::Paste(String::from("my text"));
        let click = WebEvent::Click { x: 30, y: 90 };
        let load = WebEvent::PageLoad;
        let unload = WebEvent::PageUnload;

        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);

        let x = Operations::Add;
    }
}