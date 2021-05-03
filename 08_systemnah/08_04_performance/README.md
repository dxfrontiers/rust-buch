# Example for `Kapitel 8.4: Systemnahe Programmierung, Performance`



## Running

We use several tools to analyze performance of the application.
Please make sure to have the tools listed below installed.

### Prerequisites
- [heaptrack](https://github.com/KDE/heaptrack), e.g. sudo pacman -S heaptrack
- Flamegraph (via cargo) to generate a svg containing a flamegraph `cargo install flamegraph`

### Compile and run
- Benchmarks (uses criterion): `cargo bench`
- Flamegraphs for the recursive implementation `cargo flamegraph -b sort_recursive_runner`
- Heap allocation statistics
  - Build optimized executables `cargo build -- release`
  - Run Heaptrack for each implementation 
    - Make sure you have a few hundred MB of free space on your disk and be prepared for your fans to spin up
    - `heaptrack target/release/sort_recursive_runner`
    - `heaptrack target/release/sort_iterative_runner`