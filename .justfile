# use PowerShell instead of sh:

set shell := ["powershell.exe", "-c"]

build:
    cargo build --package upgrade3 --features dynamic

run:
    cargo run --bin runner
