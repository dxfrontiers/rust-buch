use mockall::automock;

#[automock()]
pub mod module_to_be_mocked {
    pub fn return_string() -> &'static str {
        "Hello from productive code"
    }
}

#[cfg(test)]
mod tests {
    // We want to import every public function from the outer / root scope of the module
    use super::*;

    #[test]
    fn test_return_string() {
        let ctx = mock_module_to_be_mocked::return_string_context();
        ctx.expect()
            .return_const("Hello from mock!");
        assert_eq!("Hello from mock!", module_to_be_mocked::return_string());
    }
}