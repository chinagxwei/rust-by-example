fn is_odd(n: u32) -> bool {
    n % 2 == 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_9_3_hof() {
        println!("Find the sum of all the squared odd numbers under 1000");
        let upper = 1000;

        let mut acc = 0;

        for i in 0.. {
            let i_squared = i * i;

            if i_squared >= upper {
                break;
            } else if is_odd(i_squared) {
                acc += i_squared;
            }
        }
        println!("imperative style: {}", acc);

        let sum_of_squared_odd_numbers = (0..)
            .map(|x| x * x)
            .take_while(|&x| x < upper)
            .filter(|&x| is_odd(x))
            .fold(0, |sum, x| sum + x);
        println!("functional style: {}", sum_of_squared_odd_numbers);
    }
}