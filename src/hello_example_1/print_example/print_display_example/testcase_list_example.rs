use std::fmt::{Display, Formatter, Error};

struct List(Vec<i32>);

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let vec = &self.0;
        write!(f, "[")?;
        for (index, value) in vec.iter().enumerate() {
            if index != 0 { write!(f, ", ")?; };
            write!(f, "{}", value);
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_testcase_list() {
        let v = List(vec![1, 2, 3]);
        println!("{}", v);
    }
}