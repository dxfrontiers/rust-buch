use mockall::*;
use mockall::predicate::*;

trait MyTrait {

}

#[cfg(test)]
mod tests {
    // We want to import every public function from the outer / root scope of the module
    use super::*;

    mock! {
        MyStruct {} // Name of the mock struct, less the "Mock" prefix

        impl Clone for MyStruct {   // specification of the trait to mock
            fn clone(&self) -> Self;
        }
    }

    #[test]
    fn test_clone() {
        let mut mock1 = MockMyStruct::new();
        let mock2 = MockMyStruct::new();
        mock1.expect_clone()
            .return_once(move || mock2);
        let cloned = mock1.clone();
    }
}

