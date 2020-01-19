use std::mem;

fn use_move() {
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    //println!("There're {} elements in vec", haystack.len());
}

fn example_9_2_1_capture() {
    let color = "green";

    let print = || println!("`color`: {}", color);

    print();
    print();

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count)
    };

    inc();
    inc();

//    let reborrow = &mut count;

    let movable = Box::new(3);

    let consume = ||{
        println!("`move`: {}",movable);
        mem::drop(movable)
    };

    consume();
    use_move();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_9_2_1_capture() {
        example_9_2_1_capture();
    }
}