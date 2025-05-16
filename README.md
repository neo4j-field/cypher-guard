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
errors.rs → returns a CypherGuardError or Vec<String> with issues



📁 File-by-File Role
src/lib.rs
Exposes public functions like:

validate_cypher_with_schema(query, schema)

get_cypher_validation_errors(query, schema)

Delegates parsing to parser::query

Delegates validation to schema + error logic

src/parser.rs
Re-exports query() from parser::clauses::query

This is the entry point for parsing a full Cypher query

src/parser/clauses.rs
Parses top-level clauses:

MATCH, RETURN

Builds MatchClause, ReturnClause from strings

Calls into match_element and pattern_element_sequence

src/parser/patterns.rs
Parses the inner graph structure:

NodePattern via node_pattern()

RelationshipPattern via relationship_pattern()

Repeating paths via quantified_path_pattern()

These functions build the PatternElement list

src/parser/ast.rs
Defines all AST types:

Query, MatchClause, PatternElement, NodePattern, etc.

These types are instantiated by the functions in clauses.rs and patterns.rs

The AST is fully constructed here using nom-parsed values

src/schema.rs
Defines DbSchema

Implements methods to check:

has_label(label)

has_relationship_type(type)

has_property_in_nodes(key)

has_property_in_relationships(key)

Used during AST traversal to validate each element

src/errors.rs
Defines the CypherGuardError enum

Used to distinguish between:

Syntax errors (e.g., invalid query string)

Validation errors (e.g., unknown label)

src/main.rs
CLI entry point

Likely reads from stdin, calls validate_cypher_with_schema, and prints results

src/parser/utils.rs
Helper functions used during parsing:

identifier(), number_literal(), opt_identifier()

These are reused across patterns.rs and clauses.rs