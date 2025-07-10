mod errors;
pub mod parser {
    pub mod ast;
    pub mod clauses;
    pub mod components;
    pub mod patterns;
    pub mod utils;
}
mod schema;
mod validation;

use errors::convert_nom_error;
pub use errors::{
    CypherGuardError, CypherGuardParsingError, CypherGuardSchemaError, CypherGuardValidationError,
};
pub use schema::{
    DbSchema, DbSchemaConstraint, DbSchemaIndex, DbSchemaMetadata, DbSchemaProperty,
    DbSchemaRelationshipPattern, PropertyType,
};

use parser::ast::*;
pub type Result<T> = std::result::Result<T, CypherGuardError>;

/// Placeholder no-op validator
pub fn validate_cypher(_query: &str) -> Result<bool> {
    Ok(true)
}

/// Parse a Cypher query with custom error handling
pub fn parse_query(query: &str) -> std::result::Result<Query, CypherGuardParsingError> {
    match parser::clauses::parse_query(query) {
        Ok((_, ast)) => Ok(ast),
        Err(e) => Err(convert_nom_error(e, "query", query)),
    }
}

use crate::validation::{extract_query_elements, validate_query_elements};

/// Validate full query with schema: returns true if valid, or error on parse failure
pub fn validate_cypher_with_schema(query: &str, schema: &DbSchema) -> Result<bool> {
    let ast = parse_query(query)?;
    let elements = extract_query_elements(&ast);
    let errors = validate_query_elements(&elements, schema);
    if errors.is_empty() {
        Ok(true)
    } else {
        // Convert validation errors to a single error message
        let msg = errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        Err(CypherGuardError::InvalidQuery(msg))
    }
}

/// Get validation errors for a query (for Python/JS bindings)
pub fn get_cypher_validation_errors(query: &str, schema: &DbSchema) -> Vec<String> {
    match parse_query(query) {
        Ok(ast) => {
            let elements = extract_query_elements(&ast);
            let errors = validate_query_elements(&elements, schema);
            errors.into_iter().map(|e| e.to_string()).collect()
        }
        Err(_) => vec!["Invalid Cypher syntax".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_success() {
        let query = "MATCH (a)-[:KNOWS]->(b) RETURN a, b";
        let result = parse_query(query);
        assert!(result.is_ok());

        let ast = result.unwrap();
        assert!(ast.match_clause.is_some());
        assert!(ast.return_clause.is_some());
    }

    #[test]
    fn test_parse_query_failure() {
        let query = "INVALID QUERY";
        let result = parse_query(query);
        assert!(result.is_err());

        let error = result.unwrap_err();
        // Should be a CypherGuardParsingError, not a generic nom error
        assert!(matches!(error, CypherGuardParsingError::Nom(_)));
    }

    #[test]
    fn test_validate_cypher_with_schema_uses_custom_errors() {
        let schema = DbSchema::new();
        let query = "INVALID QUERY";
        let result = validate_cypher_with_schema(query, &schema);
        assert!(result.is_err());

        let error = result.unwrap_err();
        // Should be a Parsing error containing our custom error
        assert!(matches!(error, CypherGuardError::Parsing(_)));
    }
}
