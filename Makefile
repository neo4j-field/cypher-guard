# Makefile for cypher-guard Python bindings

.PHONY: all build install clean

all: build

install-maturin:
	pip install maturin

build: install-maturin
	maturin develop

clean:
	rm -rf target/
	find . -name "__pycache__" -type d -exec rm -rf {} +
	find . -name "*.so" -delete 