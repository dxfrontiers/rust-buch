#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use mockall::*;
    use std::fmt;

    mock! {
        MyStructWithMethod {
            fn provide_number(&self) -> u16;
        }

        impl fmt::Debug for MyStruct {
            fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>) -> fmt::Result {
                f.write_str("MyStructWithMethod")
            }
        }
    }

    // Take me as external trait
    trait Convertible<T> {
        fn convert(&self) -> T;
    }

    mock! {
        pub ConvertableStruct<T> {}
        impl<T> Convertible<T> for ConvertableStruct<T> {
            fn convert(&self) -> T;
        }
    }

    #[test]
    fn test_fmt() {
        let mut mock = MockMyStructWithMethod::new();
        mock.expect_fmt().returning(|f| f.write_str("MyMock"));

        assert_eq!("MyMock", format!("{:?}", mock));
    }

    #[test]
    fn test_provide_number() {
        let mut mock = MockMyStructWithMethod::new();
        mock.expect_provide_number().return_const(333u16);

        assert_eq!(333u16, mock.provide_number());
    }

    #[test]
    fn test_generic_trait() {
        let mut mock = MockConvertableStruct::<String>::new();

        mock.expect_convert()
            .return_const(String::from("I am rather static"));

        assert_eq!(mock.convert(), "I am rather static");
    }
}
