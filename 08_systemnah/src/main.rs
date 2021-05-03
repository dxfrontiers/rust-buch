use libc::size_t;

#[link(name = "ext")]
extern "C" {
    fn add(a: size_t, b: size_t,) -> size_t;
    fn increment(target: *mut u32);
    fn dangling() -> *mut u32;
    fn zero() -> *mut u32;
}

fn main() {
    let a = 10;
    let b = 32;

    let sum = unsafe{ add(a,b) };
    println!("Sum is: {}", sum);

    let mut x = 1337 as u32;
    unsafe {
        increment( &mut x as *mut u32);
    }
    println!("Incremented x is: {}", x);


    unsafe{
        let mut ptr = dangling();
        println!("Ptr is {:?}",ptr);
        //kaboom
        println!("Val at ptr is {}",*ptr);
    }

    unsafe{
        let mut ptr = zero();
        println!("Ptr is {:?}",ptr);
        //kaboom
        println!("Val at ptr is {:?}",*ptr);
    }


}