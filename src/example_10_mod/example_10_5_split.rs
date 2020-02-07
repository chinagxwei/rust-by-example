use crate::example_10_mod::my;

fn function() {
    println!("Called `function()`");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_10_5_split() {
        my::function();

        function();

        my::indirect_access();

        my::nested::function();
    }
}