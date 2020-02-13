pub mod example_20_1_1_testcase_mapreduce;


use std::thread;

static NTHREADS: i32 = 10;

pub fn threads() {
    let mut children = vec![];

    for i in 0..NTHREADS {
        children.push(
            thread::spawn(move || {
                println!("This is thread number {}", i);
            })
        );
    }

    for child in children {
        let _ = child.join();
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn example_20_1_threads() {
        threads();
    }
}