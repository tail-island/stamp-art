@echo off

set RUSTFLAGS=-C codegen-units=1 -C panic=abort -C target-cpu=native
cargo build --release
