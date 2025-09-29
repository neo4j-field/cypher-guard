# Makefile for cypher-guard Python bindings

.PHONY: all poetry-install build install clean build-python test-python build-js test-js build-rust test-rust fmt clippy clippy-all eval-rust docs docs-rust docs-python docs-js release-notes

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
	@echo "  lint-js        - Lint JavaScript/TypeScript code"
	@echo "  fmt-js         - Format JavaScript/TypeScript code"
	@echo "  fmt-js-check   - Check JavaScript/TypeScript formatting"
	@echo ""
	@echo "Documentation targets:"
	@echo "  docs           - Generate all API documentation"
	@echo "  docs-rust      - Generate Rust API documentation"
	@echo ""
	@echo "Release targets:"
	@echo "  release-notes  - Generate release notes from merged PRs"
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
	uv sync --no-install-project

build: build-python

build-python: pre-clean-for-python-build
	uv sync --no-install-project
	uv run maturin build --release
	uv pip install --reinstall-package cypher-guard target/wheels/cypher_guard-*.whl

build-python-dev: pre-clean-for-python-build
	maturin develop --release

pre-clean-for-python-build:
	cargo clean
	rm -rf target/
	rm -rf dist/
	rm -rf *.egg-info
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete
	find . -type f -name "*.so" -delete


test-python:
	uv run --no-sync pytest rust/python_bindings/tests/ -vv

test-python-unit:
	uv run --no-sync pytest rust/python_bindings/tests/unit/ -vv

# JavaScript targets
build-js:
	cd rust/js_bindings && npm install && npm run build

test-js:
	cd rust/js_bindings && npm test

lint-js:
	cd rust/js_bindings && npm run lint

lint-js-fix:
	cd rust/js_bindings && npm run lint:fix

fmt-js:
	cd rust/js_bindings && npm run format

fmt-js-check:
	cd rust/js_bindings && npm run format:check

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
	uv add --dev pdoc3
	uv run pdoc --html --output-dir docs/api/python cypher_guard
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

# Release targets
release-notes:
	@echo "Generating release notes from merged PRs..."
	@PREVIOUS_TAG=$$(git describe --tags --abbrev=0 HEAD~1 2>/dev/null || echo ""); \
	if [ -n "$$PREVIOUS_TAG" ]; then \
		PRS=$$(gh pr list --state merged --search "merged:>$$PREVIOUS_TAG" --json number,title,body,labels); \
	else \
		PRS=$$(gh pr list --state merged --json number,title,body,labels); \
	fi; \
	echo "# Release Notes"; \
	echo ""; \
	echo "## What's New"; \
	echo ""; \
	echo "$$PRS" | jq -r '.[] | "### PR #\(.number): \(.title)\n\n\(.body // "No description provided")\n"'; \
	echo "## Detailed Changes"; \
	echo ""; \
	echo "$$PRS" | jq -r '.[] | select(.body | contains("## Release Notes")) | .body' | \
		sed -n '/## Release Notes/,/##/p' | \
		sed '1d;$$d' || echo "No detailed release notes found in PR descriptions."

update-changelog:
	@echo "Updating CHANGELOG.md with recent PRs..."
	@PREVIOUS_TAG=$$(git describe --tags --abbrev=0 HEAD~1 2>/dev/null || echo ""); \
	if [ -n "$$PREVIOUS_TAG" ]; then \
		PRS=$$(gh pr list --state merged --search "merged:>$$PREVIOUS_TAG" --json number,title,body,labels); \
	else \
		PRS=$$(gh pr list --state merged --json number,title,body,labels); \
	fi; \
	echo "### Added" > temp_changelog.md; \
	echo "$$PRS" | jq -r '.[] | select(.body | contains("Type of Change") and contains("New feature")) | "- \(.title) (#\(.number))"' >> temp_changelog.md; \
	echo "" >> temp_changelog.md; \
	echo "### Changed" >> temp_changelog.md; \
	echo "$$PRS" | jq -r '.[] | select(.body | contains("Type of Change") and (contains("Bug fix") or contains("Documentation update") or contains("Performance improvement"))) | "- \(.title) (#\(.number))"' >> temp_changelog.md; \
	echo "" >> temp_changelog.md; \
	echo "### Fixed" >> temp_changelog.md; \
	echo "$$PRS" | jq -r '.[] | select(.body | contains("Type of Change") and contains("Bug fix")) | "- \(.title) (#\(.number))"' >> temp_changelog.md; \
	echo ""; \
	echo "Add this to CHANGELOG.md under [Unreleased]:"; \
	cat temp_changelog.md; \
	rm temp_changelog.md
