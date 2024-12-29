pub fn rand_even() -> i32 {
    rand::random::<i32>() & !1
}
