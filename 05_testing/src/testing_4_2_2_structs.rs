use mockall_double::double;

mod greeter {
    use mockall::automock;

    pub struct Greeter {
        pub name: String,
    }

    #[automock]
    #[allow(dead_code)]
    impl Greeter {
        pub fn greet(&self) -> String {
            format!("Hello productive code, {}!", self.name)
        }
    }
}

#[double]
use greeter::Greeter;

#[allow(dead_code)]
fn say_hello(greeter: &Greeter) -> String {
    greeter.greet()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let mut mock = Greeter::default();
        mock.expect_greet()
            .return_const(String::from("Hello from mock!"));
        let result = say_hello(&mock);

        assert_eq!("Hello from mock!", result);
    }
}
