fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes(); //バイト配列に

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let s = String::from("hello world");
    let my_string_literal = "hello world";

    let word = first_word(&s[..]);
    let word = first_word(my_string_literal);
    println!("the first word is: {}", word);
}
