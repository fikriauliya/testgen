pub trait TestSpec<T> {
    fn test_cases() -> Vec<T>;
}
