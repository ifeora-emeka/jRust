.PHONY: help test test-lexer test-parser test-codegen build check fmt fmt-check clippy clean demo cli-help cli-init cli-check cli-build cli-run

help:
	@echo "jRust Development Commands"
	@echo "=========================="
	@echo ""
	@echo "Testing:"
	@echo "  make test         - Run all tests (95 total)"
	@echo "  make test-lexer   - Run lexer tests only (19 tests)"
	@echo "  make test-parser  - Run parser tests only (26 tests)"
	@echo "  make test-codegen - Run codegen tests only (29 tests)"
	@echo ""
	@echo "Building:"
	@echo "  make build        - Build all crates"
	@echo "  make check        - Check code without building"
	@echo "  make demo         - Run transpiler demo (jrust-demo binary)"
	@echo ""
	@echo "CLI Testing:"
	@echo "  make cli-help     - Show jrust CLI help"
	@echo "  make cli-init     - Test jrust init command"
	@echo "  make cli-check    - Test jrust check command"
	@echo "  make cli-build    - Test jrust build command"
	@echo "  make cli-run      - Test jrust run command"
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
	cargo run --bin jrust-demo

cli-help:
	cargo run --bin jrust -- --help

cli-init:
	cargo run --bin jrust -- init test-project

cli-check:
	@if [ -f "test-project/src/index.jr" ]; then \
		cd test-project && cargo run --bin jrust -- check; \
	else \
		echo "Please run 'make cli-init' first to create test-project"; \
	fi

cli-build:
	@if [ -f "test-project/src/index.jr" ]; then \
		cd test-project && cargo run --bin jrust -- build; \
	else \
		echo "Please run 'make cli-init' first to create test-project"; \
	fi

cli-run:
	@if [ -f "test-project/src/index.jr" ]; then \
		cd test-project && cargo run --bin jrust -- run; \
	else \
		echo "Please run 'make cli-init' first to create test-project"; \
	fi

dev:
	cargo watch -x build -x test
