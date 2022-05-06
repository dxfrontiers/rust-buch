use mockall::predicate::*;
use mockall::*;

#[automock]
trait Foo {
    fn foo<T: 'static>(&self, t: T) -> i32;
}

#[cfg(test)]
mod tests {
    // We want to import every public function from the outer / root scope of the module
    use super::*;

    #[test]
    fn test_foo() {
        let mut mock = MockFoo::new();
        mock.expect_foo::<i16>().returning(|t| i32::from(t));
        mock.expect_foo::<i8>().returning(|t| -i32::from(t));

        assert_eq!(5, mock.foo(5i16));
        assert_eq!(-5, mock.foo(5i8));
    }
}
