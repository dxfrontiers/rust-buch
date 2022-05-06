use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait Greeter<T> {
    fn greet(self) -> T;
}

struct GreeterImpl {}

impl Greeter<String> for GreeterImpl {
    fn greet(self) -> String {
        String::from("Hello world!")
    }
}

#[allow(dead_code)]
pub fn use_greeter<T, G: Greeter<T>>(greeter: G) -> T {
    greeter.greet()
}

#[cfg(test)]
mod tests {
    // We want to import every public function from the outer / root scope of the module
    use super::*;

    #[test]
    fn test_my_greeter() {
        let mut mock = MockGreeter::<String>::new();
        mock.expect_greet()
            .times(1)
            .return_const(String::from("Hello from mockall mock!"));
        assert_eq!(use_greeter(mock), "Hello from mockall mock!");
    }

    #[test]
    fn test_my_greeter_with_str() {
        let mut mock = MockGreeter::<&'static str>::new();
        mock.expect_greet()
            .times(1)
            .return_const("Hello from str mockall mock!");
        assert_eq!(use_greeter(mock), "Hello from str mockall mock!");
    }

    #[test]
    fn test_my_greeter_with_i32() {
        let mut mock = MockGreeter::<i32>::new();
        mock.expect_greet().times(1).return_const(333);
        assert_eq!(use_greeter(mock), 333);
    }
}
