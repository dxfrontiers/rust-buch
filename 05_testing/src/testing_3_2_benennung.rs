fn greet(greeted: Option<String>) -> String {
    return match greeted {
        Some(greeted) => return format!("Hello {}!", greeted),
        None => "Hello anonymous!".into(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_somename_greetedbyname() {
        assert_eq!(greet(Option::Some(String::from("Marcel"))), "Hello Marcel!");
    }

    #[test]
    fn test_greet_none_greetedanonymously() {
        assert_eq!(greet(Option::None), "Hello anonymous!");
    }
}
