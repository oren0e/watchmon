#!/bin/bash
if ! command -v rustc --version &> /dev/null
then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        exit
fi
cargo build --release
chmod +x ./target/release/watchmon
cp ./target/release/watchmon /usr/local/bin/
