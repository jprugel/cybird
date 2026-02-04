#!/usr/bin/env nu

# Main function that handles the task selection
def main [task?: string = "help"] {
    match $task {
        "build" => {
            print "Building crazier-crab with dynamic features..."
            cargo build --package crazier-crab --features dynamic
        }
        "run" => {
            print "Running crypto-crab..."
            cargo run --bin crypto-crab
        }
        "help" => {
            print "Available tasks:"
            print "  build - Build the crazier-crab package with dynamic features"
            print "  run   - Run the crypto-crab binary"
            print ""
            print "Usage: nu tasks.nu <task>"
        }
        _ => {
            print $"Unknown task: ($task)"
            print "Run 'nu tasks.nu help' for available tasks"
        }
    }
}
