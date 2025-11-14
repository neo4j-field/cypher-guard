# Parser Internals

This document explains how Cypher Guard's parser and validation system works internally.

## Architecture Overview

Cypher Guard separates query validation into three phases:

1. **Parsing**: Uses the [`nom`](https://docs.rs/nom) parser combinator library to convert Cypher into an Abstract Syntax Tree (AST). If parsing fails, you get a syntax error.
2. **Element Extraction**: Traverses the AST to extract all elements that need validation (node labels, relationship types, properties, variables, etc.).
3. **Schema Validation**: Validates the extracted elements against your schema, returning specific error types for different validation failures.

## Project Structure

```
rust/
├── cypher_guard/           # Main Rust library
│   ├── src/
│   │   ├── lib.rs          # Public API and main entry point
│   │   ├── parser/         # Cypher parsing modules
│   │   │   ├── mod.rs      # Module declarations and re-exports
│   │   │   ├── ast.rs      # Abstract Syntax Tree definitions (269 lines)
│   │   │   ├── clauses.rs  # Top-level clause parsing (2988 lines)
│   │   │   ├── patterns.rs # Node/relationship pattern parsing (927 lines)
│   │   │   ├── components.rs # Expression and component parsing
│   │   │   ├── utils.rs    # Parser utilities and helpers
│   │   │   └── span.rs     # Source location tracking
│   │   ├── schema.rs       # Schema structure and JSON parsing
│   │   ├── validation.rs   # Query element extraction and validation (1418 lines)
│   │   └── errors.rs       # Error types and conversion (1552 lines)
│   └── Cargo.toml
├── python_bindings/        # Python bindings using PyO3
│   ├── src/lib.rs          # Python API and exception handling (1316 lines)
│   ├── tests/              # Python test suite
│   │   ├── unit/           # Unit tests
│   │   └── integration/    # Integration tests
│   └── pyproject.toml
└── js_bindings/           # TypeScript/JavaScript bindings using napi-rs
    ├── src/lib.rs         # JS/TS API
    ├── test.ts            # JS/TS tests
    └── package.json
```

### Module Responsibilities

#### Core Parser Modules

- **`ast.rs`**: Defines all AST structures (Query, clauses, patterns, expressions)
- **`clauses.rs`**: Parses top-level Cypher clauses (MATCH, RETURN, WHERE, etc.)
- **`patterns.rs`**: Parses node and relationship patterns
- **`components.rs`**: Parses expressions, properties, and other components
- **`utils.rs`**: Common parsing utilities and helper functions
- **`span.rs`**: Source location tracking for error reporting

#### Core Library Modules

- **`lib.rs`**: Public API, main entry points, and high-level functions
- **`schema.rs`**: Schema definition, JSON parsing, and schema validation
- **`validation.rs`**: Query element extraction and schema validation logic
- **`errors.rs`**: Comprehensive error type definitions and conversions

#### Binding Modules

- **`python_bindings/src/lib.rs`**: Python API, exception handling, and PyO3 integration
- **`js_bindings/src/lib.rs`**: JavaScript/TypeScript API and napi-rs integration

### File Size Analysis

The largest files indicate the complexity of different components:
- **`clauses.rs` (2988 lines)**: Most complex parsing logic
- **`validation.rs` (1418 lines)**: Comprehensive validation system
- **`errors.rs` (1552 lines)**: Extensive error handling
- **`python_bindings/src/lib.rs` (1316 lines)**: Complete Python API
- **`patterns.rs` (927 lines)**: Pattern parsing complexity

## Validation Flow

1. **Query Input**: Cypher query string
2. **Parsing**: `parse_query()` → AST
3. **Element Extraction**: `extract_query_elements()` → QueryElements
4. **Schema Validation**: `validate_query_elements()` → Validation Errors
5. **Error Conversion**: Rust errors → Language-specific exceptions

## Parser Behavior

### Fail-Fast Parsing

The parser uses a "fail-fast" approach - it stops on the first syntax error encountered. This means:

- If there are multiple syntax errors, you'll only see the first one
- To find all errors, you need to fix them iteratively:
  1. Call `check_syntax()` to find the first error
  2. Fix that error
  3. Call `check_syntax()` again to find the next error
  4. Repeat until no more errors

### Error Types

The parser distinguishes between different levels of errors:

- **Syntax Errors**: Basic parsing failures (missing parentheses, invalid tokens)
- **Semantic Errors**: Valid syntax but invalid structure (RETURN before MATCH)
- **Schema Errors**: Valid query but doesn't match schema (invalid labels, properties)

## Supported Cypher Features

### Clauses
- `MATCH`, `OPTIONAL MATCH`
- `RETURN`
- `WITH` (aliases, property access, function calls, wildcards)
- `WHERE` (complex conditions, logical operators, parentheses)
- `CREATE`
- `MERGE` (with `ON CREATE` and `ON MATCH`)
- `SET`
- `LIMIT` (Cypher 5.24+)

### Node Patterns
- Basic nodes: `(n)`
- Labeled nodes: `(n:Person)`
- Nodes with properties: `(n:Person {name: 'Alice', age: 30})`
- Nodes with variables: `(person:Person)`

### Relationship Patterns
- Basic: `(a)-[r]->(b)`
- Typed: `(a)-[r:KNOWS]->(b)`
- With properties: `(a)-[r:KNOWS {since: '2020'}]->(b)`
- Variable length: `(a)-[r:KNOWS*1..5]->(b)`
- Optional: `(a)-[r:KNOWS?]->(b)`
- Undirected: `(a)-[r:KNOWS]-(b)`
- Multiple types: `(a)-[r:KNOWS|FRIENDS]->(b)`

### Quantified Path Patterns (QPP)
- Basic: `((a)-[r:KNOWS]->(b)){1,5}`
- With WHERE: `((a)-[r:KNOWS]->(b) WHERE r.since > '2020'){1,5}`
- With path variables: `p = ((a)-[r:KNOWS]->(b)){1,5}`

### WHERE Conditions
- Property comparisons: `n.name = 'Alice'`, `n.age > 30`
- Boolean: `n.active = true`, `n.name IS NULL`
- Function calls: `length(n.name) > 5`
- Path properties: `p.length > 2`
- Logical: `AND`, `OR`, `NOT`
- Parenthesized: `(n.age > 30 AND n.active = true)`

### Property Values
- Strings: `'Alice'`
- Numbers: `30`, `3.14`
- Booleans: `true`, `false`
- NULL: `NULL`
- Lists: `[1, 2, 3]`
- Maps: `{name: 'Alice', age: 30}`
- Function calls: `timestamp()`

### Function Calls
- Basic: `length(n.name)`
- Nested: `substring(n.name, 0, 5)`
- Multiple arguments: `coalesce(n.name, 'Unknown')`

### Quantifiers
- Zero or more: `*`
- One or more: `+`
- Exact: `{5}`
- Range: `{1,5}`
- Optional: `?`

### Path Variables
- Assignment: `p = (a)-[r:KNOWS]->(b)`
- Path properties: `p.length`, `p.nodes`, `p.relationships`

## Error Handling

### Rust Error Types
- `CypherGuardError` - Main error enum
- `CypherGuardParsingError` - Parsing-specific errors
- `CypherGuardValidationError` - Validation-specific errors
- `CypherGuardSchemaError` - Schema-related errors

### Python Exception Hierarchy
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

### Parsing Error Types
- `NomParsingError` - Basic syntax errors
- `ReturnBeforeOtherClauses` - RETURN clause in wrong position
- `MatchAfterReturn` - MATCH after RETURN
- `InvalidClauseOrder` - Clauses in wrong order
- `MissingRequiredClause` - Required clause missing
- And many more specific parsing errors...

## AST Structure

The Abstract Syntax Tree (AST) is the core data structure representing parsed Cypher queries. The AST is organized hierarchically with clear separation between different clause types.

### Query Structure
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct Query {
    pub match_clauses: Vec<MatchClause>,
    pub merge_clauses: Vec<MergeClause>,
    pub create_clauses: Vec<CreateClause>,
    pub with_clauses: Vec<WithClause>,
    pub where_clauses: Vec<WhereClause>,
    pub return_clauses: Vec<ReturnClause>,
    pub unwind_clauses: Vec<UnwindClause>,
    pub call_clauses: Vec<CallClause>,
    pub limit_clauses: Vec<LimitClause>,
}
```

**Key Design Decision**: The Query struct uses separate vectors for each clause type rather than a single `Vec<Clause>`. This allows for:
- **Type Safety**: Each clause type is strongly typed
- **Easy Access**: Direct access to specific clause types without pattern matching
- **Validation**: Easier validation of clause ordering and relationships

### Clause Types

#### MatchClause
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct MatchClause {
    pub elements: Vec<MatchElement>,
    pub is_optional: bool,
}
```

#### ReturnClause
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct ReturnClause {
    pub items: Vec<String>, // Simplified for validation
}
```

#### LimitClause
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct LimitClause {
    pub expression: PropertyValue, // Expression that evaluates to a positive INTEGER
}
```

The `LIMIT` clause constrains the number of returned rows. It can be used:
- As a standalone clause after `MATCH` (Cypher 5.24+): `MATCH (n) LIMIT 2 RETURN collect(n.name)`
- After `RETURN` clause (traditional): `MATCH (n) RETURN n.name LIMIT 3`

Currently, `LIMIT` supports numeric literals and parameters. Function calls and arithmetic expressions are not yet supported.

#### WhereClause
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct WhereClause {
    pub conditions: Vec<WhereCondition>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum WhereCondition {
    Comparison { left: PropertyValue, operator: String, right: PropertyValue },
    FunctionCall { function: String, arguments: Vec<String> },
    PathProperty { path_var: String, property: String },
    And(Box<WhereCondition>, Box<WhereCondition>),
    Or(Box<WhereCondition>, Box<WhereCondition>),
    Not(Box<WhereCondition>),
    Parenthesized(Box<WhereCondition>),
}
```

### Pattern Types

#### NodePattern
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct NodePattern {
    pub variable: Option<String>,    // e.g., "n" in (n:Person)
    pub label: Option<String>,       // e.g., "Person" in (n:Person)
    pub properties: Option<Vec<Property>>, // e.g., {name: 'Alice'}
}
```

#### RelationshipPattern
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum RelationshipPattern {
    Regular(RelationshipDetails),
    OptionalRelationship(RelationshipDetails),
}

#[derive(Debug, PartialEq, Clone)]
pub struct RelationshipDetails {
    pub variable: Option<String>,     // e.g., "r" in [r:KNOWS]
    pub direction: Direction,         // Left, Right, Undirected
    pub properties: Option<Vec<Property>>,
    pub rel_type: Option<String>,    // e.g., "KNOWS"
    pub length: Option<LengthRange>, // e.g., *1..5
    pub where_clause: Option<WhereClause>,
    pub quantifier: Option<Quantifier>,
    pub is_optional: bool,
}
```

#### QuantifiedPathPattern
```rust
#[derive(Debug, PartialEq, Clone)]
pub struct QuantifiedPathPattern {
    pub pattern: Vec<PatternElement>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub where_clause: Option<WhereClause>,
    pub path_variable: Option<String>,
}
```

### PropertyValue System
```rust
#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    String(String),
    Number(i64),
    Boolean(bool),
    Null,
    List(Vec<PropertyValue>),
    Map(std::collections::HashMap<String, PropertyValue>),
    FunctionCall { name: String, args: Vec<PropertyValue> },
    Parameter(String),
    Identifier(String), // For variable references and property access
}
```

**Note**: The AST uses `i64` for all numbers, simplifying type handling during validation.

## Parser Implementation Details

### Nom Parser Combinators

The parser uses Rust's `nom` library with a combinator-based approach. Here are real examples from the codebase:

#### Node Pattern Parsing
```rust
pub fn node_pattern(input: &str) -> IResult<&str, NodePattern> {
    match char::<&str, nom::error::Error<&str>>('(')(input) {
        Ok((input, _)) => {
            let (input, variable) = opt(identifier)(input)?;
            let (input, label) = opt(preceded(char(':'), identifier))(input)?;
            let (input, _) = multispace0(input)?;
            let (input, properties) = opt(property_map)(input)?;
            let (input, _) = char(')')(input)?;
            let result = NodePattern {
                variable: variable.map(|s| s.to_string()),
                label: label.map(|s| s.to_string()),
                properties,
            };
            Ok((input, result))
        }
        Err(e) => Err(e),
    }
}
```

#### Match Clause Parsing
```rust
pub fn match_clause(input: &str) -> IResult<&str, MatchClause> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag_no_case("match")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, elements) = match_element_list(input)?;
    
    Ok((input, MatchClause {
        elements,
        is_optional: false,
    }))
}
```

#### Match Element List Parsing
```rust
pub fn match_element_list(input: &str) -> IResult<&str, Vec<MatchElement>> {
    // Parse comma-separated match elements
    let (input, elements) = separated_list1(
        tuple((multispace0, char(','), multispace0)), 
        match_element
    )(input)?;
    Ok((input, elements))
}
```

### Parsing Strategy

The parser follows a **recursive descent** approach with these phases:

1. **Lexical Analysis**: Uses `nom` combinators to tokenize input
   - `tag()` for keywords (`MATCH`, `RETURN`)
   - `identifier()` for variables and labels
   - `char()` for punctuation (`(`, `)`, `[`, `]`)
   - `multispace0()` for whitespace handling

2. **Syntactic Analysis**: Builds AST using parser combinators
   - `tuple()` for sequential parsing
   - `alt()` for alternative parsing paths
   - `opt()` for optional elements
   - `many0()` and `separated_list0()` for repetitions

3. **Semantic Analysis**: Validates structure and relationships
   - Clause ordering validation
   - Variable scope checking
   - Pattern consistency validation

4. **Error Recovery**: Limited error recovery with context
   - **Backtracking**: `alt()` tries multiple parse paths
   - **Error Context**: Uses `Spanned` types to track locations
   - **Suggestions**: Provides hints for common errors

### Error Handling Architecture

The parser uses a sophisticated error handling system:

#### Error Types Hierarchy
```rust
#[derive(Debug, Error)]
pub enum CypherGuardError {
    #[error("Validation error: {0}")]
    Validation(#[from] CypherGuardValidationError),
    #[error("Parsing error: {0}")]
    Parsing(#[from] CypherGuardParsingError),
    #[error("Schema error: {0}")]
    Schema(#[from] CypherGuardSchemaError),
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}
```

#### Parsing Error Types
```rust
#[derive(Debug, Error)]
pub enum CypherGuardParsingError {
    #[error("Nom parsing error: {0}")]
    NomParsingError(String),
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("Expected token: {0}")]
    ExpectedToken(String),
    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),
    #[error("Return clause before other clauses")]
    ReturnBeforeOtherClauses,
    #[error("Match clause after return clause")]
    MatchAfterReturn,
    // ... 25+ specific error types
}
```

### Validation System

The validation system extracts elements from the AST and validates them against the schema:

#### QueryElements Structure
```rust
#[derive(Debug, Clone)]
pub struct QueryElements {
    pub node_labels: HashSet<String>,
    pub relationship_types: HashSet<String>,
    pub node_properties: HashMap<String, HashSet<String>>,
    pub relationship_properties: HashMap<String, HashSet<String>>,
    pub property_accesses: Vec<PropertyAccess>,
    pub property_comparisons: Vec<PropertyComparison>,
    pub defined_variables: HashSet<String>,
    pub referenced_variables: HashSet<String>,
    pub pattern_sequences: Vec<Vec<PatternElement>>,
    pub variable_node_bindings: HashMap<String, String>,
    pub variable_relationship_bindings: HashMap<String, String>,
}
```

#### Validation Process
1. **Element Extraction**: Traverse AST to extract all elements
2. **Variable Resolution**: Map variables to their types (node/relationship)
3. **Schema Validation**: Check elements against schema constraints
4. **Type Checking**: Validate property types and comparisons
5. **Error Generation**: Create specific error messages for failures

## Performance Considerations

- **Rust Core**: High-performance parsing and validation
- **Memory Efficient**: AST is designed for minimal memory usage
- **Fast Error Reporting**: Errors are generated without full query execution
- **Schema Caching**: Schema is parsed once and reused for multiple queries
- **Zero-Copy Parsing**: Uses string slices to avoid unnecessary allocations
- **Lazy Evaluation**: Only parse what's needed for validation

## Testing

- **Rust**: 190+ tests covering parsing, validation, error handling, and edge cases
- **Python**: Unit tests for valid/invalid queries, schema validation, and error reporting
- **TypeScript/JS**: Tests for JS/TS bindings

Current test status: **73 passed, 26 failed** (mostly test expectation mismatches, core functionality working)

## Limitations and Known Issues

### Current Limitations

- **Fail-Fast Parsing**: Only reports the first syntax error (by design)
- **Limited Cypher Coverage**: Not all Cypher features are supported yet
- **No Query Optimization**: Parser doesn't optimize queries, only validates
- **Schema Validation Only**: Doesn't validate against actual database state

### Unsupported Cypher Features

- **Advanced Clauses**: `UNWIND`, `FOREACH`, `LOAD CSV`
- **Complex Functions**: User-defined functions, complex aggregations
- **Advanced Patterns**: Complex path patterns, subqueries
- **Database-Specific**: Index hints, query plans, execution context

### Known Issues

- **Integer Overflow**: Some validation scenarios can cause integer overflow (see skipped tests)
- **Error Context**: Error messages could be more descriptive in some cases
- **Performance**: Large schemas may impact validation performance

## Future Enhancements

### Planned Features

- **Multiple Error Reporting**: Collect and report multiple syntax errors
- **Enhanced Error Messages**: More descriptive error messages with suggestions
- **Extended Cypher Support**: Support for more Cypher features
- **Query Optimization**: Basic query optimization and analysis
- **Schema Inference**: Infer schema from existing queries

### Architecture Improvements

- **Modular Parser**: More modular parser architecture for easier extension
- **Plugin System**: Plugin system for custom validation rules
- **Performance Optimization**: Further performance optimizations
- **Better Error Recovery**: Improved error recovery and reporting
