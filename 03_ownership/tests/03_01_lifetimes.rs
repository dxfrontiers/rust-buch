#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod lifetimes{
    use std::borrow::Borrow;
    use std::thread;
    use std::sync::{Arc, Mutex};


    /**
        IMPORTANT: Some functions are commented out since they showcase code that does not compile
        It is recommended that you comment them back-in one-by-one and understand why they fail.
    */

/*    #[test]
    fn test_lifetime_intro() {
        let x;
        {
            let y = 7;
            x = &y;
        } // Ende der Lifetime von y
        // error[E0597]: `y` does not live long enough
        println!("x: {}",x);
    }*/


/*    #[test]
    fn test_lifetime_basics_illegal() {
        let x = "Test".to_string();
        let a = &x;
        my_fn_owned(x);
        // error[E0505]: cannot move out of `x` because it is borrowed
        println!("a is {}",a);
    }*/

    #[test]
    fn test_lifetime_basics() {
        let s = "Test".to_string();
        shout(&s);
        println!("s is {}",s);
    }

    // immutable reference
    fn shout(phrase: &String){
        println!("shouting: {}",phrase.to_uppercase());
    }

    // takes ownership
    fn my_fn_owned(phrase: String){
        println!("shouting: {}",phrase.to_uppercase());
    }


    /**
        How how the compiler can infer the lifetime of the return value if only one parameter is given
    */
    #[test]
    fn test_lifetime_one_param() {
        let v = vec![42,9,7,1];
        let favourite = extract_favourite(&v);
        println!("v is {:?}",v);
        println!("favourite is {:?}",favourite);
        assert_eq!(*favourite,42);
    }

    fn extract_favourite(input: &Vec<u8>) -> &u8{
        &input[0]
    }

    /**
        Show that we need to explicitly state lifetimes of input and output parameters,
        if the compiler cannot infer them.
    */
    #[test]
    fn test_lifetime_two_params() {
        let a = 7;
        {
            let b = 5;
            let prettier = select_prettier_number(&a,&b);
            assert_eq!(*prettier,b);
            drop(b);
        }
        assert_eq!(a,7);
    }

    /**
        Here it gets ugly with lifetimes:
        Lets say we have a complex function that decides which one of thwo pased numbers is prettier.
        The numbers are passed as borrows and the result is also a reference.
        With numbers one could easily just return the prettier number but if `copy` or `clone` are
        not an option, this function is not too far from realistic.

        Since it is the functions job to decide between the numbers, we cannot fix their lifetimes
        for the return value. The following does not work.
        ```
            select_prettier_number<'a,'b >(_x: &'a i32, y: &'b i32) -> &'____ i32
                    which one to use, a or b ? --------------------------^

        ```
        Instead, we need to use `:` to indicate that `b` 'lives at least as long as' `a` so that we
        can return the shorter of the two lifetimes, `a`.
        An alternative would be to use the same lifetime for both parameters, e.g.
        ```
            fn select_prettier_number<'a>(_x: &'a i32, y: &'a i32) -> &'a i32{
        ```
        This has however implications outside of the function
    */
    fn select_prettier_number<'a,'b: 'a>(_x: &'a i32, y: &'b i32) -> &'a i32{
        y
    }


    /**
        Showcases that the compiler can determine lifetimes from the &self parameter in functions
        of structs.
    */
    struct Foo(String);
    impl Foo{
        fn get_tuple(&self, i: & usize) -> &str{
            self.0.get(*i..(i+2)).unwrap()
        }
    }



    /**
        The struct has an owned and a borrowed field.
        The borrowed field needs to have a lifetime.
        The compiler needs to make sure, that the borrow lives at least as long as the struct.
    */
    struct Holder<'a>{
        owned: String,
        borrowed: &'a String
    }

    #[test]
    fn test_lifetime_struct() {
        let a = "A".to_string();
        let b = "B".to_string();
        let holder = Holder{ owned: a, borrowed: &b };
        // drop(b); // illegal, since b is still borrowed in `holder`
        println!("borrowed is {}",holder.borrowed);
    }


    /*
        Static lifetimes and the lazy static macro
     */


    const BASE_TEN: u32 = 10;
    static mut useless_vec: Vec<u32> = Vec::new();
    const const_useless_vec: Vec<u32> = Vec::new();
    #[test]
    fn test_static(){
        println!("Base 10: {}",BASE_TEN);
        unsafe{
            // Only allowed in unsafe block
            println!("Static mut: {}",useless_vec.len());
        }
        println!("Const: {}",const_useless_vec.len());

        // allowed but not recommended
        thread::spawn(||{println!("from the other thread:{}", unsafe{  useless_vec.len()})}).join().unwrap();
    }


    /**
        This would not work
        `static NICE_VEC: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));`
        error[E0015]: calls in statics are limited to constant functions, tuple structs and tuple variants
        We use the lazy static crate to solve ths is problem
     */

    lazy_static! {
        static ref NICE_VEC: Mutex<Vec<u32>> = Mutex::new(Vec::new());
    }

    #[test]
    fn test_lazy_static(){

        thread::spawn(||{
            NICE_VEC.lock().unwrap().push(7);
            println!("b:{}",NICE_VEC.lock().unwrap().len());
            }).join().unwrap();
        thread::spawn(||{println!("b:{}",const_useless_vec.len())}).join().unwrap();
    }




    /* TODO fliegt vermutlich raus
78 |     fn get_tuple(s: &String, i: & usize) -> &str{
   |                     -------     -------     ^ expected named lifetime parameter

fn get_tuple(s: &String, i: & usize) -> &str{
    s.get(*i..(i+2)).unwrap()
}
 */

    /*
    #[test]
    fn test_elision_of_self() {
        let foo = Foo("Test123".to_string());
        let idx = 0;
        let res= foo.get_tuple(&idx);
        println!("Tuple {} is {}", idx, res);


        // let foo = "Test123".to_string();
        // let idx = 0;
        // let res= get_tuple(&foo, &idx);
        // println!("Tuple {} is {}", idx, res);


    }*/

    /* TODO fliegt vermutlich raus
    trait HolderView<'a>{
        fn get_a(&'a self) -> &'a str;
        fn get_b(&'a self) -> &'a str;
    }
    impl <'a,'b> HolderView<'a> for Holder<'b>{
        fn get_a(&'a self) -> &'a str {
            self.owned.as_str()
        }

        fn get_b(&'a self) -> &'a str {
            self.borrowed
        }
    }

    #[test]
    fn test_lifetime_impl_trait() {
        let a = "A".to_string();
        let b = "B".to_string();
        let holder = Holder{ owned: a, borrowed: &b };
        let ref_a = holder.get_a();
        let ref_b = holder.get_b();
        println!("Ref a {}, {:p}",ref_a,ref_a);
        println!("Ref b {}, {:p}",ref_b,ref_b);
    }

     */

}