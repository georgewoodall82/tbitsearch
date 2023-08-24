#!/bin/bash

cargo build --target x86_64-unknown-linux-gnu --release
cargo build --target x86_64-pc-windows-gnu --release
cargo build --target aarch64-unknown-linux-gnu --release
cargo build --target armv7-unknown-linux-gnueabihf --release

mkdir ./target/output

mv -f ./target/x86_64-unknown-linux-gnu/release/tbitsearch ./target/output/tbitsearch-linux-x86_64
mv -f ./target/x86_64-pc-windows-gnu/release/tbitsearch.exe ./target/output/tbitsearch-windows-x86_64.exe
mv -f ./target/aarch64-unknown-linux-gnu/release/tbitsearch ./target/output/tbitsearch-linux-aarch64
mv -f ./target/armv7-unknown-linux-gnueabihf/release/tbitsearch ./target/output/tbitsearch-linux-armv7
