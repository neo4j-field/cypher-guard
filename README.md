# cypher-guard

Cypher Guard is a Rust library and CLI tool for validating Cypher queries against a user-defined schema. It supports both Rust and Python integration via PyO3 and maturin.

## Features
- Validate Cypher query syntax
- Validate node labels, relationship types, and properties against a schema
- JSON-based schema loading
- Python bindings via PyO3

## Installation

### Rust
Clone the repository and build with Cargo:
```sh
cargo build --release
```

### Python
Build and install the Python module using maturin:
```sh
pip install maturin
cd cypher-guard
maturin develop
```

## Usage

### CLI
You can use the CLI to validate Cypher queries:
```sh
echo "MATCH (n:Person) RETURN n" | cargo run --bin cypher-guard
```

### Library
Add `cypher-guard` as a dependency in your Rust project:
```toml
[dependencies]
cypher-guard = { path = "./cypher-guard" }
```

### Python
After installing with maturin, import and use in Python:
```python
import cypher_guard
# Use validation functions
```

## Contributing
Contributions are welcome! Please open issues or submit pull requests for improvements or bug fixes.

## License
MIT
