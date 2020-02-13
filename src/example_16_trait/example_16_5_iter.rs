struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_16_4_iter() {
        let mut sequence = 0..3;

        println!("Four consecutive `next` calls on 0..3");
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());
        println!("> {:?}", sequence.next());

        for i in 0..3 {
            println!("> {}", i);
        }

        println!("The first four terms of the Fibonacci sequence are: ");
        for i in fibonacci().take(4) {
            println!("> {}", i);
        }

        println!("The next four terms of Fibonacci sequence are: ");
        for i in fibonacci().skip(4).take(4) {
            println!("> {}", i);
        }

        let array = [1u32, 3, 3, 7];
        println!("Iterate the following arry {:?}", &array);
        for i in array.iter() {
            println!("> {}", i);
        }
    }
}