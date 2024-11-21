fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let r = "hello";
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える

    println!("{}", s); // これは`hello, world!`と出力する
}
