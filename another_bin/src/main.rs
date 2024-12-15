use my_library::library::{book::Book, bookshelf::Bookshelf};

fn main() {
    let mut shelf = Bookshelf::new();
    let book1 = Book::new("gpt本", "名前");
    let book2 = Book::new("ダミー本", "名前2");
    shelf.add_book(book1);
    shelf.add_book(book2);

    let found_book = shelf.search_books("gpt");
    println!("{:?}", found_book);
}
