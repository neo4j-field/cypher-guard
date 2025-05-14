#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

mod errors;
mod parser;
mod schema;

use errors::CypherGuardError;
pub use schema::DbSchema;

pub type Result<T> = std::result::Result<T, CypherGuardError>;

/// Placeholder validation function
pub fn validate_cypher(_query: &str) -> Result<bool> {
    // TODO: Implement validation logic
    Ok(true)
}
pub fn validate_cypher_with_schema(query: &str, schema: &DbSchema) -> Result<bool> {
    match parser::query(query) {
        Ok((_, _ast)) => {
            let errors = get_cypher_validation_errors(query, schema);
            Ok(errors.is_empty())
        }
        Err(_) => Err(CypherGuardError::InvalidQuery),
    }
}

pub fn get_cypher_validation_errors(query: &str, schema: &DbSchema) -> Vec<String> {
    let mut errors = Vec::new();
    match parser::query(query) {
        Ok((_, ast)) => {
            // Walk the AST and check labels, rel_types, properties
            for element in &ast.match_clause.elements {
                match element {
                    parser::MatchElement::Pattern(pattern) => {
                        for pe in pattern {
                            match pe {
                                parser::PatternElement::Node(node) => {
                                    if let Some(label) = &node.label {
                                        if !schema.has_label(label) {
                                            errors.push(format!("Label '{}' not in schema", label));
                                        }
                                    }
                                    if let Some(props) = &node.properties {
                                        for prop in props {
                                            if !schema.has_property_in_nodes(&prop.key)
                                                && !schema.has_property_in_relationships(&prop.key)
                                            {
                                                errors.push(format!(
                                                    "Property '{}' not in schema",
                                                    prop.key
                                                ));
                                            }
                                        }
                                    }
                                }
                                parser::PatternElement::Relationship(rel) => {
                                    if let Some(rel_type) = &rel.rel_type {
                                        if !schema.has_relationship_type(rel_type) {
                                            errors.push(format!(
                                                "Relationship type '{}' not in schema",
                                                rel_type
                                            ));
                                        }
                                    }
                                    if let Some(props) = &rel.properties {
                                        for prop in props {
                                            if !schema.has_property_in_relationships(&prop.key) {
                                                errors.push(format!(
                                                    "Property '{}' not in schema",
                                                    prop.key
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    parser::MatchElement::QuantifiedPathPattern(qpp) => {
                        for pe in &qpp.pattern {
                            match pe {
                                parser::PatternElement::Node(node) => {
                                    if let Some(label) = &node.label {
                                        if !schema.has_label(label) {
                                            errors.push(format!("Label '{}' not in schema", label));
                                        }
                                    }
                                    if let Some(props) = &node.properties {
                                        for prop in props {
                                            if !schema.has_property_in_nodes(&prop.key) {
                                                errors.push(format!(
                                                    "Property '{}' not in schema",
                                                    prop.key
                                                ));
                                            }
                                        }
                                    }
                                }
                                parser::PatternElement::Relationship(rel) => {
                                    if let Some(rel_type) = &rel.rel_type {
                                        if !schema.has_relationship_type(rel_type) {
                                            errors.push(format!(
                                                "Relationship type '{}' not in schema",
                                                rel_type
                                            ));
                                        }
                                    }
                                    if let Some(props) = &rel.properties {
                                        for prop in props {
                                            if !schema.has_property_in_relationships(&prop.key) {
                                                errors.push(format!(
                                                    "Property '{}' not in schema",
                                                    prop.key
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {
            errors.push("Invalid Cypher syntax".to_string());
        }
    }
    errors
}
