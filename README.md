# cypher-guard

![Cypher Guard Banner](assets/cypher-guard-banner.png)

## Introduction

Cypher Guard is an open-source Rust library for parsing and validating [Cypher](https://neo4j.com/developer/cypher/) queries against a user-defined schema. It provides robust, schema-aware validation for graph queries, with bindings for Python and TypeScript/JavaScript. The Python and TypeScript/JS bindings expose a simple API in those languages, but leverage the full performance and safety of Rust under the hood. Cypher Guard is designed for use in developer tools, CI pipelines, and anywhere you need to ensure Cypher query correctness.

---

## Quickstart

### Rust

```sh
cargo build --release
```

### Python

```sh
# Install uv (if not already)
pip install uv

# Build and install Python bindings
make build-python
```

### TypeScript/JavaScript

```sh
make build-js
```

---

## Usage

### Rust Library

```rust
use cypher_guard::{validate_cypher_with_schema, DbSchema};

// Load your schema
let schema_json = r#"{
    "node_props": {
        "Person": [
            {"name": "name", "neo4j_type": "STRING"},
            {"name": "age", "neo4j_type": "INTEGER"}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"name": "since", "neo4j_type": "DateTime"}
        ]
    },
    "relationships": [
        {"start": "Person", "end": "Person", "rel_type": "KNOWS"}
    ],
    "metadata": {"index": [], "constraint": []}
}"#;

let schema = DbSchema::from_json_string(schema_json).unwrap();
let query = "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since";

match validate_cypher_with_schema(query, &schema) {
    Ok(true) => println!("Query is valid!"),
    Ok(false) => println!("Query is invalid"),
    Err(e) => println!("Error: {}", e),
}
```

### Python API

```python
from cypher_guard import validate_cypher, get_validation_errors, parse_query

schema_json = '''{
    "node_props": {
        "Person": [
            {"name": "name", "neo4j_type": "STRING"},
            {"name": "age", "neo4j_type": "INTEGER"}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"name": "since", "neo4j_type": "DateTime"}
        ]
    },
    "relationships": [
        {"start": "Person", "end": "Person", "rel_type": "KNOWS"}
    ],
    "metadata": {"index": [], "constraint": []}
}'''

query = "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since"

# Validate query
try:
    is_valid = validate_cypher(query, schema_json)
    print(f"Query is valid: {is_valid}")
except Exception as e:
    print(f"Validation error: {e}")

# Get all validation errors
errors = get_validation_errors(query, schema_json)
for error in errors:
    print(f"Error: {error}")

# Parse query (returns AST as dict)
ast = parse_query(query)
print(f"Parsed AST: {ast}")
```

### TypeScript/JavaScript API

```typescript
import { validateCypher, getValidationErrors } from "cypher-guard";

const schemaJson = `{
    "node_props": {
        "Person": [
            {"name": "name", "neo4j_type": "STRING"},
            {"name": "age", "neo4j_type": "INTEGER"}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"name": "since", "neo4j_type": "DateTime"}
        ]
    },
    "relationships": [
        {"start": "Person", "end": "Person", "rel_type": "KNOWS"}
    ],
    "metadata": {"index": [], "constraint": []}
}`;

const query = "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since";

try {
    const isValid = validateCypher(query, schemaJson);
    console.log(`Query is valid: ${isValid}`);
} catch (error) {
    console.log(`Validation error: ${error}`);
}

const errors = getValidationErrors(query, schemaJson);
errors.forEach(error => console.log(`Error: ${error}`));
```

---

## Makefile Commands

The Makefile provides convenient shortcuts for building, testing, and linting across Rust, Python, and JavaScript/TypeScript bindings.

| Command         | Description                                                                 |
|-----------------|-----------------------------------------------------------------------------|
| `make` or `make build`         | Build and install the Python extension using uv and maturin.      |
| `make build-python`            | Build and install the Python extension (`uv run maturin develop`). |
| `make test-python`             | Run the Python test suite with pytest.                                |
| `make build-js`                | Install and build the JS/TS bindings (`npm install && npm run build`).|
| `make test-js`                 | Run the JS/TS test suite (`npm test`).                                |
| `make build-rust`              | Build the Rust library (`cargo build`).                               |
| `make test-rust`               | Run Rust tests, formatting, and clippy lints.                        |
| `make fmt`                     | Check Rust code formatting (`cargo fmt --all -- --check`).            |
| `make clippy`                  | Run clippy linter on the main Rust crate.                             |
| `make clippy-all`              | Run clippy linter on all Rust crates.                                 |
| `make clean`                   | Remove build artifacts, Python caches, and node modules.              |
| `make install`                 | Alias for `make build`.                                               |

