use mockall::*;
use mockall::predicate::*;

#[automock]
pub trait Greeter {
    fn greet(self) -> String;
}

struct GermanGreeter {}

impl Greeter for GermanGreeter {
    fn greet(self) -> String {
        String::from("Hallo produktive Welt!")
    }
}

pub fn use_greeter<G: Greeter>(greeter: G) -> String {
    greeter.greet()
}

#[cfg(test)]
mod tests {
    // We want to import every public function from the outer / root scope of the module
    use super::*;

    #[test]
    fn test_greeter() {
        let mut mock = MockGreeter::new();
        mock.expect_greet()
            .return_const("Hello from the mock!");

        assert_eq!("Hello from the mock!", use_greeter(mock) );
    }

    #[test]
    fn test_greeter_productive_code() {
        assert_eq!("Hallo produktive Welt!", use_greeter(GermanGreeter{}) );
    }
}