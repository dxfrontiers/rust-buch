#!/usr/bin/env sh

cd rust
cargo clean
cargo build --release
cd ..

cd java
mvn clean install
cd ..