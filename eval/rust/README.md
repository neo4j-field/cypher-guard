# Cypher Guard Evaluation Tool

A comprehensive evaluation framework for testing Cypher Guard's query validation capabilities against structured test datasets.

## Features

- 🔍 **Automatic Query Discovery**: Recursively finds all YAML query files in specified directories
- 📊 **Schema Validation**: Loads and validates queries against Neo4j schema definitions  
- 📈 **Statistical Reporting**: Provides accuracy metrics and detailed validation statistics
- 🎨 **Rich Output**: Color-coded results with detailed error reporting
- 📂 **Batch Processing**: Processes multiple query files and generates consolidated reports
- ⚡ **Fast Execution**: Written in Rust for optimal performance

## Installation

The evaluation tool is part of the Cypher Guard workspace. To build and run:

```bash
# From the project root
cargo build --bin eval

# Or run directly
cargo run --bin eval -- [OPTIONS]
```

## Usage

### Basic Usage

```bash
# Run with default settings (looks for schema and queries in data/)
cargo run --bin eval

# Run with verbose output
cargo run --bin eval -- --verbose

# Run with detailed per-query results
cargo run --bin eval -- --detailed

# Specify custom paths
cargo run --bin eval -- \
  --schema /path/to/schema.json \
  --queries /path/to/queries \
  --verbose \
  --detailed
```

### Command Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--schema` | `-s` | Path to schema JSON file | `../../data/schema/eval_schema.json` |
| `--queries` | `-q` | Directory containing query YAML files | `../../data/queries` |
| `--verbose` | `-v` | Enable verbose output during processing | `false` |
| `--detailed` | `-d` | Show detailed results for each query | `false` |

## Query File Format

Query files should be in YAML format with the following structure:

```yaml
name: "Query Set Name"
description: "Description of the query set"
category: "query_category"
queries:
  - name: "individual_query_name"
    description: "Description of what this query does"
    cypher: |
      MATCH (n:Node)
      RETURN n.property
  - name: "another_query"
    description: "Another query description"
    cypher: |
      MATCH (a)-[r:REL]->(b)
      WHERE a.prop > 10
      RETURN a, b
```

## Directory Structure

The tool expects queries to be organized in directories that indicate their expected validation status:

```
data/queries/
├── valid/          # Queries that should pass validation
│   ├── simple.yml
│   ├── advanced.yml
│   └── functions.yml
└── invalid/        # Queries that should fail validation
    ├── malformed.yml
    └── schema_violations.yml
```

- Files in `valid/` directories are expected to pass validation
- Files in `invalid/` directories are expected to fail validation
- The tool calculates accuracy based on whether results match expectations

## Output Format

### Summary Report

```
🚀 Cypher Guard Evaluation Tool
Schema: data/schema/eval_schema.json
Queries: data/queries

📊 EVALUATION SUMMARY
═══════════════════════════════════════
Files processed: 10
Total queries: 156

Results:
  ✅ Correct validations: 140
  ❌ Incorrect validations: 16
  🚫 Parsing errors: 0

🎯 Accuracy: 89.7%
```

### Detailed Results (with --detailed)

```
📋 DETAILED RESULTS
═══════════════════════════════════════

📂 data/queries/valid/simple.yml
   ✅ PASS basic_node_match
      📝 Basic node matching and property selection
   ✅ PASS where_clause_filtering
      📝 WHERE clause with comparison and boolean filtering
   ❌ FAIL complex_aggregation
      📝 Complex aggregation with multiple GROUP BY
      🚫 Error: Property 'nonexistent' not in schema for label 'Person'
```

### Failed Validation Details

For any failures, the tool shows:

```
❌ FAILED VALIDATIONS
═══════════════════════════════════════

📂 File: data/queries/valid/advanced.yml
📝 Query: complex_subquery
📋 Description: Complex subquery with multiple WITH clauses
🎯 Expected: VALID
📊 Got: INVALID
🚫 Error: Label 'NonExistentNode' not in schema
🔍 Cypher:
MATCH (n:NonExistentNode)
RETURN n.property
──────────────────────────────────────────────────
```

## Schema Format

The tool expects Neo4j schema files in JSON format with the following structure:

```json
{
  "nodeProps": {
    "Person": [
      {
        "name": "firstName",
        "neo4j_type": "STRING",
        "example_values": ["John", "Jane"]
      }
    ]
  },
  "relProps": {
    "WORKS_FOR": [
      {
        "name": "startDate",
        "neo4j_type": "DATE_TIME"
      }
    ]
  },
  "relationships": [
    {
      "start": "Person",
      "end": "Company", 
      "rel_type": "WORKS_FOR"
    }
  ],
  "metadata": {
    "constraint": [...],
    "index": [...]
  }
}
```

## Performance

The evaluation tool is designed for high performance:

- **Parallel Processing**: Uses Rust's built-in concurrency for file processing
- **Memory Efficient**: Streams large query files without loading everything into memory
- **Fast Schema Loading**: Efficient JSON parsing and validation
- **Minimal Allocations**: Optimized for repeated query validation

## Integration

### CI/CD Pipeline

The tool can be integrated into continuous integration pipelines:

```bash
#!/bin/bash
# Run evaluation and check for regressions
cargo run --bin eval -- --schema schema.json --queries test_queries/

# Exit with non-zero code if accuracy drops below threshold
if [ $accuracy -lt 90 ]; then
  echo "Accuracy dropped below 90%"
  exit 1
fi
```

### Development Workflow

Use the tool during development to test schema changes:

```bash
# Test against current schema
cargo run --bin eval -- --verbose

# Test with modified schema
cargo run --bin eval -- --schema new_schema.json --detailed
```

## Error Handling

The tool provides comprehensive error reporting for:

- **Schema Loading Errors**: Invalid JSON format, missing required fields
- **Query File Errors**: YAML parsing errors, missing required fields  
- **Validation Errors**: Cypher syntax errors, schema violations
- **File System Errors**: Missing files, permission issues

## Contributing

To add new test queries:

1. Create or modify YAML files in `data/queries/valid/` or `data/queries/invalid/`
2. Follow the expected query file format
3. Run the evaluation tool to verify your queries
4. Update schema files if introducing new node labels or properties

## Examples

### Testing Schema Changes

```bash
# Baseline evaluation
cargo run --bin eval -- --schema current_schema.json > baseline_results.txt

# Test with new schema
cargo run --bin eval -- --schema new_schema.json > new_results.txt

# Compare results
diff baseline_results.txt new_results.txt
```

### Validating Specific Query Categories

```bash
# Test only simple queries
cargo run --bin eval -- --queries data/queries/valid/simple.yml

# Test functions and predicates
cargo run --bin eval -- --queries data/queries/valid/functions.yml --detailed
```

## Troubleshooting

**Schema Loading Issues**: Ensure all field names use snake_case (e.g., `neo4j_type`, not `neo4jType`)

**Query Parsing Errors**: Check YAML syntax and ensure proper indentation

**Low Accuracy**: Review failed queries to identify schema gaps or validation logic issues

**Performance Issues**: For very large query sets, consider running without `--detailed` flag 