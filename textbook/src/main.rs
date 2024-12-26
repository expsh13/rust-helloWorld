use std::ops::Mul;

fn main() {
    fn square<T>(x: T) -> T
    where
        T: Mul<Output = T> + Copy,
    {
        x * x
    }
}
