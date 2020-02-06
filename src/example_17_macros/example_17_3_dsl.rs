macro_rules! calculate {
    (eval $e:expr) => {
        {
            {
                let val:usize = $e;
                println!("{} = {}", stringify!{$e}, val);
            }
        }
    };
}


fn example_17_3_dsl() {
    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_17_3_dsl() {
        example_17_3_dsl();
    }
}