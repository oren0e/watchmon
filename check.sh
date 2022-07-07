#!/bin/bash
if ! command -v rustc --version &> /dev/null
then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        exit
fi
echo "Rust is installed!"
