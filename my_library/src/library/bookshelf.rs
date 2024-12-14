use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

use super::book::Book;

pub struct Bookshelf {
    books: Vec<Book>,
    matcher: SkimMatcherV2,
}
impl Bookshelf {
    pub fn new() -> Self {
        let matcher = SkimMatcherV2::default();
        Self {
            books: Vec::new(),
            matcher: matcher,
        }
    }
    // 本を追加する
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    // 検索
    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|book| self.matcher.fuzzy_match(&book.title, title_query).is_some())
            .collect()
    }

    pub fn remove_book(&mut self, book: &Book) -> Option<Book> {
        todo!("Implement `Bookshelf::remove_book`");
    }
    pub fn take_all_books(&mut self) -> Vec<Book> {
        todo!("Implement `Bookshelf::take_all_books`")
    }
}

#[cfg(test)]
mod tests {
    use crate::library::book;

    use super::{Book, Bookshelf};
    #[test]
    fn test_bookshelf() {
        let mut shelf = Bookshelf::new();
        let book1 = Book::new("タイトル", "名前");
        let book2 = Book::new("あああ", "名前2");
        shelf.add_book(book1);
        shelf.add_book(book2);

        let found_books = shelf.search_books("タイ");
        println!("{:?}", found_books);
    }
}
