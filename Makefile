.PHONY: help test test-lexer test-parser test-codegen build check fmt fmt-check clippy clean demo dev

help:
	@echo "jRust Development Commands"
	@echo "=========================="
	@echo ""
	@echo "Testing:"
	@echo "  make test         - Run all tests (46 total)"
	@echo "  make test-lexer   - Run lexer tests only (13 tests)"
	@echo "  make test-parser  - Run parser tests only (15 tests)"
	@echo "  make test-codegen - Run codegen tests only (13 tests)"
	@echo ""
	@echo "Building:"
	@echo "  make build        - Build all crates"
	@echo "  make check        - Check code without building"
	@echo "  make demo         - Run transpiler demo"
	@echo ""
	@echo "Code Quality:"
	@echo "  make fmt          - Format all code"
	@echo "  make fmt-check    - Check formatting"
	@echo "  make clippy       - Run clippy linter"
	@echo ""
	@echo "Maintenance:"
	@echo "  make clean        - Remove build artifacts"
	@echo "  make dev          - Watch mode (rebuild on changes)"

test:
	cargo test --all

test-lexer:
	cargo test --test lexer_tests

test-parser:
	cargo test --test parser_tests

test-codegen:
	cargo test --test codegen_tests

build:
	cargo build --all

check:
	cargo check --all

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all

clean:
	cargo clean

demo:
	cargo run -q --bin jrust

dev:
	cargo watch -x build -x test
