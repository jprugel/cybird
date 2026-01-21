# use PowerShell instead of sh:

set shell := ["powershell.exe", "-c"]

rebuild:
    cargo clean
    cargo build --package upgrade3 --features dynamic
    cargo run --bin runner

build:
    cargo build --package upgrade3 --features dynamic
