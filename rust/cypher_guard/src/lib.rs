#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

mod errors;
mod parser;
mod schema;

use errors::CypherGuardError;
pub use schema::DbSchema;

use crate::parser::ast::{MatchElement, NodePattern, PatternElement, RelationshipPattern};
use crate::parser::clauses::parse_query;
use crate::parser::clauses::Clause;
use std::collections::HashSet;
pub type Result<T> = std::result::Result<T, CypherGuardError>;

/// Tracks validation state (errors + alias scope)
pub struct ValidationContext {
    pub errors: Vec<String>,
    pub current_aliases: HashSet<String>, // placeholder for alias/with scope support
}

/// Placeholder no-op validator
pub fn validate_cypher(_query: &str) -> Result<bool> {
    Ok(true)
}

/// Validate full query with schema: returns true if valid, or error on parse failure
pub fn validate_cypher_with_schema(query: &str, schema: &DbSchema) -> Result<bool> {
    match parse_query(query) {
        Ok((_, _ast)) => {
            let errors = get_cypher_validation_errors(query, schema);
            Ok(errors.is_empty())
        }
        Err(_) => Err(CypherGuardError::InvalidQuery),
    }
}

/// Main validation routine: parse and traverse the AST
pub fn get_cypher_validation_errors(query: &str, schema: &DbSchema) -> Vec<String> {
    match parse_query(query) {
        Ok((_, ast)) => {
            let mut ctx = ValidationContext {
                errors: Vec::new(),
                current_aliases: HashSet::new(),
            };

            // Validate all elements in the match clause
            for el in &ast.match_clause.elements {
                validate_match_element(el, schema, &mut ctx);
            }

            ctx.errors
        }
        Err(_) => vec!["Invalid Cypher syntax".to_string()],
    }
}

/// Validate one MatchElement (Pattern or Quantified)
fn validate_match_element(el: &MatchElement, schema: &DbSchema, ctx: &mut ValidationContext) {
    match el {
        MatchElement::Pattern(pat) => validate_pattern(pat, schema, ctx),
        MatchElement::QuantifiedPathPattern(qpp) => validate_pattern(&qpp.pattern, schema, ctx),
    }
}

/// Validate a pattern sequence: node-rel-node-rel...
fn validate_pattern(pattern: &[PatternElement], schema: &DbSchema, ctx: &mut ValidationContext) {
    for pe in pattern {
        match pe {
            PatternElement::Node(node) => validate_node(node, schema, ctx),
            PatternElement::Relationship(rel) => validate_relationship(rel, schema, ctx),
        }
    }
}

/// Validate a node (label, properties)
fn validate_node(node: &NodePattern, schema: &DbSchema, ctx: &mut ValidationContext) {
    if let Some(label) = &node.label {
        if !schema.has_label(label) {
            ctx.errors.push(format!("Label '{}' not in schema", label));
        }
    }
    if let Some(props) = &node.properties {
        for prop in props {
            if !schema.has_property_in_nodes(&prop.key)
                && !schema.has_property_in_relationships(&prop.key)
            {
                ctx.errors
                    .push(format!("Property '{}' not in schema", prop.key));
            }
        }
    }
}

/// Validate a relationship (type, properties)
fn validate_relationship(
    rel: &RelationshipPattern,
    schema: &DbSchema,
    ctx: &mut ValidationContext,
) {
    match rel {
        RelationshipPattern::Regular(details) => {
            if let Some(rel_type) = &details.rel_type {
                if !schema.has_relationship_type(rel_type) {
                    ctx.errors
                        .push(format!("Relationship type '{}' not in schema", rel_type));
                }
            }
            if let Some(props) = &details.properties {
                for prop in props {
                    if !schema.has_property_in_relationships(&prop.key) {
                        ctx.errors
                            .push(format!("Property '{}' not in schema", prop.key));
                    }
                }
            }
        }
        RelationshipPattern::OptionalRelationship(_) => {
            // Skip validation for optional relationships
        }
    }
}
