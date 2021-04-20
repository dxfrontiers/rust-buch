use crate::kapitel_5_5_2_6_module::a_module;

pub fn use_module() -> &'static str {
    return a_module::return_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_module() {
        let ctx = a_module::return_string_context();
        ctx.expect()
            .return_const("Hello from a different mock!");
        assert_eq!("Hello from a different mock!", a_module::return_string());
    }
}