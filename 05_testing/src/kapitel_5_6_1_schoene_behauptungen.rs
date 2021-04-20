
fn return_string() -> &'static str {
    return "Hello";
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};

    #[test]
    fn pretty_assertions() {
        assert_eq!("Hell", return_string());
    }

    #[test]
    fn pretty_assertions_with_message() {
        assert_eq!("Gude", return_string(), "Hier klappt etwas nicht.");
    }
}