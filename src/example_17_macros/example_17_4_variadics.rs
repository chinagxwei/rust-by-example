macro_rules! calculate {
    (eval $e:expr) => {
        {
            {
                let val: usize = $e;
                println!("{} = {}", stringify!{$e}, val);
            }
        }
    };
    (eval $e:expr, $(eval $es:expr), +) => {
        {
            calculate!{eval $e}
            calculate!{$(eval $es), +}
        }
    }
}

fn example_17_4_variadics(){
    calculate!{
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_17_4_variadics() {
        example_17_4_variadics();
    }
}