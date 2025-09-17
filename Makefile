# Makefile for cypher-guard Python bindings

.PHONY: all poetry-install build install clean build-python test-python build-js test-js build-rust test-rust fmt clippy clippy-all eval-rust docs docs-rust docs-python docs-js

all: build-python

help:
	@echo "Cypher Guard Build System"
	@echo "========================="
	@echo ""
	@echo "Rust targets:"
	@echo "  build-rust     - Build Rust components"
	@echo "  test-rust      - Run Rust tests"
	@echo "  fmt            - Check Rust formatting"
	@echo "  clippy         - Run Rust linting"
	@echo ""
	@echo "Python targets:"
	@echo "  build-python   - Build Python bindings"
	@echo "  test-python    - Run Python tests"
	@echo ""
	@echo "JavaScript targets:"
	@echo "  build-js       - Build JavaScript bindings"
	@echo "  test-js        - Run JavaScript tests"
	@echo ""
	@echo "Documentation targets:"
	@echo "  docs           - Generate all API documentation"
	@echo "  docs-rust      - Generate Rust API documentation"
	@echo "  docs-python    - Generate Python API documentation"
	@echo "  docs-js        - Generate JavaScript API documentation"
	@echo ""
	@echo "Evaluation targets:"
	@echo "  eval-rust      - Run evaluation with default settings"
	@echo ""
	@echo "Utility targets:"
	@echo "  load-eval-data - Load test data into Neo4j"
	@echo "  clean          - Clean all build artifacts"
	@echo "  help           - Show this help message"

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
uv-install:
	cd rust/python_bindings && uv sync

build: build-python

build-python: uv-install
	cd rust/python_bindings && uv run maturin build --release
	cd rust/python_bindings && uv pip install --force-reinstall ../../target/wheels/cypher_guard-*-cp*-*.whl

test-python: uv-install
	cd rust/python_bindings && uv run pytest tests/ -vv

# JavaScript targets
build-js:
	cd rust/js_bindings && npm install && npm run build

test-js:
	cd rust/js_bindings && npm test

# Documentation targets
docs: docs-rust docs-python docs-js
	@echo "All API documentation generated at docs/api/"

docs-rust:
	@echo "Generating Rust API documentation..."
	cd rust/cypher_guard && cargo doc --no-deps
	mkdir -p docs/api/rust
	cp -r target/doc/* docs/api/rust/
	@echo "Rust documentation generated at docs/api/rust/"

docs-python: uv-install
	@echo "Generating Python API documentation..."
	cd rust/python_bindings && uv add --dev pdoc3
	cd rust/python_bindings && uv run pdoc --html --output-dir ../../docs/api/python cypher_guard
	@echo "Python documentation generated at docs/api/python/"

docs-js:
	@echo "Generating JavaScript API documentation..."
	cd rust/js_bindings && npm ci
	cd rust/js_bindings && npm install --save-dev typedoc
	cd rust/js_bindings && npx typedoc --out ../../docs/api/javascript index.d.ts --theme default
	@echo "JavaScript documentation generated at docs/api/javascript/"

# Utility targets
install: build

clean:
	rm -rf target/
	find . -name "__pycache__" -type d -exec rm -rf {} +
	find . -name "*.so" -delete
	find . -name "node_modules" -type d -exec rm -rf {} +
	find . -name "*.node" -delete
	rm -rf docs/api/

load-eval-data:
	python3 data/ingest.py

# Evaluation targets
eval-rust: build-rust
	cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries data/queries \
		--verbose \
		--detailed
