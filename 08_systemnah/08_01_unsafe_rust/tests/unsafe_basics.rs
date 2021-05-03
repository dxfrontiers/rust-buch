
#[cfg(test)]
mod unsafe_basics{

    #[test]
    fn sample_pointer_basics(){
        let x = 5;
        let ptr = &x as *const i32;
        println!("Pointer is {:?}",ptr);
        unsafe {
            println!("x is {}",*ptr);
        }
    }

    #[test]
    fn sample_raw_pointers(){
        let a = 6;
        unsafe_increase(&a);
        print_variable_at_pointer( &a as *const i32);
        assert_eq!(a,7);
    }



    fn unsafe_increase(var: &i32){
        unsafe {
            let readable_ptr = var as *const i32;
            let writeable_ptr = readable_ptr as *mut i32;
            *writeable_ptr += 1
        }
    }
    #[test]
    fn poke_around_in_mem(){
        let a = 6;
        let readable_ptr = &a as *const i32;
        let mut addr = readable_ptr as u64;
        print_variable_at_pointer(addr as *const i32);
        addr += 4;
        print_variable_at_pointer(addr as *const i32);
    }


    /// Helper function to print a pointer and its value without any checks
    fn print_variable_at_pointer(ptr: *const i32){
        println!("Pointer is {:?}",ptr);
        unsafe {
            println!("Var is {}",*ptr);
        }
    }

}
