#[derive(Copy, Clone)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutable borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 1989;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_15_3_1_mut() {
        let immutable = Book {
            author: "Douglas Hofstadter",
            title: "Godel, Escher, Bach",
            year: 1979,
        };

        let mut mutable = immutable;

        borrow_book(&immutable);

        borrow_book(&mutable);

        new_edition(&mut mutable);

//    new_edition(&mut immutable);
    }
}

