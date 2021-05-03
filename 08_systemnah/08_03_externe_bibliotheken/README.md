# Example for `Kapitel 8.3: Systemnahe Programmierung, Externe Bibliotheken`

This rust crate contains several examples that showcase how code of other languages can be used in Rust.
In this case we use the C language to demonstrate the interoperability and the use of unsafe.

## Running

Since we need to have artifacts available, compiled from C, this examples require a bit of setup.

### Prerequisites
We use make and GCC to build the native libraries.
Please make sure you have a working toolchain.
If you are on linux, making sure that `make` und `gcc` are installed will most likely be enough.
On Windows, we recomend a mingw installation. Please refer to the official docs on how to achieve this

### Compiling the native code
The native code lives in the directory `./include`. 
You can execute `make` from there if installed, otherwise simply execute the contents of the makefile by hand.
This will create two files, `ext.o` and `libext.a`. 
Whilst `ext.o` does no harm, you can safely delete it, since we will only use `libext.a`.

Rust will find `libext.a` through a combination of two mechanisms:
- In the `build.rs` file, we added `./include` to the link-search path.
- In the `main.rs` file, we specified the name of the linked library as `ext`.

### Compile and run Rust
As you are used to by now, simply run `cargo run` to compile and run the code.
Be aware that the code will intentionally segfault after printing some things.
