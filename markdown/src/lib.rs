/// ```
/// use markdown::my_func;
/// let n = my_func().unwrap();
/// ```
pub fn my_func() -> Option<i32> {
    Some(42)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_func() {
        assert_eq!(my_func(), Some(42));
    }
}
