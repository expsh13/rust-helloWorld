use std::iter::Iterator;

fn main() {
    #[derive(Debug, Clone)]
    enum List<T> {
        Node { data: T, next: Box<List<T>> },
        Nil,
    }
    impl<T> List<T> {
        fn new() -> List<T> {
            List::Nil
        }

        // リストを消費して、そのリストの先頭に引数dataを追加したリストを返す
        fn cons(self, data: T) -> List<T> {
            List::Node {
                data: data,
                next: Box::new(self),
            }
        }

        // 不変イテレーターを返す
        fn iter<'a>(&'a self) -> ListIter<'a, T> {
            ListIter { elm: self }
        }
    }

    // 不変イテレーターの型
    struct ListIter<'a, T> {
        elm: &'a List<T>,
    }
    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        // 次の要素を返す
        fn next(&mut self) -> Option<Self::Item> {
            match self.elm {
                List::Node { data, next } => {
                    self.elm = next;
                    Some(data)
                }
                List::Nil => None,
            }
        }
    }
}
