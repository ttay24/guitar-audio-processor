pub fn test_rust() -> String {
    println!("test!");
    "test_rust".into()
}

pub mod test_vst;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(test_rust(), "test_rust".to_string());
    }
}
