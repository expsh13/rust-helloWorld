use std::{
    fs::File,
    io::{Read, Write},
    iter::Iterator,
    path::Path,
};

use serde::{Deserialize, Serialize};
fn main() {
    #[derive(Debug, Clone, Serialize, Deserialize)]
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

    let list = List::new().cons(1).cons(2).cons(3);

    let json = serde_json::to_string(&list).unwrap();
    println!("JSON: {} bytes {json}", json.len());

    let yml = serde_yaml::to_string(&list).unwrap();
    println!("YAML: {} bytes {yml}", yml.len());

    let msgpack = rmp_serde::to_vec(&list).unwrap();
    println!("MessagePack: {} bytes", msgpack.len());

    let list = serde_json::from_str::<List<i32>>(&json).unwrap();
    println!("List: {:?}", list);

    let list = serde_yaml::from_str::<List<i32>>(&yml).unwrap();
    println!("List: {:?}", list);

    let list = rmp_serde::from_slice::<List<i32>>(&msgpack).unwrap();
    println!("List: {:?}", list);

    // ファイルへの書き出し
    let path = Path::new("test.yml");
    let mut f = File::create(path).unwrap();
    f.write_all(yml.as_bytes()).unwrap();

    // ファイルからの読み込み
    let path = Path::new("test.yml");
    let mut f = File::open(path).unwrap();
    let mut yml = String::new();
    f.read_to_string(&mut yml).unwrap();

    // デシリアライズ
    let list = serde_yaml::from_str::<List<i32>>(&yml).unwrap();
    println!("List: {:?}", list);
}
