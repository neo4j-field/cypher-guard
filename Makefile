# Makefile for cypher-guard Python bindings

.PHONY: all build install clean build-python test-python build-js test-js

all: build

install-maturin:
	pip install maturin

build: install-maturin
	maturin develop

build-python:
	cd rust/python_bindings && maturin develop

test-python:
	pytest rust/python_bindings/tests/ -vv

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