type NanoSecond = u64;
type Inch = u64;

type U64T = u64;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_5_4_alias() {
        let nanoseconds: NanoSecond = 5 as U64T;
        let inches: Inch = 2 as U64T;

        println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
    }
}