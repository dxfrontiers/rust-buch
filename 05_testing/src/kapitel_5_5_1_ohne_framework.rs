

pub trait Greeter {
    fn greet(self) -> String;
}

struct GreeterImpl {}

impl Greeter for GreeterImpl {
    fn greet(self) -> String {
        String::from("Hello world!")
    }
}

pub fn use_greeter<G: Greeter>(greeter: G) -> String {
    greeter.greet()
}

#[cfg(test)]
mod tests {
    // We want to import every public function from the outer / root scope of the module
    use super::*;

    struct MockGreeter {}

    impl Greeter for MockGreeter {
        fn greet(self) -> String {
            String::from("Hello from the mock!")
        }
    }

    #[test]
    fn test_greeter() {
        assert_eq!(use_greeter(MockGreeter{}), "Hello from the mock!");
    }
}