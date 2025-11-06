# scripts/update-deps.sh

#!/bin/bash
set -e

echo "==================================="
echo "Cargo Dependency Update"
echo "==================================="
echo ""

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Parse command line arguments
UPDATE_MODE="${1:-compatible}"  # compatible, latest, or specific

case "$UPDATE_MODE" in
    compatible)
        echo -e "${YELLOW}Updating to latest compatible versions (respects Cargo.toml constraints)...${NC}"
        cargo update
        ;;
    latest)
        echo -e "${YELLOW}Updating to latest versions (may break compatibility)...${NC}"
        if command -v cargo-edit >/dev/null 2>&1; then
            cargo upgrade
        else
            echo -e "${RED}cargo-upgrade not installed. Install with: cargo install cargo-edit${NC}"
            exit 1
        fi
        ;;
    specific)
        if [ -z "$2" ]; then
            echo -e "${RED}Error: Please specify package name${NC}"
            echo "Usage: $0 specific <package-name>"
            exit 1
        fi
        echo -e "${YELLOW}Updating $2 to latest compatible version...${NC}"
        cargo update -p "$2"
        ;;
    *)
        echo -e "${RED}Invalid mode: $UPDATE_MODE${NC}"
        echo "Usage: $0 [compatible|latest|specific <package>]"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}Running tests to verify compatibility...${NC}"
cargo test

echo ""
echo -e "${GREEN}Running clippy to check for issues...${NC}"
cargo clippy -- -D warnings

echo ""
echo -e "${GREEN}Update complete! Review Cargo.lock changes before committing.${NC}"