use deeply::nested::function as other_function;

fn function() {
    println!("Called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("Called `deeply::nested::function()`");
        }
    }
}

fn example_10_3_use() {
    other_function();

    println!("Entering block");

    {
        use deeply::nested::function;
        function();
        println!("Leaving block")
    }

    function();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_10_3_use() {
        example_10_3_use();
    }
}