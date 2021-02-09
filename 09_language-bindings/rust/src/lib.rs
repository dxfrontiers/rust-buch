use j4rs::prelude::*;
use j4rs_derive::*;

#[call_from_java("de.digitalfrontiers.rustbuch.dash.RustDasher.printHello")]
fn print_hello() {
    println!("Hello from the Rust world!");
}

#[call_from_java("de.digitalfrontiers.rustbuch.dash.RustDasher.printHelloWithName")]
fn print_hello_with_name(i: Instance) {
    let jvm: Jvm = Jvm::attach_thread().unwrap();
    let name = match jvm.to_rust(i) {
        Ok(t) => t,
        Err(e) => e.to_string()
    };
    println!("Hello from the Rust world, my dearest {}!", name);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
