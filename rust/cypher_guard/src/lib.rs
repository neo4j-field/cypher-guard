#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

mod errors;
mod parser;
mod schema;

use errors::CypherGuardError;
pub use schema::DbSchema;

use crate::parser::ast::{MatchElement, NodePattern, PatternElement, RelationshipPattern};
use crate::parser::clauses::parse_query;
use std::collections::HashSet;
use std::collections::HashMap;
pub type Result<T> = std::result::Result<T, CypherGuardError>;

/// Tracks validation state (errors + alias scope)
pub struct ValidationContext {
    pub errors: Vec<String>,
    pub current_aliases: HashSet<String>, // placeholder for alias/with scope support
    pub var_types: HashMap<String, VarInfo>, // variable name -> type info
}

#[derive(Debug, Clone)]
pub enum VarInfo {
    Node { label: String },
    Relationship { rel_type: String },
}

/// Placeholder no-op validator
pub fn validate_cypher(_query: &str) -> Result<bool> {
    Ok(true)
}

/// Validate full query with schema: returns true if valid, or error on parse failure
pub fn validate_cypher_with_schema(query: &str, schema: &DbSchema) -> Result<bool> {
    match parse_query(query) {
        Ok((_, ast)) => {
            let mut ctx = ValidationContext {
                errors: Vec::new(),
                current_aliases: HashSet::new(),
                var_types: HashMap::new(),
            };

            // Validate all elements in the match clause
            for el in &ast.match_clause.elements {
                validate_match_element(el, schema, &mut ctx);
            }

            // Validate return clause
            if let Some(return_clause) = &ast.return_clause {
                for item in &return_clause.items {
                    if let Some((var, prop)) = item.split_once('.') {
                        if let Some(var_info) = ctx.var_types.get(var) {
                            match var_info {
                                VarInfo::Node { label } => {
                                    if !schema.has_node_property(label, prop) {
                                        let error = format!(
                                            "Property '{}' not in schema for label '{}'",
                                            prop, label
                                        );
                                        ctx.errors.push(error);
                                    }
                                }
                                VarInfo::Relationship { rel_type } => {
                                    if !schema.has_relationship_property(rel_type, prop) {
                                        let error = format!(
                                            "Property '{}' not in schema for relationship type '{}'",
                                            prop, rel_type
                                        );
                                        ctx.errors.push(error);
                                    }
                                }
                            }
                        } else {
                            let error = format!(
                                "Variable '{}' not found in MATCH clause",
                                var
                            );
                            ctx.errors.push(error);
                        }
                    }
                }
            }

            Ok(ctx.errors.is_empty())
        }
        Err(e) => {
            eprintln!("Parser error: {:?}", e);
            Err(CypherGuardError::InvalidQuery)
        }
    }
}

