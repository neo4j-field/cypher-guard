# cypher-guard

Cypher Guard is a Rust library and CLI tool for validating Cypher queries against a user-defined schema. It supports both Rust and Python integration via PyO3 and maturin.

## Features
- Validate Cypher query syntax
- Validate node labels, relationship types, and properties against a schema
- JSON-based schema loading
- Python bindings via PyO3
- **Schema-aware validation**: Ensures all labels, relationship types, and properties in a query exist in the provided schema
- **Makefile** for easy Python extension build

## Installation

### Rust
Clone the repository and build with Cargo:
```sh
cargo build --release
```

### Python
Build and install the Python module using maturin or the provided Makefile:
```sh
# Option 1: Use maturin directly
pip install maturin
cd cypher-guard
maturin develop

# Option 2: Use the Makefile (installs maturin if needed)
make
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
After installing with maturin or `make`, import and use in Python:
```python
import cypher_guard

schema = '''
{
    "labels": ["Person"],
    "rel_types": ["KNOWS"],
    "properties": {"name": {"type": "String"}, "age": {"type": "Integer"}},
    "enums": {}
}
'''
query = 'MATCH (n:Person {name: "Alice", age: 30}) RETURN n'

# Validate query
is_valid = cypher_guard.validate_cypher_py(query, schema)
print("Is valid:", is_valid)

# Get validation errors
errors = cypher_guard.get_validation_errors_py(query, schema)
print("Errors:", errors)
```

## Contributing
Contributions are welcome! Please open issues or submit pull requests for improvements or bug fixes.

## License
MIT
