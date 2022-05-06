use libc::size_t;

#[link(name = "ext")]
extern "C" {
    fn add(a: size_t, b: size_t,) -> size_t;
    fn increment(target: *mut u32);
    fn dangling() -> *mut u32;
    fn zero() -> *mut u32;
}

fn main() {
    demonstrate_add();
    demonstrate_increment();

    // This may cause a segfault
    demonstrate_dangling();

    // This will cause a segfault
    demonstrate_zero();
}

/**
    Demonstrates the use of an external function to operate on two variables known to the borrow checker.
    This function uses pass-by-value
    Since we do not have to handle raw pointers, everything except calling the function is safe rust.
*/
fn demonstrate_add(){
    let a = 10;
    let b = 32;
    let sum = unsafe{ add(a,b) };
    println!("Sum is: {}", sum);
}

/**
    This function uses pass-by-reference
    The called function may as well do nasty things with the pointer but since we pass a pointer to
    `x` that is only used for the function call, we can expect `x` to be safe.
*/
fn demonstrate_increment(){
    let mut x = 1337 as u32;
    unsafe {
        increment( &mut x as *mut u32);
    }
    println!("Incremented x is: {}", x);
}

/**
    This function demonstrates the (accidential) creation of a dangling pointer.
    The `dangling` dunction returns a pointer to a local variable that is no longer valid when the
    function returns. Depending on the compiler used, the OS and its settings, this might or might
    not lead to a memory error.
    If the program does not crash, we can observe a more or less random value in the second print
*/
fn demonstrate_dangling(){
    let ptr = unsafe{ dangling() };
    println!("Ptr is {:?}",ptr);

    //kaboom ?
    unsafe{
        println!("Val at ptr is {}",*ptr);
    }
}
/**
    This function demonstrates the (incorrect) use of a null pointer.
    Although we could check if the pointer is not equal zero, it is not required that a null pointer
     points to 0x0 in all cases.
    Using a null pointer wil cause a segfault
*/
fn demonstrate_zero(){
    let ptr = unsafe{ zero() };
    println!("Ptr is {:?}",ptr);

    //kaboom
    unsafe{
        println!("Val at ptr is {:?}",*ptr);
    }
}