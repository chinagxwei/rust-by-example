struct Cardinal;

struct BlueJay;

struct Turkey;

trait Red {}

trait Blue {}

impl Red for Cardinal {}

impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str { "red" }

fn blue<T: Blue>(_: &T) -> &'static str { "blue" }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_14_4_1_testcase_empty() {
        let cardinal = Cardinal;
        let blue_jay = BlueJay;

        let _turkey = Turkey;

        println!("A cardinal is {}", red(&cardinal));
        println!("A blue jay is {}", blue(&blue_jay));
//    println!("A turkey is {}",blue(&_turkey));
    }
}