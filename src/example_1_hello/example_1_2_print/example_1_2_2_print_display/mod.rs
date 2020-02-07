mod example_1_2_2_1_test_case_list;

use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
struct MinMax(i64, i64);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D { x: f64, y: f64 }

impl Display for Point2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn example_1_2_2_print_display() {
        let minmax = MinMax(0, 14);

        println!("Compare structures:");
        println!("Display: {}", minmax);
        println!("Debug: {:?}", minmax);

        let big_range =   MinMax(-300, 300);
        let small_range = MinMax(-3, 3);

        println!("The big range is {big} and the small is {small}",
                 small = small_range,
                 big = big_range);

        let point = Point2D { x: 3.3, y: 7.2 };

        println!("Compare points:");
        println!("Display: {}", point);
        println!("Debug: {:?}", point);

        // 报错。`Debug` 和 `Display` 都被实现了，但 `{:b}` 需要 `fmt::Binary`
        // 得到实现。这语句不能运行。
        // println!("What does Point2D look like in binary: {:b}?", point);
    }
}