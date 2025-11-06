# Makefile

.PHONY: help deps-check deps-update deps-audit deps-clean deps-install-tools

help:
	@echo "Available commands:"
	@echo ""
	@echo "Build & Run:"
	@echo "  make build              - Build Docker image"
	@echo "  make run                - Run bot container"
	@echo "  make stop               - Stop and remove bot container"
	@echo "  make logs               - View bot logs"
	@echo ""
	@echo "Dependency Management:"
	@echo "  make deps-install-tools - Install cargo helper tools"
	@echo "  make deps-check         - Check for outdated/vulnerable dependencies"
	@echo "  make deps-update        - Update dependencies (compatible versions)"
	@echo "  make deps-upgrade       - Update dependencies (latest versions)"
	@echo "  make deps-audit         - Check for security vulnerabilities"
	@echo "  make deps-tree          - Show dependency tree"
	@echo "  make deps-clean         - Remove Cargo.lock and rebuild"
	@echo ""
	@echo "Development:"
	@echo "  make test               - Run tests"
	@echo "  make clippy             - Run clippy linter"
	@echo "  make fmt                - Format code"
	@echo "  make check              - Run all checks (fmt, clippy, test)"

# Install cargo helper tools
deps-install-tools:
	@chmod +x scripts/install-cargo-tools.sh
	@./scripts/install-cargo-tools.sh

# Check dependencies for updates and vulnerabilities
deps-check:
	@chmod +x scripts/check-updates.sh
	@./scripts/check-updates.sh

# Update dependencies (compatible versions only)
deps-update:
	@chmod +x scripts/update-deps.sh
	@./scripts/update-deps.sh compatible

# Update dependencies to latest versions (may break)
deps-upgrade:
	@chmod +x scripts/update-deps.sh
	@./scripts/update-deps.sh latest

# Security audit only
deps-audit:
	@cargo audit

# Show dependency tree
deps-tree:
	@cargo tree

# Clean and regenerate Cargo.lock
deps-clean:
	@rm -f Cargo.lock
	@cargo update
	@echo "Cargo.lock regenerated"

# Development commands
test:
	@cargo test

clippy:
	@cargo clippy -- -D warnings

fmt:
	@cargo fmt --all

check: fmt clippy test
	@echo "All checks passed!"