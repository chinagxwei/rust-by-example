use std::fmt::Debug;

mod example_14_4_1_testcase_empty;

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }

#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.length
    }
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn example_14_4_bounds() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

//    print_debug(&_triangle);
//    println!("Area: {}", area(&_triangle));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_14_4_bounds() {
        example_14_4_bounds();
    }
}