/// Main validation routine: parse and traverse the AST
pub fn get_cypher_validation_errors(query: &str, schema: &DbSchema) -> Vec<String> {
    match parse_query(query) {
        Ok((_, ast)) => {
            println!("AST: {:?}", ast); // Debug: Print the AST
            let mut ctx = ValidationContext {
                errors: Vec::new(),
                current_aliases: HashSet::new(),
                var_types: HashMap::new(),
            };

            // Validate all elements in the match clause
            for el in &ast.match_clause.elements {
                validate_match_element(el, schema, &mut ctx);
            }

            println!("Variable mapping: {:?}", ctx.var_types); // Debug: Print the variable mapping

            // Validate return clause
            if let Some(return_clause) = &ast.return_clause {
                for item in &return_clause.items {
                    if let Some((var, prop)) = item.split_once('.') {
                        println!("Validating RETURN item: {} -> {}", var, prop); // Debug: Print RETURN item
                        if let Some(var_info) = ctx.var_types.get(var) {
                            match var_info {
                                VarInfo::Node { label } => {
                                    println!("Node variable '{}' has label '{}'", var, label); // Debug: Print node label
                                    if !schema.has_node_property(label, prop) {
                                        let error = format!(
                                            "Property '{}' not in schema for label '{}'",
                                            prop, label
                                        );
                                        println!("Validation error: {}", error); // Debug: Print validation error
                                        ctx.errors.push(error);
                                    }
                                }
                                VarInfo::Relationship { rel_type } => {
                                    println!("Relationship variable '{}' has type '{}'", var, rel_type); // Debug: Print relationship type
                                    if !schema.has_relationship_property(rel_type, prop) {
                                        let error = format!(
                                            "Property '{}' not in schema for relationship type '{}'",
                                            prop, rel_type
                                        );
                                        println!("Validation error: {}", error); // Debug: Print validation error
                                        ctx.errors.push(error);
                                    }
                                }
                            }
                        } else {
                            let error = format!(
                                "Variable '{}' not found in MATCH clause",
                                var
                            );
                            println!("Validation error: {}", error); // Debug: Print validation error
                            ctx.errors.push(error);
                        }
                    }
                }
            }

            println!("Final validation errors: {:?}", ctx.errors); // Debug: Print final errors
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
            PatternElement::Node(node) => {
                // Record variable if present
                if let Some(var) = &node.variable {
                    if let Some(label) = &node.label {
                        ctx.var_types.insert(var.clone(), VarInfo::Node { label: label.clone() });
                        println!("Recorded node variable: {} -> {:?}", var, ctx.var_types.get(var)); // Debug: Print recorded node variable
                    } else {
                        // Record node variable without label
                        ctx.var_types.insert(var.clone(), VarInfo::Node { label: "".to_string() });
                        println!("Recorded node variable without label: {} -> {:?}", var, ctx.var_types.get(var)); // Debug: Print recorded node variable
                    }
                }
                validate_node(node, schema, ctx)
            }
            PatternElement::Relationship(rel) => {
                // Record variable if present
                if let RelationshipPattern::Regular(details) = rel {
                    if let Some(var) = &details.variable {
                        if let Some(rel_type) = &details.rel_type {
                            ctx.var_types.insert(var.clone(), VarInfo::Relationship { rel_type: rel_type.clone() });
                            println!("Recorded relationship variable: {} -> {:?}", var, ctx.var_types.get(var)); // Debug: Print recorded relationship variable
                        } else {
                            // Record relationship variable without type
                            ctx.var_types.insert(var.clone(), VarInfo::Relationship { rel_type: "".to_string() });
                            println!("Recorded relationship variable without type: {} -> {:?}", var, ctx.var_types.get(var)); // Debug: Print recorded relationship variable
                        }
                    }
                }
                validate_relationship(rel, schema, ctx)
            }
        }
    }
}

/// Validate a node (label, properties)
fn validate_node(node: &NodePattern, schema: &DbSchema, ctx: &mut ValidationContext) {
    if let Some(label) = &node.label {
        if !schema.has_label(label) {
            ctx.errors.push(format!("Label '{}' not in schema", label));
        }
        if let Some(props) = &node.properties {
            for prop in props {
                if !schema.has_node_property(label, &prop.key) {
                    ctx.errors.push(format!(
                        "Property '{}' not in schema for label '{}'",
                        prop.key, label
                    ));
                }
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
                if let Some(props) = &details.properties {
                    for prop in props {
                        if !schema.has_relationship_property(rel_type, &prop.key) {
                            ctx.errors.push(format!(
                                "Property '{}' not in schema for relationship type '{}'",
                                prop.key, rel_type
                            ));
                        }
                    }
                }
            }
        }
        RelationshipPattern::OptionalRelationship(_) => {
            // Skip validation for optional relationships
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_query() {
        let query = "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since";
        let schema = DbSchema {
            node_props: std::collections::HashMap::new(),
            rel_props: std::collections::HashMap::new(),
            relationships: vec![],
            metadata: Default::default(),
        };
        match crate::parser::clauses::parse_query(query) {
            Ok((rest, ast)) => {
                println!("Parsed AST: {:?}", ast);
                println!("Remaining: {:?}", rest);
            }
            Err(e) => {
                eprintln!("Parse error: {:?}", e);
                panic!("Parse error: {:?}", e);
            }
        }
    }
}
