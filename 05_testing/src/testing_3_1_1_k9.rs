#[allow(dead_code)]
fn return_error() -> Result<&'static str, &'static str> {
    Err("Das klappt nicht!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use k9::assert_err_matches_regex;

    #[test]
    fn test_error() {
        assert_err_matches_regex!(return_error(), "klappt.*nicht");
    }
    // Bitte Kommentare entfernen, um die schöne Fehlermeldung zu sehen.
    /*
    fn test_error() {
        assert_err_matches_regex!(return_error(), "klappt.*doch");
    }
    */
}
