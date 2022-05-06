use mockall::automock;

#[automock]
pub trait Greeter {
    fn greet(self) -> String;
}

struct GreeterImpl {}

impl Greeter for GreeterImpl {
    fn greet(self) -> String {
        String::from("Hello world!")
    }
}

#[allow(dead_code)]
pub fn use_greeter<G: Greeter>(greeter: G) -> String {
    greeter.greet()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeter() {
        let mut mock = MockGreeter::new();
        mock.expect_greet().return_const("Hello from the mock!");

        assert_eq!("Hello from the mock!", use_greeter(mock));
    }

    #[test]
    fn test_greeter_productive_code() {
        assert_eq!("Hello world!", use_greeter(GreeterImpl {}));
    }
}
