# scripts/check-updates.sh

#!/bin/bash
set -e

echo "==================================="
echo "Cargo Dependency Update Check"
echo "==================================="
echo ""

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# 1. Check for outdated dependencies
echo -e "${YELLOW}[1/5] Checking for outdated dependencies...${NC}"
if command_exists cargo-outdated; then
    cargo outdated
else
    echo -e "${RED}cargo-outdated not installed. Install with: cargo install cargo-outdated${NC}"
fi
echo ""

# 2. Check for security vulnerabilities
echo -e "${YELLOW}[2/5] Checking for security vulnerabilities...${NC}"
if command_exists cargo-audit; then
    cargo audit
else
    echo -e "${RED}cargo-audit not installed. Install with: cargo install cargo-audit${NC}"
fi
echo ""

# 3. Check for unused dependencies
echo -e "${YELLOW}[3/5] Checking for unused dependencies...${NC}"
if command_exists cargo-udeps; then
    cargo +nightly udeps
else
    echo -e "${RED}cargo-udeps not installed. Install with: cargo install cargo-udeps --locked${NC}"
fi
echo ""

# 4. Check dependency tree for duplicates
echo -e "${YELLOW}[4/5] Checking for duplicate dependencies...${NC}"
cargo tree --duplicates
echo ""

# 5. Generate dependency graph (if graphviz installed)
echo -e "${YELLOW}[5/5] Generating dependency graph...${NC}"
if command_exists cargo-deps && command_exists dot; then
    cargo deps --all-deps | dot -Tpng > dependency-graph.png
    echo -e "${GREEN}Dependency graph saved to dependency-graph.png${NC}"
else
    echo -e "${RED}cargo-deps or graphviz not installed.${NC}"
    echo "Install with: cargo install cargo-deps && apt-get install graphviz"
fi
echo ""

echo -e "${GREEN}==================================="
echo "Check complete!"
echo "===================================${NC}"