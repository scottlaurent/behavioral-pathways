# Makefile for Behavioral Pathways

.PHONY: help test tests longitudinal-tests fmt lint check build clean

# Default target
help:
	@echo "Behavioral Pathways - Development"
	@echo ""
	@echo "Common tasks:"
	@echo "  make test       - Run tests (like phpunit)"
	@echo "  make tests      - Run tests with coverage (like phpunit --coverage-text)"
	@echo "  make longitudinal-tests - Run ignored longitudinal tests"
	@echo "  make fmt        - Format code (like php-cs-fixer)"
	@echo "  make lint       - Lint code (like phpstan)"
	@echo "  make check      - Quick compile check"
	@echo "  make build      - Build release"
	@echo "  make clean      - Clean build artifacts"

# Run tests (like phpunit)
test:
	cargo nextest run

# Run tests with coverage (like phpunit --coverage-text)
tests:
	cargo llvm-cov nextest

# Run ignored longitudinal tests
longitudinal-tests:
	cargo test --test longitudinal -- --ignored --nocapture

# Format code (like php-cs-fixer fix)
fmt:
	cargo fmt

# Lint code (like phpstan analyze)
lint:
	cargo clippy -- -D warnings

# Quick compile check (faster than full build)
check:
	cargo check

# Run all checks before commit
check-all: fmt lint test
	@echo "All checks passed!"

# Build release
build:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean
