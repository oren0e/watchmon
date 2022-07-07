# Overview

A command line utility to watch and monitor for the existence of a specific term in the contents of a file. If the term is not found in the file the specified bash command gets executed.

# Installation

Currently the installation is done by:

1. Cloning this repo
2. Checking if Rust is installed: `make check_rust`. If rust is not installed this will download and install it
3. Running `make install`

A `crates.io` release is planned.

# Usage

Run `watchmon --file <FILE PATH> --text-term <TEXT TERM> --command <BASH COMMAND>`. The program will run forever. For more info run `watchmon --help`.

# Contribute

You are more than welcome to make a contribution and open issues. To make a contribution please open a PR and make sure the tests and CI pass before requesting a code review.
