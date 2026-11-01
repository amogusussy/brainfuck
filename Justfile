alias b := build

build:
    export RUST_BACKTRACE=1
    cargo build --release
