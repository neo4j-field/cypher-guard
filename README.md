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
cd rust/python_bindings
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
from cypher_guard import validate_cypher_py, get_validation_errors_py

# Example schema JSON
schema_json = '''
{
    "node_props": {
        "Person": [
            {"name": "name", "neo4j_type": {"type": "STRING"}},
            {"name": "age", "neo4j_type": {"type": "INTEGER"}}
        ],
        "Movie": [
            {"name": "title", "neo4j_type": {"type": "STRING"}},
            {"name": "year", "neo4j_type": {"type": "INTEGER"}}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"name": "since", "neo4j_type": {"type": "STRING"}}
        ],
        "ACTED_IN": [
            {"name": "role", "neo4j_type": {"type": "STRING"}}
        ]
    },
    "relationships": [
        {"start": "Person", "end": "Person", "rel_type": "KNOWS"},
        {"start": "Person", "end": "Movie", "rel_type": "ACTED_IN"}
    ],
    "metadata": {
        "indexes": [],
        "constraints": []
    }
}
'''

# Example query
query = 'MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since'

# Validate query
is_valid = validate_cypher_py(query, schema_json)
print("Is valid:", is_valid)

# Get validation errors
errors = get_validation_errors_py(query, schema_json)
print("Errors:", errors)
```

## Contributing
Contributions are welcome! Please open issues or submit pull requests for improvements or bug fixes.

## License
MIT

## Implementation Details

### Two-Phase Validation: Parse → Validate

Cypher Guard separates query validation into two distinct phases:

#### 1. Parsing Phase

- Uses the [`nom`](https://docs.rs/nom) parser combinator library to convert a raw Cypher string into a structured **Abstract Syntax Tree (AST)**.
- If the input is **not valid Cypher syntax**, parsing fails early and returns a `nom::Err`.
- On success, a fully-formed `Query` struct is constructed, consisting of nested structs/enums like:
  - `MatchClause`
  - `PatternElement`
  - `NodePattern`, `RelationshipPattern`, etc.

#### 2. Validation Phase

- Once the AST is built, Cypher Guard traverses the tree in **preorder DFS**.
- Each node is validated against a user-provided schema (`DbSchema`) to ensure:
  - All labels are defined in the schema
  - All relationship types exist
  - All properties are known and correctly typed
- Validation returns a list of **semantic errors**, if any are found.

This phase separation provides a clear distinction between **syntactic validity** (can it be parsed?) and **semantic validity** (does it make sense in your graph?).

---

### Error Tree

The *error tree* in Cypher Guard is a conceptual model that captures validation issues **in relation to AST structure**.

- Parsing errors bubble up immediately and stop execution.
- Validation errors are collected as you traverse the AST.
- Each AST node may emit one or more errors (e.g., unknown label, invalid property).
- The collected errors **preserve context** and can be traced back to specific query parts (e.g., `MatchClause -> Pattern[0] -> Node`).

This approach supports precise, explainable diagnostics — ideal for debugging tools, UIs, and LLM feedback loops.

---

### Submodules and Structure

Cypher Guard is composed of several focused submodules:

#### `parser`
- The core parsing logic, built using `nom`.
- Split into:
  - `clauses.rs`: Parses top-level clauses like `MATCH`, `RETURN`
  - `patterns.rs`: Parses internal path structures like nodes, relationships, quantified paths
  - `ast.rs`: Defines all AST node types (`Query`, `MatchClause`, `PatternElement`, etc.)
- The root function `parser::query()` returns a `Query` struct or a parse error.

#### `schema`
- Defines the `DbSchema` struct and logic for:
  - Validating whether a label, relationship, or property exists
  - Performing type checks for properties (e.g., String, Integer, Enum)
- Used during the validation phase.

#### `errors`
- Defines the `CypherGuardError` enum used throughout the library.
- Encapsulates:
  - Parsing failures (`InvalidQuery`)
  - Schema validation errors (missing labels, properties)
- Returned by top-level functions like `validate_cypher_with_schema`.

---

### Summary of Flow
```
[Cypher Query String]
        ↓
parser.rs → query()
        ↓
parser/clauses.rs → match_clause(), return_clause()
        ↓
parser/patterns.rs → node_pattern(), relationship_pattern(), pattern_element_sequence()
        ↓
parser/ast.rs → builds up structs like Query, MatchClause, PatternElement, etc.
        ↓
lib.rs → get_cypher_validation_errors()
        ↓
schema.rs → checks labels, rel types, and properties against DbSchema
        ↓
errors.rs → returns a CypherGuardError or Vec<String> with issues```


