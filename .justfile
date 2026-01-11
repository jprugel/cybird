# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

rebuild:
    cargo clean
    cargo build --package iron-sword --features dynamic
    cargo run --bin runner
