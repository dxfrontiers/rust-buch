#[allow(dead_code)]
fn return_string() -> &'static str {
    "Hello"
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn pretty_assertions() {
        assert_eq!("Hello", return_string());
    }

    #[test]
    fn pretty_assertions_with_message() {
        assert_eq!(
            "Hello",
            return_string(),
            "Hier klappt etwas\
         nicht. Ich erwarte {} - bekomme jedoch {}",
            "Gude",
            return_string()
        );
    }
}
