use j4rs::prelude::*;
use j4rs_derive::*;

#[call_from_java("de.digitalfrontiers.rustbuch.dash.RustDasher.printHello")]
fn my_function_with_no_args() {
    println!("Hello from the Rust world!");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
