
/*
    We expect the built library to be located in ./include
    If you have not already, build it with `make` in ./include
 */
fn main() {
    println!(r"cargo:rustc-link-search=include");
}