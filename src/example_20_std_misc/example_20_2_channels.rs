use std::sync::mpsc;
use std::thread;

static NTHREADS: i32 = 3;

pub fn channels() {
    let (sh, rh) = mpsc::channel::<i32>();

    for i in 0..NTHREADS {
        let thread_sh = sh.clone();
        thread::spawn(move || {
            thread_sh.send(i).unwrap();

            println!("thread {} finished", i);
        });
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);

    for _ in 0..NTHREADS {
        ids.push(rh.recv());
    }

    println!("{:?}", ids);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_20_2_channels() {
        channels();
    }
}