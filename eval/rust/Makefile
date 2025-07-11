# Cypher Guard Evaluation Tool Makefile

.PHONY: help build eval eval-verbose eval-detailed clean test

# Default target
help:
	@echo "Cypher Guard Evaluation Tool"
	@echo "============================"
	@echo ""
	@echo "Available targets:"
	@echo "  build         - Build the evaluation tool"
	@echo "  eval          - Run evaluation with default settings"
	@echo "  eval-verbose  - Run evaluation with verbose output"
	@echo "  eval-detailed - Run evaluation with detailed results"
	@echo "  eval-summary  - Run evaluation and show only summary"
	@echo "  test          - Run evaluation on test queries only"
	@echo "  clean         - Clean build artifacts"
	@echo "  help          - Show this help message"

# Build the evaluation tool
build:
	cd ../../ && cargo build --bin eval

# Run evaluation with default settings
eval: build
	cd ../../ && cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries data/queries

# Run evaluation with verbose output
eval-verbose: build
	cd ../../ && cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries data/queries \
		--verbose

# Run evaluation with detailed results
eval-detailed: build
	cd ../../ && cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries data/queries \
		--detailed

# Run evaluation with both verbose and detailed output
eval-full: build
	cd ../../ && cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries data/queries \
		--verbose \
		--detailed

# Run evaluation and show only summary (pipe to filter)
eval-summary: build
	cd ../../ && cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries data/queries 2>/dev/null | grep -A 20 "EVALUATION SUMMARY"

# Test only valid queries
test-valid: build
	cd ../../ && cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries data/queries/valid \
		--verbose

# Test only simple queries
test-simple: build
	cd ../../ && cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries data/queries/valid/simple.yml \
		--detailed

# Clean build artifacts
clean:
	cd ../../ && cargo clean

# Run with custom schema
eval-custom-schema: build
	@echo "Usage: make eval-custom-schema SCHEMA=path/to/schema.json"
	@if [ -z "$(SCHEMA)" ]; then \
		echo "Error: SCHEMA variable not set"; \
		exit 1; \
	fi
	cd ../../ && cargo run --bin eval -- \
		--schema $(SCHEMA) \
		--queries data/queries \
		--verbose

# Run with custom queries directory
eval-custom-queries: build
	@echo "Usage: make eval-custom-queries QUERIES=path/to/queries"
	@if [ -z "$(QUERIES)" ]; then \
		echo "Error: QUERIES variable not set"; \
		exit 1; \
	fi
	cd ../../ && cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries $(QUERIES) \
		--verbose

# Performance benchmark
benchmark: build
	@echo "Running performance benchmark..."
	cd ../../ && time cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries data/queries 2>&1 | \
		grep -E "(Files processed|Total queries|Accuracy)"

# Continuous integration mode
ci: build
	cd ../../ && cargo run --bin eval -- \
		--schema data/schema/eval_schema.json \
		--queries data/queries > eval_results.txt
	@if grep -q "🎯 Accuracy: [0-9][0-9]%" eval_results.txt; then \
		echo "✅ Evaluation passed"; \
		exit 0; \
	else \
		echo "❌ Evaluation failed - accuracy too low"; \
		cat eval_results.txt; \
		exit 1; \
	fi 