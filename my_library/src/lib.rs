mod library {
    mod book {
        pub struct Book {
            title: String,
            author: String,
        }
        impl Book {
            fn new(title: &str, author: &str) -> Self {
                Self {
                    title: title.to_string(),
                    author: author.to_string(),
                }
            }
        }
        pub struct Page {
            pub content: String,
        }
    }
    mod magazine {
        pub struct Page {
            pub content: String,
        }
    }
    mod bookshelf {
        use super::book::Book;
        use super::book::Page;
        use super::magazine::Page;

        struct Bookshelf {
            books: Vec<Book>,
        }
        impl Bookshelf {
            fn new() -> Self {
                Self { books: Vec::new() }
            }
            pub fn add_book(&mut self, book: Book) {
                self.books.push(book);
            }
            pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
                todo!("Implement `Bookshelf::search_books`");
            }
            pub fn remove_book(&mut self, book: &Book) -> Option<Book> {
                todo!("Implement `Bookshelf::remove_book`");
            }
            pub fn take_all_books(&mut self) -> Vec<Book> {
                todo!("Implement `Bookshelf::take_all_books`")
            }
        }
    }
}
