#[derive(PartialOrd, PartialEq)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let Inches(inches) = self;

        Centimeters(*inches as f64 * 2.54)
    }
}

struct Seconds(i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_16_1_derive() {
        let one_second = Seconds(3);

        let foot = Inches(9);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100f64);

        let cmp = if foot.to_centimeters() < meter { "smaller" } else { "bigger" };

        println!("One foot is {} than one meter.", cmp);
    }
}