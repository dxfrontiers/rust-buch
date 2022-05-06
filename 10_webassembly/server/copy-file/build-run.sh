# cargo wasi run --runtime-args "--dir=$PWD" -- hello.txt copied.txt # does not work at the moment
# cargo wasi build --release # does not work at the moment, either

cargo build --release --target wasm32-wasi
wasmtime --dir=$PWD --mapdir=/result::/tmp target/wasm32-wasi/release/copy-file.rustc.wasm $PWD/hello.txt /result/copied.txt