# Makefile for IP Counter Service

.PHONY: help build test test-unit test-integration bench clean run dev docker-build docker-run fmt lint check

# Default target
help:
	@echo "Available targets:"
	@echo "  build           - Build the project in release mode"
	@echo "  test            - Run all tests"
	@echo "  test-unit       - Run unit tests only"
	@echo "  test-integration - Run integration tests only"
	@echo "  bench           - Run benchmarks"
	@echo "  clean           - Clean build artifacts"
	@echo "  run             - Run the service"
	@echo "  dev             - Run in development mode with auto-reload"
	@echo "  docker-build    - Build Docker image"
	@echo "  docker-run      - Run Docker container"
	@echo "  fmt             - Format code"
	@echo "  lint            - Run clippy linter"
	@echo "  check           - Run fmt, lint, and tests"

# Build the project
build:
	cargo build --release

# Run all tests
test:
	cargo test --all-features

# Run unit tests only
test-unit:
	cargo test --lib --bins

# Run integration tests only
test-integration:
	cargo test --test '*'

# Run benchmarks
bench:
	cargo bench

# Clean build artifacts
clean:
	cargo clean
	rm -rf target/

# Run the service
run: build
	RUST_LOG=info ./target/release/unique_ip_counter

# Development mode with cargo-watch
dev:
	cargo watch -x 'run' -w src/

# Docker build
docker-build:
	docker build -t unique_ip_counter:latest .

# Docker run
docker-run:
	docker run -p 5000:5000 -p 9102:9102 unique_ip_counter:latest

# Format code
fmt:
	cargo fmt --all

# Lint code
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Run all checks
check: fmt lint test

# Load test using wrk (requires wrk to be installed)
load-test:
	@echo "Starting load test..."
	@echo '{"timestamp":"2024-01-01T00:00:00Z","ip":"192.168.1.1"}' > /tmp/test.json
	wrk -t12 -c400 -d30s -s scripts/post.lua http://localhost:5000/logs

# Coverage report (requires cargo-tarpaulin)
coverage:
	cargo tarpaulin --out Html --output-dir target/coverage

# Security audit
audit:
	cargo audit

# Documentation
docs:
	cargo doc --no-deps --open

# Install development dependencies
install-dev-deps:
	cargo install cargo-watch cargo-tarpaulin cargo-audit

# CI pipeline simulation
ci: check audit
	@echo "CI checks passed!"
