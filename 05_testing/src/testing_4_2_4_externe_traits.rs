#[cfg(test)]
mod tests {
    use mockall::mock;
    use std::fmt;

    mock! {
        MyStruct { }

        impl fmt::Debug for MyStruct {
            fn fmt<'a>(&self, f: &mut fmt::Formatter<'a>)
                -> fmt::Result {
                f.write_str("MyStruct")
            }
        }
    }

    #[test]
    fn test_fmt() {
        let mut mock = MockMyStruct::new();
        mock.expect_fmt().returning(|f| f.write_str("MyMock"));

        assert_eq!("MyMock", format!("{:?}", mock));
    }
}
