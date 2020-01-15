type NanoSecond = u64;
type Inch = u64;

type u64_t = u64;

fn alias_example() {
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_example() {
        alias_example();
    }
}