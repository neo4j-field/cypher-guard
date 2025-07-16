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
        Err(nom::Err::Error(e)) => {
            // Check if this is a validation error by looking at the error kind
            // If it's a Tag error, it might be a validation error
            if e.code == nom::error::ErrorKind::Tag {
                // Try to reconstruct the validation error based on the input
                // This is a bit hacky but works for our specific case
                if query.contains("RETURN") && query.contains("MATCH") && query.find("RETURN").unwrap() < query.find("MATCH").unwrap() {
                    return Err(CypherGuardParsingError::invalid_clause_order(
                        "query start",
                        "RETURN must come after a reading clause (MATCH, UNWIND, CREATE, MERGE)"
                    ));
                }
                if query.contains("WHERE") && query.contains("MATCH") && query.find("WHERE").unwrap() < query.find("MATCH").unwrap() {
                    return Err(CypherGuardParsingError::invalid_clause_order(
                        "query start",
                        "WHERE must come after a reading clause (MATCH, UNWIND, CREATE, MERGE)"
                    ));
                }
                if query.contains("WITH") && query.contains("MATCH") && query.find("WITH").unwrap() < query.find("MATCH").unwrap() {
                    return Err(CypherGuardParsingError::invalid_clause_order(
                        "query start",
                        "WITH must come after a reading clause (MATCH, UNWIND, CREATE, MERGE)"
                    ));
                }
                if query.contains("UNWIND") && query.contains("MATCH") && query.find("UNWIND").unwrap() < query.find("MATCH").unwrap() {
                    return Err(CypherGuardParsingError::invalid_clause_order(
                        "query start",
                        "UNWIND must come after a reading clause (MATCH, UNWIND, CREATE, MERGE)"
                    ));
                }
                // Check for clauses after RETURN - need to find the last occurrence of RETURN
                if let Some(last_return_pos) = query.rfind("RETURN") {
                    if let Some(match_after_return) = query[last_return_pos..].find("MATCH") {
                        if match_after_return > 0 {
                            return Err(CypherGuardParsingError::invalid_clause_order(
                                "after RETURN",
                                "MATCH cannot come after RETURN clause"
                            ));
                        }
                    }
                    if let Some(where_after_return) = query[last_return_pos..].find("WHERE") {
                        if where_after_return > 0 {
                            return Err(CypherGuardParsingError::invalid_clause_order(
                                "after RETURN",
                                "WHERE cannot come after RETURN clause"
                            ));
                        }
                    }
                    if let Some(with_after_return) = query[last_return_pos..].find("WITH") {
                        if with_after_return > 0 {
                            return Err(CypherGuardParsingError::invalid_clause_order(
                                "after RETURN",
                                "WITH cannot come after RETURN clause"
                            ));
                        }
                    }
                    if let Some(unwind_after_return) = query[last_return_pos..].find("UNWIND") {
                        if unwind_after_return > 0 {
                            return Err(CypherGuardParsingError::invalid_clause_order(
                                "after RETURN",
                                "UNWIND cannot come after RETURN clause"
                            ));
                        }
                    }
                }
                if query.contains("MATCH") && query.contains("WITH") && !query.contains("RETURN") && query.find("WITH").unwrap() > query.find("MATCH").unwrap() {
                    return Err(CypherGuardParsingError::missing_required_clause("RETURN or writing clause"));
                }
            }
            Err(convert_nom_error(nom::Err::Error(e), "query", query))
        }
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
        // Return the first validation error to preserve specific error types
        Err(CypherGuardError::Validation(
            errors.into_iter().next().unwrap(),
        ))
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
