# Cypher Guard

![Cypher Guard Banner](assets/cypher-guard-banner.png)

A Rust library for parsing and validating [Cypher](https://neo4j.com/developer/cypher/) queries with Python and JavaScript bindings. Provides robust, schema-aware validation for graph queries with high performance and safety.

---

## Quick Start

### Python

```bash
# Install from PyPI
pip install cypher-guard
```

```python
from cypher_guard import validate_cypher, DbSchema

# Create schema from Neo4j GraphRAG format
schema = DbSchema.from_dict({
    "node_props": {
        "Person": [{"property": "name", "type": "STRING"}]
    },
    "relationships": [
        {"start": "Person", "type": "KNOWS", "end": "Person"}
    ]
})

# Validate query
errors = validate_cypher("MATCH (p:Person) RETURN p.name", schema)
if not errors:
    print("Query is valid!")
```

### JavaScript/TypeScript

```bash
# Install from npm
npm install cypher-guard
```

```typescript
import { validateCypher, DbSchema } from "cypher-guard";

const schema = DbSchema.fromDict({
    node_props: {
        Person: [{ property: "name", type: "STRING" }]
    },
    relationships: [
        { start: "Person", type: "KNOWS", end: "Person" }
    ]
});

const errors = validateCypher("MATCH (p:Person) RETURN p.name", schema);
```

### Rust

```bash
# Add to Cargo.toml
cargo add cypher-guard
```

```rust
use cypher_guard::{validate_cypher_with_schema, DbSchema};

let schema = DbSchema::from_json_string(schema_json)?;
let errors = validate_cypher_with_schema(query, &schema)?;
```

---

## Core API

### Python Functions

- **`validate_cypher(query, schema)`** - Returns list of validation errors
- **`check_syntax(query)`** - Check syntax only (no schema needed)
- **`is_write(query)`** - Check if query modifies data
- **`is_read(query)`** - Check if query is read-only
- **`has_parser_errors(query)`** - Check if query has syntax errors

### Schema Classes

- **`DbSchema`** - Main schema container
- **`DbSchemaProperty`** - Property definitions
- **`DbSchemaRelationshipPattern`** - Relationship patterns

### Error Handling

Specific exception types for different validation failures:
- `CypherValidationError` - Base validation error
- `InvalidNodeLabel`, `InvalidRelationshipType` - Schema errors
- `UndefinedVariable`, `TypeMismatch` - Query errors
- `NomParsingError` - Syntax errors

---

## Schema Format

Cypher Guard uses the Neo4j GraphRAG schema format:

```json
{
    "node_props": {
        "Person": [
            {"property": "name", "type": "STRING"},
            {"property": "age", "type": "INTEGER"}
        ]
    },
    "rel_props": {
        "KNOWS": [
            {"property": "since", "type": "DATE"}
        ]
    },
    "relationships": [
        {"start": "Person", "type": "KNOWS", "end": "Person"}
    ],
    "metadata": {
        "constraint": [],
        "index": []
    }
}
```

### Integration with Neo4j GraphRAG

```python
from neo4j_graphrag.schema import get_structured_schema
from cypher_guard import DbSchema

# Get schema from Neo4j
schema = get_structured_schema(driver, is_enhanced=True)

# Convert to DbSchema
db_schema = DbSchema.from_dict(schema)

# Use for validation
errors = validate_cypher(query, db_schema)
```

---

## Development

For local development, building from source, and contributing:

**[Development Guide](docs/DEVELOPMENT.md)** - Complete setup, build, and testing instructions

---

## Documentation

- **[API Reference](docs/API.md)** - Complete API documentation
- **[Schema Format](docs/SCHEMA.md)** - Schema definition guide
- **[Development Guide](docs/DEVELOPMENT.md)** - Local development and building from source
- **[Parser Internals](docs/PARSER_INTERNALS.md)** - How the parser works
- **[Releases](docs/RELEASES.md)** - Download latest releases
- **[Versioning](docs/VERSIONING.md)** - Release management

---

## License

MIT