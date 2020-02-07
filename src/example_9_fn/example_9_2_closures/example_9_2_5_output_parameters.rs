fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_9_2_5_output_parameters() {
        let fn_plain = create_fn();
        let mut fn_mut = create_fnmut();
        fn_plain();
        fn_mut();
    }
}