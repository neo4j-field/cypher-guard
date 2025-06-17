# Makefile for cypher-guard Python bindings

.PHONY: all poetry-install build install clean build-python test-python build-js test-js build-rust

all: build

poetry-install:
	poetry install

build: poetry-install
	poetry run maturin develop

build-python: poetry-install
	cd rust/python_bindings && poetry run maturin develop

test-python: poetry-install
	poetry run pytest rust/python_bindings/tests/ -vv

build-js:
	cd rust/js_bindings && npm install && npm run build

test-js:
	cd rust/js_bindings && npm test

clean:
	rm -rf target/
	find . -name "__pycache__" -type d -exec rm -rf {} +
	find . -name "*.so" -delete
	find . -name "node_modules" -type d -exec rm -rf {} +
	find . -name "*.node" -delete 