# Cypher Guard API Reference

This document provides an overview of the Cypher Guard API across all supported languages. For detailed auto-generated documentation, see the [API Documentation](https://neo4j-field.github.io/cypher-guard/api/).

## Overview

Cypher Guard provides a unified API for validating Cypher queries against a Neo4j schema. The API is available in Rust, Python, and JavaScript/TypeScript with consistent functionality across all languages.

## Core Concepts

### Schema Definition

All APIs use a JSON schema format to define your graph structure:

```json
{
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
  "metadata": {
    "index": [],
    "constraint": []
  }
}
```

### Validation Process

1. **Parse**: Convert Cypher query to AST
2. **Extract**: Identify all elements (labels, relationships, properties)
3. **Validate**: Check elements against schema
4. **Report**: Return validation results and errors

## Rust API

### Main Functions

```rust
use cypher_guard::{validate_cypher_with_schema, DbSchema};

// Load schema from JSON string
let schema = DbSchema::from_json_string(schema_json)?;

// Validate a query
let result = validate_cypher_with_schema(query, &schema)?;
```

### Error Types

```rust
use cypher_guard::errors::CypherGuardError;

match result {
    Ok(true) => println!("Query is valid"),
    Ok(false) => println!("Query is invalid"),
    Err(CypherGuardError::ValidationError(e)) => println!("Validation error: {}", e),
    Err(CypherGuardError::ParsingError(e)) => println!("Parsing error: {}", e),
    Err(CypherGuardError::SchemaError(e)) => println!("Schema error: {}", e),
}
```

## Python API

### Main Functions

```python
from cypher_guard import validate_cypher, get_validation_errors, parse_query

# Validate a query
try:
    is_valid = validate_cypher(query, schema_json)
    print(f"Query is valid: {is_valid}")
except CypherValidationError as e:
    print(f"Validation error: {e}")

# Get all validation errors
errors = get_validation_errors(query, schema_json)
for error in errors:
    print(f"Error: {error}")

# Parse query to AST
ast = parse_query(query)
```

### Exception Types

```python
from cypher_guard import (
    CypherValidationError,
    InvalidNodeLabel,
    InvalidRelationshipType,
    InvalidNodeProperty,
    InvalidRelationshipProperty,
    InvalidPropertyAccess,
    UndefinedVariable,
    TypeMismatch
)

try:
    validate_cypher(query, schema)
except InvalidNodeLabel as e:
    print(f"Invalid node label: {e}")
except InvalidRelationshipType as e:
    print(f"Invalid relationship type: {e}")
```

## JavaScript/TypeScript API

### Main Functions

```typescript
import { validateCypher, getValidationErrors, parseQuery } from "cypher-guard";

// Validate a query
try {
    const isValid = validateCypher(query, schemaJson);
    console.log(`Query is valid: ${isValid}`);
} catch (error) {
    console.log(`Validation error: ${error}`);
}

// Get all validation errors
const errors = getValidationErrors(query, schemaJson);
errors.forEach(error => console.log(`Error: ${error}`));

// Parse query to AST
const ast = parseQuery(query);
```

### Type Definitions

```typescript
interface ValidationResult {
    isValid: boolean;
    errors: ValidationError[];
}

interface ValidationError {
    type: string;
    message: string;
    location?: {
        line: number;
        column: number;
    };
}
```

## Schema Loading

### From File

```rust
// Rust
let schema = DbSchema::from_json_file("schema.json")?;
```

```python
# Python
from cypher_guard import DbSchema

schema = DbSchema.from_json_file("schema.json")
```

```typescript
// JavaScript
import { DbSchema } from "cypher-guard";

const schema = DbSchema.fromJsonFile("schema.json");
```

### From String

```rust
// Rust
let schema = DbSchema::from_json_string(schema_json)?;
```

```python
# Python
schema = DbSchema.from_json_string(schema_json)
```

```typescript
// JavaScript
const schema = DbSchema.fromJsonString(schemaJson);
```

## Error Handling

### Rust Error Types

- `CypherGuardError::ValidationError` - Query validation failures
- `CypherGuardError::ParsingError` - Cypher syntax errors
- `CypherGuardError::SchemaError` - Schema loading/validation errors

### Python Exceptions

- `CypherValidationError` - Base validation exception
- `InvalidNodeLabel` - Invalid node label in schema
- `InvalidRelationshipType` - Invalid relationship type
- `InvalidNodeProperty` - Invalid property on node
- `InvalidRelationshipProperty` - Invalid property on relationship
- `InvalidPropertyAccess` - Invalid property access
- `UndefinedVariable` - Variable not defined
- `TypeMismatch` - Type validation failure

### JavaScript Errors

JavaScript uses standard Error objects with specific error types in the message and additional properties for error details.

## Performance Considerations

- **Rust**: Fastest performance, suitable for high-throughput validation
- **Python**: Good performance with PyO3 bindings, easy integration
- **JavaScript**: Good performance for web applications and Node.js

## Examples

### Basic Validation

```rust
// Rust
let query = "MATCH (p:Person) RETURN p.name";
let schema = DbSchema::from_json_string(schema_json)?;
let is_valid = validate_cypher_with_schema(query, &schema)?;
```

```python
# Python
query = "MATCH (p:Person) RETURN p.name"
is_valid = validate_cypher(query, schema_json)
```

```typescript
// JavaScript
const query = "MATCH (p:Person) RETURN p.name";
const isValid = validateCypher(query, schemaJson);
```

### Complex Validation with Error Details

```rust
// Rust
match validate_cypher_with_schema(query, &schema) {
    Ok(true) => println!("✅ Query is valid"),
    Ok(false) => println!("❌ Query is invalid"),
    Err(e) => println!("🚫 Error: {}", e),
}
```

```python
# Python
try:
    is_valid = validate_cypher(query, schema_json)
    if is_valid:
        print("✅ Query is valid")
    else:
        print("❌ Query is invalid")
except CypherValidationError as e:
    print(f"🚫 Validation error: {e}")
```

```typescript
// JavaScript
try {
    const isValid = validateCypher(query, schemaJson);
    if (isValid) {
        console.log("✅ Query is valid");
    } else {
        console.log("❌ Query is invalid");
    }
} catch (error) {
    console.log(`🚫 Error: ${error}`);
}
```

## Advanced Usage

### Custom Error Handling

```rust
// Rust - Custom error handling
use cypher_guard::errors::*;

fn validate_query(query: &str, schema: &DbSchema) -> Result<(), String> {
    match validate_cypher_with_schema(query, schema) {
        Ok(true) => Ok(()),
        Ok(false) => Err("Query is invalid".to_string()),
        Err(CypherGuardError::ValidationError(e)) => Err(format!("Validation: {}", e)),
        Err(CypherGuardError::ParsingError(e)) => Err(format!("Parsing: {}", e)),
        Err(CypherGuardError::SchemaError(e)) => Err(format!("Schema: {}", e)),
    }
}
```

### Batch Validation

```python
# Python - Batch validation
queries = [
    "MATCH (p:Person) RETURN p.name",
    "MATCH (p:Person)-[:KNOWS]->(f:Person) RETURN p, f",
    "MATCH (p:Person) WHERE p.age > 30 RETURN p"
]

results = []
for query in queries:
    try:
        is_valid = validate_cypher(query, schema_json)
        results.append((query, is_valid, None))
    except Exception as e:
        results.append((query, False, str(e)))

for query, is_valid, error in results:
    if is_valid:
        print(f"✅ {query}")
    else:
        print(f"❌ {query}: {error}")
```

## Integration Examples

### CI/CD Pipeline

```yaml
# GitHub Actions example
- name: Validate Cypher queries
  run: |
    python -c "
    from cypher_guard import validate_cypher
    import json
    
    with open('schema.json') as f:
        schema = json.load(f)
    
    with open('queries.cypher') as f:
        queries = f.readlines()
    
    for i, query in enumerate(queries, 1):
        try:
            if not validate_cypher(query.strip(), schema):
                print(f'Query {i} is invalid')
                exit(1)
        except Exception as e:
            print(f'Query {i} error: {e}')
            exit(1)
    
    print('All queries are valid!')
    "
```

### Web Application

```typescript
// Express.js middleware
import { validateCypher } from "cypher-guard";

app.post('/validate-query', (req, res) => {
    const { query, schema } = req.body;
    
    try {
        const isValid = validateCypher(query, schema);
        res.json({ isValid, errors: [] });
    } catch (error) {
        res.json({ 
            isValid: false, 
            errors: [{ type: 'validation', message: error.message }] 
        });
    }
});
```

## Troubleshooting

### Common Issues

1. **Schema Loading Errors**: Ensure JSON is valid and follows the schema format
2. **Import Errors**: Make sure the package is properly installed
3. **Feature Flags**: Rust requires `python-bindings` feature for Python integration

### Debug Mode

```rust
// Rust - Enable debug logging
env_logger::init();
let result = validate_cypher_with_schema(query, &schema)?;
```

```python
# Python - Enable debug output
import logging
logging.basicConfig(level=logging.DEBUG)
```

## Related Documentation

- [Auto-generated API docs](https://neo4j-field.github.io/cypher-guard/api/)
- [Schema Format](SCHEMA.md)
- [Examples](../examples/)
- [Releases](RELEASES.md) 