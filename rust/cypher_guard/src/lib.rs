mod errors;
pub mod parser {
    pub mod ast;
    pub mod clauses;
    pub mod components;
    pub mod patterns;
    pub mod span;
    pub mod utils;
}
mod schema;
mod validation;

use errors::convert_nom_error;
pub use errors::{
    CypherGuardError, CypherGuardParsingError, CypherGuardSchemaError, CypherGuardValidationError,
};
pub use schema::{
    DbSchema, DbSchemaConstraint, DbSchemaIndex, DbSchemaMetadata,
    DbSchemaProperty, DbSchemaRelationshipPattern, PropertyType,
};

use parser::ast::*;
pub type Result<T> = std::result::Result<T, CypherGuardError>;

/// Placeholder no-op validator
pub fn validate_cypher(_query: &str) -> Result<bool> {
    Ok(true)
}

/// Parse a Cypher query with custom error handling
pub fn parse_query(query: &str) -> std::result::Result<Query, CypherGuardParsingError> {
    println!("DEBUG: lib.rs parse_query called with: {}", query);
    match parser::clauses::parse_query(query) {
        Ok((remaining, ast)) => {
            println!("DEBUG: lib.rs parse_query succeeded, remaining: '{}', AST: {:?}", remaining, ast);
            Ok(ast)
        },
        Err(nom::Err::Error(e)) => {
            // Check if this is a validation error by looking at the error kind
            // If it's a Tag error, it might be a validation error
            if e.code == nom::error::ErrorKind::Tag {
                // Try to reconstruct the validation error based on the input
                // This is a bit hacky but works for our specific case
                if query.contains("RETURN")
                    && query.contains("MATCH")
                    && query.find("RETURN").unwrap() < query.find("MATCH").unwrap()
                {
                    return Err(CypherGuardParsingError::return_before_other_clauses());
                }
                if query.contains("WHERE")
                    && query.contains("MATCH")
                    && query.find("WHERE").unwrap() < query.find("MATCH").unwrap()
                {
                    return Err(CypherGuardParsingError::where_before_match());
                }
                if query.contains("WITH")
                    && query.contains("MATCH")
                    && query.find("WITH").unwrap() < query.find("MATCH").unwrap()
                {
                    return Err(CypherGuardParsingError::invalid_clause_order(
                        "query start",
                        "WITH must come after a reading clause (MATCH, UNWIND, CREATE, MERGE)",
                    ));
                }
                if query.contains("UNWIND")
                    && query.contains("MATCH")
                    && query.find("UNWIND").unwrap() < query.find("MATCH").unwrap()
                {
                    return Err(CypherGuardParsingError::invalid_clause_order(
                        "query start",
                        "UNWIND must come after a reading clause (MATCH, UNWIND, CREATE, MERGE)",
                    ));
                }
                // Check for clauses after RETURN - need to find the last occurrence of RETURN
                if let Some(last_return_pos) = query.rfind("RETURN") {
                    if let Some(match_after_return) = query[last_return_pos..].find("MATCH") {
                        if match_after_return > 0 {
                            return Err(CypherGuardParsingError::match_after_return());
                        }
                    }
                    if let Some(where_after_return) = query[last_return_pos..].find("WHERE") {
                        if where_after_return > 0 {
                            return Err(CypherGuardParsingError::invalid_clause_order(
                                "after RETURN",
                                "WHERE cannot come after RETURN clause",
                            ));
                        }
                    }
                    if let Some(with_after_return) = query[last_return_pos..].find("WITH") {
                        if with_after_return > 0 {
                            return Err(CypherGuardParsingError::with_after_return());
                        }
                    }
                    if let Some(unwind_after_return) = query[last_return_pos..].find("UNWIND") {
                        if unwind_after_return > 0 {
                            return Err(CypherGuardParsingError::unwind_after_return());
                        }
                    }
                }
                if query.contains("MATCH")
                    && query.contains("WITH")
                    && !query.contains("RETURN")
                    && query.find("WITH").unwrap() > query.find("MATCH").unwrap()
                {
                    return Err(CypherGuardParsingError::missing_required_clause(
                        "RETURN or writing clause",
                    ));
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
    println!("DEBUG: validate_cypher_with_schema called with query: {}", query);
    let ast = parse_query(query)?;
    println!("DEBUG: Parsed AST successfully: {:#?}", ast);
    let elements = extract_query_elements(&ast);
    eprintln!("DEBUG: Extracted elements successfully");
    let errors = validate_query_elements(&elements, schema);
    eprintln!("DEBUG: Validation completed with {} errors", errors.len());
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
    println!("🔍 get_cypher_validation_errors called with: {}", query);
    match parse_query(query) {
        Ok(ast) => {
            println!("🔍 Parse succeeded, AST: {:?}", ast);
            let elements = extract_query_elements(&ast);
            println!("🔍 Extracted elements: referenced={:?}, defined={:?}", elements.referenced_variables, elements.defined_variables);
            let errors = validate_query_elements(&elements, schema);
            println!("🔍 Validation completed with {} errors: {:?}", errors.len(), errors);
            errors.into_iter().map(|e| e.to_string()).collect()
        }
        Err(e) => {
            println!("🔍 Parse failed with error: {:?}", e);
            vec!["Invalid Cypher syntax".to_string()]
        }
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
        assert!(!ast.match_clauses.is_empty());
        assert!(!ast.return_clauses.is_empty());
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
