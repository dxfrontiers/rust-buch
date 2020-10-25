

pub mod to_be_mocked_wrapper_module {
    use mockall::*;

    #[automock]
    pub trait TraitToBeMocked {
        fn method_returning_passed_int_value(&self, x: u32) -> u32;
    }

    #[automock()]
    pub mod module_to_be_mocked {
        fn world() -> &'static str {
            "world"
        }

        pub fn hello_world() -> String {
            format!("Hello {}!", world())
        }
    }

    cfg_if::cfg_if! {
        if #[cfg(test)] {
            pub use mock_module_to_be_mocked as prepared_module;
        } else {
            pub use module_to_be_mocked as prepared_module;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::module_to_be_mocked;
        use pretty_assertions::{assert_eq};

        #[test]
        fn test_helloworld() {
            assert_eq!(module_to_be_mocked::hello_world(), "Hello world!");
        }
    }
}

pub mod lets_mock_module {
    use super::to_be_mocked_wrapper_module::prepared_module::hello_world;
    use super::to_be_mocked_wrapper_module::TraitToBeMocked;

    pub fn do_something() -> String {
        hello_world()
    }

    pub fn call_with_four(x: &dyn TraitToBeMocked) -> u32 {
        x.method_returning_passed_int_value(4)
    }

    pub struct MyStruct {}

    impl TraitToBeMocked for MyStruct {
        fn method_returning_passed_int_value(&self, x: u32) -> u32 {
            x
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use mockall::*;
        use crate::hello_mockall::to_be_mocked_wrapper_module;

        #[test]
        fn test_call_with_four() {

            let mut mock = to_be_mocked_wrapper_module::MockTraitToBeMocked::new();
            mock.expect_method_returning_passed_int_value()
                .with(predicate::eq(4))
                .times(1)
                .returning(|x| x + 1);
            let result = call_with_four(&mock);
            let expected = 5;
            assert_eq!(expected, result);
        }

        #[test]
        fn test_do_something() {

            let ctx = to_be_mocked_wrapper_module::prepared_module::hello_world_context();
            ctx.expect()
                .returning(|| String::from("Gude"));

            assert_eq!(super::do_something(), "Gude");
        }

        #[test]
        fn test_without_any_mock() {

            let ctx = to_be_mocked_wrapper_module::prepared_module::hello_world_context();
            ctx.expect()
                .returning(|| String::from("Gude"));

            assert_eq!(super::do_something(), "Gude");
        }
    }

}