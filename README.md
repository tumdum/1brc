# 1BRC: One Billion Row Challenge in Rust

## Creating the measurements file with 1B rows

First, the generator has to be build.
```shell
cargo build --bin gen-examples --features=random --release
```

Then, the generator can be run to create the measurements file.
```shell
cargo run --bin gen-examples --features=random --release -- <MAX_NUMBER_OF_CITIES> <NUMBER_OF_ROWS> > measurements.txt
```

Be patient as it can take more than a minute to have the file generated.
