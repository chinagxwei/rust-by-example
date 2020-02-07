use std::mem;

///
/// 内部不允许有可变声明的变量进行变更变量内容。
///
fn apply_fn<F>(f: F) where F: Fn() {
    f()
}

fn apply_fn_one<F>(f: F) where F: FnOnce() {
    f()
}

fn apply_fn_mut<F>(mut f: F) where F: FnMut() {
    f()
}

fn apply_to_3<F>(f: F) -> i32
    where F: Fn(i32) -> i32 {
    f(3)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_9_2_2_input_parameters() {
        let greeting = "hello";

        let mut farewell = "goodbye".to_owned();

        let diary = move || {
            println!("I said {}.", greeting);

            farewell.push_str("!!!");

            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzzz");

            mem::drop(farewell);
        };

        apply_fn_one(diary);
//    apply_fn(diary);
//    apply_fn_mut(diary);

        let double = |x| 2 * x;
        println!("3 double: {}", apply_to_3(double));
    }
}