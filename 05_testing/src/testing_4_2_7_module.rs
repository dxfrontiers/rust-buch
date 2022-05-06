use mockall_double::double;

mod outer {
    use mockall::automock;
    #[automock]
    #[allow(dead_code)]
    pub mod a_module {
        pub fn return_string() -> &'static str {
            "Hello from productive code"
        }
    }
}

#[double]
pub use outer::a_module;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mocked_return_string() {
        let ctx = a_module::return_string_context();
        ctx.expect().return_const("Hello from mock!");
        assert_eq!("Hello from mock!", a_module::return_string());
    }
}
