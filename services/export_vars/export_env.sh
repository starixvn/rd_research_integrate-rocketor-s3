#!/bin/sh

# Run the Rust program and capture its output
export_commands=$(export_vars)

# Source the export commands
eval "$export_commands"