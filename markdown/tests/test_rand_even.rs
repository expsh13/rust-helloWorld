#[test]
fn test_rand_even() {
    for _ in 0..100 {
        // 返り値が偶数であることを確認
        let result = markdown::rand_even();
        assert_eq!(result % 2, 0);
    }
}
