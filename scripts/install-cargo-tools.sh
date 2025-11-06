# scripts/install-cargo-tools.sh

#!/bin/bash
set -e

echo "==================================="
echo "Installing Cargo Helper Tools"
echo "==================================="
echo ""

# Array of tools to install
declare -a tools=(
    "cargo-outdated::Check for outdated dependencies"
    "cargo-audit::Security vulnerability scanner"
    "cargo-udeps::Find unused dependencies"
    "cargo-edit::Add, remove, upgrade dependencies"
    "cargo-deps::Generate dependency graphs"
    "cargo-watch::Auto-rebuild on file changes"
    "cargo-expand::Expand macros for debugging"
)

for tool_info in "${tools[@]}"; do
    IFS='::' read -r tool_name description <<< "$tool_info"
    
    echo "Installing $tool_name - $description"
    
    if [ "$tool_name" == "cargo-udeps" ]; then
        # cargo-udeps requires nightly
        cargo +nightly install "$tool_name" --locked
    else
        cargo install "$tool_name" --locked
    fi
    
    echo ""
done

echo "==================================="
echo "All tools installed successfully!"
echo "==================================="