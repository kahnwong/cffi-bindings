#!/bin/sh

cd bindings/rust || exit
make build-so
cargo build --release

./target/release/cffi
