pub fn test_rust() -> String {
    "test_rust".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(test_rust(), "test_rust".to_string());
    }
}
