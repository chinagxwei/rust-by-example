#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_19_2_vec() {
        let collected_iterator: Vec<i32> = (0..10).collect();
        println!("Collected (0..10) into: {:?}", collected_iterator);

        let mut xs = vec![1i32, 2, 3];
        println!("Initial vector: {:?}", xs);

        println!("Push 4 into the vector");
        xs.push(4);
        println!("Vector: {:?}", xs);

//        collected_iterator.push(4);

        println!("Vector size: {}", xs.len());

        println!("Second element: {}", xs[1]);

        println!("Pop last element: {:?}", xs.pop());

//        println!("Fourth element: {}", xs[3]);

        println!("Contents of xs:");
        for i in xs.iter() {
            println!("> {}", i);
        }

        for (i, v) in xs.iter().enumerate() {
            println!("In position {} we have value {}", i, v);
        }

        for i in xs.iter_mut() {
            *i *= 3;
        }

        println!("Update vector: {:?}", xs);
    }
}