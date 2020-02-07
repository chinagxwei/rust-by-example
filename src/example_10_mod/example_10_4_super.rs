fn function() {
    println!("Called `function()`");
}

mod cool {
    pub fn function() {
        println!("Called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("Called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("Called `my::cool::function()`")
        }
    }

    pub fn indirect_call() {
        println!("Called `my::indirect_call`, that\n> ");

        self::function();

        function();

        self::cool::function();

        super::function();

        {
            use cool::function as root_function;
            root_function();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_10_4_super() {
        my::indirect_call();
    }
}