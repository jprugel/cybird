# use PowerShell instead of sh:

set shell := ["powershell.exe", "-c"]

build:
    cargo build --package crazier-crab --features dynamic

run:
    cargo run --bin crypto-crab
