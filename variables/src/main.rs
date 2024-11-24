fn main() {
    let mut s = String::from("hello");

    let _r1 = &s; // 問題なし
    let _r2 = &s; // 問題なし
    let _r3 = &mut s; // 大問題！
}
