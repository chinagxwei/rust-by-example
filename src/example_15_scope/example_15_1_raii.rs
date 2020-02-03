
struct ToDrop;

impl Drop for ToDrop{
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn create_box(){
    let box_1 = Box::new(3i32);
}

fn example_15_1_raii(){

    let box_2 = Box::new(5i32);

    {
        let box_3 = Box::new(4i32);
    }

    for i in 0u32..1_000 {
        create_box();
    }

    let x = ToDrop;
    println!("Made a ToDrop!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_15_1_raii() {
        example_15_1_raii();
    }
}