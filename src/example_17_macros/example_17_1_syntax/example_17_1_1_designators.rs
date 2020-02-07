
macro_rules! create_function {
    ($function_name:ident) => {
        fn $function_name() {
            println!("You called {:?}()", stringify!($function_name))
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}",stringify!($expression),$expression)
    };
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_17_1_1_designators() {
        foo();
        bar();
        print_result!(1u32 + 2);
        print_result!({
           let x = 1u32;
           x * x + 2 * x - 1
        });
    }
}