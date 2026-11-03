struct Book {
    name: String,
    author: String,
    publisher: String,
    year: u16,
    price: u32,
    page: u16,
}

impl Book {
    fn new(name: &str, author: &str, publisher: &str, year: u16, price: u32, page: u16) -> Self {
        Book {
            name: String::from(name),
            author: String::from(author),
            publisher: String::from(publisher),
            year,
            price,
            page,
        }
    }
    fn display(&self) {
        println!(
            "Name: {}, Author: {}, Publisher: {}, Year: {}, Price: {}, Page: {}",
            self.name, self.author, self.publisher, self.year, self.price, self.page
        );
    }
}

fn main() {
    let book = Book::new(
        "The Rust Programming Language",
        "Steve Klabnik and Carol Nichols",
        "No Starch Press",
        2018,
        39,
        552,
    );
    book.display();
}
