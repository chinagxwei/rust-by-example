fn print_n(n: i32) {
    if n % 15 == 0 {
        println!("fizzbuzz");
    } else if n % 3 == 0 {
        println!("fizz");
    } else if n % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}


fn example_8_4_for_1() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..52 {
        print_n(n);
    }
}

fn example_8_4_for_2() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..=51 {
        print_n(n);
    }
}

fn example_8_4_for_3() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name)
        }
    }
}

fn example_8_4_for_4() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name)
        }
    }
}

fn example_8_4_for_5() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello"
        }
    }
    println!("names: {:?}", names);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_8_4_for() {
        example_8_4_for_1();
        example_8_4_for_2();
        example_8_4_for_3();
        example_8_4_for_4();
        example_8_4_for_5();
    }
}
