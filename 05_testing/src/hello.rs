pub fn hello() -> String {
    String::from("Hello")
}

#[cfg(test)]
mod tests {
    // We want to import every public function from the outer / root scope of the module
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello");
    }
}