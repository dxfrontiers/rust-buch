use crate::kapitel_1_6_2_4_modules::mock_module_to_be_mocked;
use crate::kapitel_1_6_2_4_modules::module_to_be_mocked;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_return_string() {
        let ctx = mock_module_to_be_mocked::return_string_context();
        ctx.expect()
            .return_const("Hello from mock!");
        assert_eq!("Hello from mock!", module_to_be_mocked::return_string());
    }
}