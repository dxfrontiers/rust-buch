use mockall::*;
use mockall::predicate::*;

pub trait Greeter<T> {
    fn greet(self) -> T;
}

struct GreeterImpl {}

impl Greeter<String> for GreeterImpl {
    fn greet(self) -> String {
        String::from("Hello world!")
    }
}

pub fn use_greeter<T, G: Greeter<T>>(greeter: G) -> T {
    greeter.greet()
}

#[cfg(test)]
mod tests {
    // We want to import every public function from the outer / root scope of the module
    use super::*;

    #[test]
    fn test_my_greeter() {
        mock! {
            MyGreeter {} // Name of the mock struct, less the "Mock" prefix

            impl Greeter<String> for MyGreeter {   // specification of the trait to mock
                fn greet(self) -> String {
                    String::from("I won't be used")
                }
            }
        }

        let mut mock = MockMyGreeter::new();
        mock.expect_greet()
            .times(1)
            .return_const(String::from("Hello from mockall mock!"));
        assert_eq!(use_greeter(mock), "Hello from mockall mock!");
    }

    struct MockStrGreeter {}

    impl Greeter<&'static str> for MockStrGreeter {
        fn greet(self) -> &'static str {
            "Hello from the mock!"
        }
    }

    #[test]
    fn test_str_greeter() {
        assert_eq!(use_greeter(MockStrGreeter{}), "Hello from the mock!");
    }

    struct MockI32Greeter {}

    impl Greeter<i32> for MockI32Greeter {
        fn greet(self) -> i32 {
            333
        }
    }

    #[test]
    fn test_i32_greeter() {
        assert_eq!(use_greeter(MockI32Greeter{}), 333);
    }

}