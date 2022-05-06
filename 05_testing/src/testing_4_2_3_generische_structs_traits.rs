use mockall::automock;

#[automock]
trait Greeter<T: 'static> {
    fn greet(self, t: T) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let mut mock = MockGreeter::<&'static str>::new();
        mock.expect_greet().returning(|t| format!("Hello {}!", t));
        assert_eq!("Hello reader!", mock.greet("reader"));
    }
}
