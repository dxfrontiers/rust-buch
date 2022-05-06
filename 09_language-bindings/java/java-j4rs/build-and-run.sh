#!/usr/bin/env sh

set -e

echo "Building... "
./build.sh &> /dev/null

lib_path="${PWD}/rust/target/release"
jar_path="${PWD}/java/target/j4rs-1.0-SNAPSHOT-jar-with-dependencies.jar"

java -Djava.library.path=$lib_path -jar $jar_path