**Note:** Most commands are cross-platform and will set up all dependencies for you.

---

## How It Works: Parsing & Validation Flow

Cypher Guard separates query validation into three phases:

1. **Parsing**: Uses the [`nom`](https://docs.rs/nom) parser combinator library to convert Cypher into an Abstract Syntax Tree (AST). If parsing fails, you get a syntax error.
2. **Element Extraction**: Traverses the AST to extract all elements that need validation (node labels, relationship types, properties, variables, etc.).
3. **Schema Validation**: Validates the extracted elements against your schema, returning specific error types for different validation failures.

### Project Structure

```
rust/
├── cypher_guard/           # Main Rust library
│   ├── src/
│   │   ├── lib.rs          # Public API
│   │   ├── parser/         # Cypher parsing modules
│   │   │   ├── ast.rs      # Abstract Syntax Tree definitions
│   │   │   ├── clauses.rs  # Top-level clause parsing (MATCH, RETURN, etc.)
│   │   │   ├── patterns.rs # Node/relationship pattern parsing
│   │   │   ├── components.rs # Expression and component parsing
│   │   │   └── utils.rs    # Parser utilities
│   │   ├── schema.rs       # Schema structure and JSON parsing
│   │   ├── validation.rs   # Query element extraction and validation
│   │   └── errors.rs       # Error types and conversion
│   └── Cargo.toml
├── python_bindings/        # Python bindings using PyO3
│   ├── src/lib.rs          # Python API and exception handling
│   ├── tests/              # Python test suite
│   └── pyproject.toml
└── js_bindings/           # TypeScript/JavaScript bindings using napi-rs
    ├── src/lib.rs         # JS/TS API
    ├── test.ts            # JS/TS tests
    └── package.json
```

### Validation Flow

1. **Query Input**: Cypher query string
2. **Parsing**: `parse_query()` → AST
3. **Element Extraction**: `extract_query_elements()` → QueryElements
4. **Schema Validation**: `validate_query_elements()` → Validation Errors
5. **Error Conversion**: Rust errors → Language-specific exceptions

---

## Features

### Core Functionality
- **Full Cypher parsing**: MATCH, OPTIONAL MATCH, RETURN, WITH, WHERE, CREATE, MERGE, SET
- **Schema-aware validation**: Labels, relationship types, properties, type checking
- **Custom error types**: Specific exceptions for different validation failures
- **Multi-language bindings**: Rust, Python, and TypeScript/JavaScript APIs

### Supported Cypher Features

#### Clauses
- `MATCH`, `OPTIONAL MATCH`
- `RETURN`
- `WITH` (aliases, property access, function calls, wildcards)
- `WHERE` (complex conditions, logical operators, parentheses)
- `CREATE`
- `MERGE` (with `ON CREATE` and `ON MATCH`)
- `SET`

#### Node Patterns
- Basic nodes: `(n)`
- Labeled nodes: `(n:Person)`
- Nodes with properties: `(n:Person {name: 'Alice', age: 30})`
- Nodes with variables: `(person:Person)`

#### Relationship Patterns
- Basic: `(a)-[r]->(b)`
- Typed: `(a)-[r:KNOWS]->(b)`
- With properties: `(a)-[r:KNOWS {since: '2020'}]->(b)`
- Variable length: `(a)-[r:KNOWS*1..5]->(b)`
- Optional: `(a)-[r:KNOWS?]->(b)`
- Undirected: `(a)-[r:KNOWS]-(b)`
- Multiple types: `(a)-[r:KNOWS|FRIENDS]->(b)`

#### Quantified Path Patterns (QPP)
- Basic: `((a)-[r:KNOWS]->(b)){1,5}`
- With WHERE: `((a)-[r:KNOWS]->(b) WHERE r.since > '2020'){1,5}`
- With path variables: `p = ((a)-[r:KNOWS]->(b)){1,5}`

#### WHERE Conditions
- Property comparisons: `n.name = 'Alice'`, `n.age > 30`
- Boolean: `n.active = true`, `n.name IS NULL`
- Function calls: `length(n.name) > 5`
- Path properties: `p.length > 2`
- Logical: `AND`, `OR`, `NOT`
- Parenthesized: `(n.age > 30 AND n.active = true)`

#### Property Values
- Strings: `'Alice'`
- Numbers: `30`, `3.14`
- Booleans: `true`, `false`
- NULL: `NULL`
- Lists: `[1, 2, 3]`
- Maps: `{name: 'Alice', age: 30}`
- Function calls: `timestamp()`

#### Function Calls
- Basic: `length(n.name)`
- Nested: `substring(n.name, 0, 5)`
- Multiple arguments: `coalesce(n.name, 'Unknown')`

#### Quantifiers
- Zero or more: `*`
- One or more: `+`
- Exact: `{5}`
- Range: `{1,5}`
- Optional: `?`

#### Path Variables
- Assignment: `p = (a)-[r:KNOWS]->(b)`
- Path properties: `p.length`, `p.nodes`, `p.relationships`

### Error Types

The library provides specific error types for different validation failures:

#### Python Exceptions
- `CypherValidationError` - Base class for all validation errors
- `InvalidNodeLabel` - Invalid node label in schema
- `InvalidRelationshipType` - Invalid relationship type in schema
- `InvalidNodeProperty` - Invalid property on node label
- `InvalidRelationshipProperty` - Invalid property on relationship type
- `InvalidPropertyAccess` - Invalid property access in query
- `InvalidPropertyName` - Invalid property name
- `UndefinedVariable` - Variable referenced but not defined
- `TypeMismatch` - Type mismatch in property values
- `InvalidRelationship` - Invalid relationship pattern
- `InvalidLabel` - Invalid label usage

#### Rust Error Types
- `CypherGuardError` - Main error enum
- `CypherGuardParsingError` - Parsing-specific errors
- `CypherGuardValidationError` - Validation-specific errors
- `CypherGuardSchemaError` - Schema-related errors

---

## Schema Format

Cypher Guard uses a JSON schema format to define your graph structure:

```json
{
    "node_props": {
        "Person": [
            {"name": "name", "neo4j_type": "STRING"},
            {"name": "age", "neo4j_type": "INTEGER"},
            {"name": "active", "neo4j_type": "BOOLEAN"}
        ],
        "Movie": [
            {"name": "title", "neo4j_type": "STRING"},
            {"name": "year", "neo4j_type": "INTEGER"}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"name": "since", "neo4j_type": "DateTime"}
        ],
        "ACTED_IN": [
            {"name": "role", "neo4j_type": "STRING"}
        ]
    },
    "relationships": [
        {"start": "Person", "end": "Person", "rel_type": "KNOWS"},
        {"start": "Person", "end": "Movie", "rel_type": "ACTED_IN"}
    ],
    "metadata": {
        "index": [],
        "constraint": []
    }
}
```

### Schema Components

- **node_props**: Node labels and their properties with Neo4j types
- **rel_props**: Relationship types and their properties
- **relationships**: Valid relationship patterns between node labels
- **metadata**: Indexes and constraints (future use)

---

## Test Coverage

- **Rust**: 190+ tests covering parsing, validation, error handling, and edge cases
- **Python**: Unit tests for valid/invalid queries, schema validation, and error reporting (`rust/python_bindings/tests/unit/`)
- **TypeScript/JS**: Tests for JS/TS bindings (`rust/js_bindings/test.ts`)

Current test status: **73 passed, 26 failed** (mostly test expectation mismatches, core functionality working)

---

## Documentation

- **[Releases](docs/RELEASES.md)** - Download latest releases and view changelog
- **[Versioning Guide](docs/VERSIONING.md)** - Release management and versioning
- **[API Reference](docs/API.md)** - Complete API documentation
- **[Schema Format](docs/SCHEMA.md)** - Schema definition guide

---

## Contributing

This project is open source and welcomes contributions! Please open issues or submit pull requests for improvements or bug fixes.

### Development Setup

1. Clone the repository
2. Install dependencies: `make uv-install`
3. Build: `make build-python`
4. Test: `make test-python`

### Troubleshooting

#### UV Caching Issues on macOS

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

---

## License

MIT