# Development Guide

This guide covers local development, building from source, and contributing to Cypher Guard.

## Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Python**: 3.8+ with `pip` or `uv`
- **Node.js**: 16+ with `npm`
- **Git**: For cloning the repository

## Local Setup

### 1. Clone Repository

```bash
git clone <repo-url>
cd cypher-guard
```

### 2. Build Python Bindings

```bash
# Install uv (recommended)
pip install uv

# Build and install Python bindings
make build-python

# Or manually
cd rust/python_bindings
uv run maturin develop
```

### 3. Build JavaScript Bindings

```bash
# Build JS/TS bindings
make build-js

# Or manually
cd rust/js_bindings
npm install
npm run build
```

### 4. Build Rust Library

```bash
# Build Rust library
make build-rust

# Or manually
cargo build --release
```

## Build Commands

The Makefile provides convenient shortcuts for all build operations:

| Command | Description |
|---------|-------------|
| `make` or `make build` | Build and install Python extension using uv and maturin |
| `make build-python` | Build and install Python extension (`uv run maturin develop`) |
| `make build-js` | Install and build JS/TS bindings (`npm install && npm run build`) |
| `make build-rust` | Build the Rust library (`cargo build`) |
| `make clean` | Remove build artifacts, Python caches, and node modules |
| `make install` | Alias for `make build` |

## Testing

### Run All Tests

```bash
# Run all test suites
make test-rust
make test-python
make test-js
```

### Individual Test Suites

#### Rust Tests

```bash
# Run Rust tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

#### Python Tests

```bash
# Run Python tests
make test-python

# Or manually
cd rust/python_bindings
uv run pytest

# Run specific test file
uv run pytest tests/unit/test_validation.py

# Run with verbose output
uv run pytest -v
```

#### JavaScript Tests

```bash
# Run JS/TS tests
make test-js

# Or manually
cd rust/js_bindings
npm test
```

## Code Quality

### Formatting

```bash
# Check formatting
make fmt

# Format code
cargo fmt --all
```

### Linting

```bash
# Run clippy on main crate
make clippy

# Run clippy on all crates
make clippy-all

# Or manually
cargo clippy -- -D warnings -A clippy::uninlined_format_args
```

## Troubleshooting

### UV Caching Issues on macOS

If you encounter issues where code changes aren't reflected after rebuilding (e.g., old debug output persisting, functions not updating), this is due to a known UV bug on macOS where `.so` files aren't properly updated.

**Symptoms:**
- Old debug output or panics appearing despite source code changes
- New functions not available after rebuild
- `maturin develop` appears to succeed but changes aren't reflected

**Solutions:**

1. **First try**: Clean rebuild
   ```bash
   make clean && make build-python
   ```

2. **If that fails**: The Makefile uses `uv pip install --force-reinstall` which should handle most caching issues, but if you still have problems:
   ```bash
   # Remove virtual environment and rebuild completely
   rm -rf rust/python_bindings/.venv
   make build-python
   ```

3. **Last resort**: Bump version numbers in `Cargo.toml` and `pyproject.toml` files to force complete recompilation.

**Note**: This issue is specific to UV on macOS and doesn't affect production deployments or other platforms.

### Common Build Issues

#### Python Bindings Won't Build

```bash
# Ensure Rust toolchain is installed
rustup show

# Clean and rebuild
make clean
make build-python

# Check Python version compatibility
python --version  # Should be 3.8+
```

#### JavaScript Bindings Won't Build

```bash
# Ensure Node.js version is compatible
node --version  # Should be 16+

# Clean and rebuild
make clean
make build-js

# Check npm cache
npm cache clean --force
```

#### Rust Compilation Errors

```bash
# Update Rust toolchain
rustup update

# Check for clippy warnings
cargo clippy

# Clean build artifacts
cargo clean
cargo build
```

## Project Structure

```
cypher-guard/
├── rust/
│   ├── cypher_guard/           # Main Rust library
│   │   ├── src/
│   │   │   ├── lib.rs          # Public API
│   │   │   ├── parser/         # Cypher parsing modules
│   │   │   ├── schema.rs       # Schema structure
│   │   │   ├── validation.rs   # Query validation
│   │   │   └── errors.rs       # Error types
│   │   └── Cargo.toml
│   ├── python_bindings/        # Python bindings
│   │   ├── src/lib.rs          # Python API
│   │   ├── tests/              # Python tests
│   │   └── pyproject.toml
│   └── js_bindings/           # JavaScript bindings
│       ├── src/lib.rs         # JS/TS API
│       ├── test.ts            # JS/TS tests
│       └── package.json
├── docs/                      # Documentation
├── assets/                    # Images and assets
└── Makefile                   # Build automation
```

## Contributing

### Development Workflow

1. **Fork and clone** the repository
2. **Create a feature branch**: `git checkout -b feature/your-feature`
3. **Make changes** and test locally
4. **Run all tests**: `make test-rust && make test-python && make test-js`
5. **Check code quality**: `make fmt && make clippy`
6. **Commit changes**: `git commit -m "Add your feature"`
7. **Push and create PR**: `git push origin feature/your-feature`

### Code Standards

- **Rust**: Follow standard Rust conventions, use `cargo fmt` and `cargo clippy`
- **Python**: Follow PEP 8, use type hints where appropriate
- **JavaScript**: Follow standard JS/TS conventions
- **Tests**: Add tests for new functionality
- **Documentation**: Update docs for API changes

### Release Process

See [Versioning Guide](VERSIONING.md) for detailed release management.

## Performance Testing

### Benchmarking

```bash
# Run Rust benchmarks
cargo bench

# Profile Python performance
cd rust/python_bindings
uv run python -m cProfile -s cumtime your_script.py
```

### Memory Usage

```bash
# Check memory usage in Rust
cargo build --release
valgrind --tool=massif ./target/release/your_binary

# Python memory profiling
uv run pip install memory-profiler
uv run python -m memory_profiler your_script.py
```
