fn main() {
    #[derive(Clone)]
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool
    }

    fn build_book_data(title: String, author: String, pages: u32, available: bool) -> Book {
        Book {
            title,
            author,
            pages,
            available
        }
    }

    // First method
    let book1: Book = Book {
        title: "Book1".to_string(),
        author: "Author1".to_string(),
        pages: 3,
        available: true
    };

    // Second method
    let book2: Book = build_book_data("Book2".to_string(), "Author2".to_string(), 5, false);

    // If the instance want to inherit data from others
    let book3: Book = Book {
        title: "Book3".to_string(),
        ..book1.clone()
    };

    println!("{} has author {}, {} pages and are now available: {}", book1.title, book1.author, book1.pages, book1.available);
    println!("{} has author {}, {} pages and are now available: {}", book2.title, book2.author, book2.pages, book2.available);
    println!("{} has author {}, {} pages and are now available: {}", book3.title, book3.author, book3.pages, book3.available);
}