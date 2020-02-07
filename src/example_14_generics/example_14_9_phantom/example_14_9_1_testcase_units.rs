use std::marker::PhantomData;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
enum Inch {}

#[derive(Debug, Copy, Clone)]
enum Mm {}

#[derive(Debug, Copy, Clone)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_14_9_1_testcase_units() {
        let one_foot: Length<Inch> = Length(12.0, PhantomData);
        let one_meter: Length<Mm> = Length(1000.0, PhantomData);

        let two_feet = one_foot + one_foot;
        let two_meters = one_meter + one_meter;

        println!("one foot + one_foot = {:?} in", two_feet.0);
        println!("one meter + one_meter = {:?} mm", two_meters.0);

        // 无意义的运算当然会失败：
        // 编译期错误：类型不匹配。
        //let one_feter = one_foot + one_meter;
    }
}