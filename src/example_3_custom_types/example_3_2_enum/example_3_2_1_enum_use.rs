#![allow(dead_code)]

pub enum Status {
    Rich,
    Poor,
}

pub enum Work {
    Civilian,
    Soldier,
}

#[cfg(test)]
mod tests {
    use super::*;
    use Status::{Poor, Rich};
    use Work::{Civilian, Soldier};

    #[test]
    fn example_3_2_1_enum_use() {
        let status = Poor;

        let work = Civilian;

        match status {
            Rich => println!("The rich have lots of money!"),
            Poor => println!("The poor have nno money...")
        }

        match work {
            Civilian => println!("Civilians work!"),
            Soldier => println!("Soldiers fight!")
        }
    }
}