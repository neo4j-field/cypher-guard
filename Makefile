# Makefile for cypher-guard Python bindings

.PHONY: all poetry-install build install clean build-python test-python build-js test-js build-rust test-rust fmt clippy clippy-all

all: build-python

# Rust targets (matches CI)
test-rust: fmt clippy build
	cargo test --verbose

fmt:
	cargo fmt --all -- --check

clippy:
	cd rust/cypher_guard && cargo clippy -- -D warnings -A clippy::uninlined_format_args

clippy-all:
	cargo clippy -- -D warnings -A clippy::uninlined_format_args

build-rust:
	cargo build --verbose

# Python targets
poetry-install:
	cd rust/python_bindings && poetry install

build: build-python

build-python: poetry-install
	cd rust/python_bindings && poetry run maturin develop

test-python: poetry-install
	cd rust/python_bindings && poetry run pytest tests/ -vv

# JavaScript targets
build-js:
	cd rust/js_bindings && npm install && npm run build

test-js:
	cd rust/js_bindings && npm test

# Utility targets
install: build

clean:
	rm -rf target/
	find . -name "__pycache__" -type d -exec rm -rf {} +
	find . -name "*.so" -delete
	find . -name "node_modules" -type d -exec rm -rf {} +
	find . -name "*.node" -delete 