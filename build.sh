#!/usr/bin/env bash
rustup override set $(scripts/min-tool-version.sh rustc)
rustup component add rust-src
rustup component add rustfmt
rustup component add clippy

make LLVM=1 rustavailable
make LLVM=1 -j `nproc`
