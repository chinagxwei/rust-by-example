use List::*;

enum List {
    Con(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Con(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Con(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Con(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("Nil")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_3_2_3_test_case_linked_list() {
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);

        println!("linked list has length: {}", list.len());
        println!("{}", list.stringify())
    }
}