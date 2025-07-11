# CypherGuard Schema Documentation

This directory contains the JSON schema definition and example files for the CypherGuard database schema validation system.

## Files

- `cypher_guard_schema.json` - The main JSON schema definition that validates CypherGuard database schemas
- `example_schema.json` - An example schema file demonstrating proper usage
- `README.md` - This documentation file

## Schema Structure

The CypherGuard schema is defined by the following main components:

### 1. Node Properties (`node_props`)
Defines the properties for each node label in your graph database.

```json
{
  "node_props": {
    "Person": [
      {
        "name": "first_name",
        "neo4j_type": "STRING",
        "example_values": ["John", "Jane", "Alice"]
      }
    ]
  }
}
```

### 2. Relationship Properties (`rel_props`)
Defines the properties for each relationship type.

```json
{
  "rel_props": {
    "WORKS_FOR": [
      {
        "name": "start_date",
        "neo4j_type": "DATE_TIME",
        "example_values": ["2020-01-15T08:00:00Z"]
      }
    ]
  }
}
```

### 3. Relationship Patterns (`relationships`)
Defines the allowed relationship patterns between node labels.

```json
{
  "relationships": [
    {
      "start": "Person",
      "end": "Company",
      "rel_type": "WORKS_FOR"
    }
  ]
}
```

### 4. Metadata (`metadata`)
Contains database constraints and indexes.

```json
{
  "metadata": {
    "constraint": [...],
    "index": [...]
  }
}
```

## Property Types

The following Neo4j property types are supported:

- `STRING` - Text values
- `INTEGER` - Whole numbers
- `FLOAT` - Decimal numbers
- `BOOLEAN` - True/false values
- `POINT` - Spatial coordinates
- `DATE_TIME` - Date and time values
- `LIST` - Arrays of values

## Property Constraints

Properties can have additional constraints:

- `enum_values` - Restricted set of allowed values
- `min_value` / `max_value` - Numeric range constraints
- `distinct_value_count` - Number of unique values
- `example_values` - Sample values for documentation

## Usage

1. Create your database schema file following the structure in `example_schema.json`
2. Validate your schema file against `cypher_guard_schema.json` using any JSON schema validator
3. Use the validated schema with the CypherGuard library for Cypher query validation

## Validation

To validate a schema file, you can use tools like:

- Online JSON Schema validators
- Command-line tools like `ajv-cli`
- Programming language libraries that support JSON Schema Draft 07

Example with ajv-cli:
```bash
npm install -g ajv-cli
ajv validate -s cypher_guard_schema.json -d your_schema.json
```

## Example Usage in Code

```rust
use cypher_guard::DbSchema;

// Load schema from JSON file
let schema = DbSchema::from_json_file("data/schema/example_schema.json")?;

// Validate a Cypher query
let query = "MATCH (p:Person)-[:WORKS_FOR]->(c:Company) RETURN p.first_name, c.company_name";
let is_valid = validate_cypher_with_schema(query, &schema)?;
```

## Schema Validation Rules

The schema enforces several validation rules:

1. **Property names** must be in snake_case format
2. **Node labels and relationship types** must be valid identifiers
3. **Min/max values** must be logically consistent (min ≤ max)
4. **Enum values** must be non-empty arrays when specified
5. **All required fields** must be present

For more information, see the CypherGuard documentation. 