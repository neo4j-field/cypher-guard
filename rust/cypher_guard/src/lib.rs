#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;

mod errors;
mod parser;
mod schema;

use errors::CypherGuardError;
pub use schema::{DbSchema, DbSchemaProperty, PropertyType};

use crate::parser::ast::{
    MatchElement, NodePattern, PatternElement, Query, RelationshipPattern, WhereClause,
    WhereCondition,
};
use crate::parser::clauses::parse_query;
use std::collections::HashMap;
use std::collections::HashSet;
pub type Result<T> = std::result::Result<T, CypherGuardError>;

/// Tracks validation state (errors + alias scope)
pub struct ValidationContext<'a> {
    pub errors: Vec<String>,
    pub current_aliases: HashSet<String>, // placeholder for alias/with scope support
    pub var_types: HashMap<String, VarInfo>, // variable name -> type info
    pub schema: &'a DbSchema,
}

#[derive(Debug, Clone)]
pub enum VarInfo {
    Node { label: String },
    Relationship { rel_type: String },
    Path,
}

/// Placeholder no-op validator
pub fn validate_cypher(_query: &str) -> Result<bool> {
    Ok(true)
}

/// Validate full query with schema: returns true if valid, or error on parse failure
pub fn validate_cypher_with_schema(query: &str, schema: &DbSchema) -> Result<bool> {
    println!("Validating query: {}", query);
    println!("Schema: {:?}", schema);
    let ast = parse_query(query).map_err(|e| {
        println!("Parser error: {:?}", e);
        CypherGuardError::InvalidQuery
    })?;
    println!("AST: {:?}", ast);
    let mut ctx = ValidationContext {
        errors: Vec::new(),
        current_aliases: HashSet::new(),
        var_types: HashMap::new(),
        schema,
    };
    validate_query(&ast.1, &mut ctx);
    if ctx.errors.is_empty() {
        Ok(true)
    } else {
        Err(CypherGuardError::InvalidQuery)
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
                schema,
            };

            // Validate all elements in the match clause
            if let Some(match_clause) = &ast.match_clause {
                for el in &match_clause.elements {
                    validate_match_element(el, schema, &mut ctx);
                }
            }

            println!("Variable mapping: {:?}", ctx.var_types); // Debug: Print the variable mapping

            // Validate WHERE clause if present
            if let Some(where_clause) = &ast.where_clause {
                validate_where_clause(where_clause, schema, &mut ctx);
            }

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
                                    println!(
                                        "Relationship variable '{}' has type '{}'",
                                        var, rel_type
                                    ); // Debug: Print relationship type
                                    if !schema.has_relationship_property(rel_type, prop) {
                                        let error = format!(
                                            "Property '{}' not in schema for relationship type '{}'",
                                            prop, rel_type
                                        );
                                        println!("Validation error: {}", error); // Debug: Print validation error
                                        ctx.errors.push(error);
                                    }
                                }
                                VarInfo::Path => {
                                    // Path property validation if needed
                                }
                            }
                        } else {
                            let error = format!("Variable '{}' not found in MATCH clause", var);
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

/// Validate a Cypher query against a schema
fn validate_query(query: &Query, ctx: &mut ValidationContext) {
    // Validate match clause
    if let Some(match_clause) = &query.match_clause {
        for el in &match_clause.elements {
            validate_match_element(el, ctx.schema, ctx);
        }
    }

    // Validate WHERE clause if present
    if let Some(where_clause) = &query.where_clause {
        for condition in &where_clause.conditions {
            match condition {
                WhereCondition::Comparison {
                    left,
                    operator: _,
                    right: _,
                } => {
                    if let Some((var, prop)) = left.split_once('.') {
                        if let Some(var_info) = ctx.var_types.get(var) {
                            match var_info {
                                VarInfo::Node { label } => {
                                    if !ctx.schema.has_node_property(label, prop) {
                                        ctx.errors.push(format!(
                                            "Property '{}' not in schema for label '{}'",
                                            prop, label
                                        ));
                                    }
                                }
                                VarInfo::Relationship { rel_type } => {
                                    if !ctx.schema.has_relationship_property(rel_type, prop) {
                                        ctx.errors.push(format!(
                                            "Property '{}' not in schema for relationship type '{}'",
                                            prop, rel_type
                                        ));
                                    }
                                }
                                VarInfo::Path => {
                                    // Path property validation if needed
                                }
                            }
                        } else {
                            ctx.errors.push(format!("Variable '{}' not defined", var));
                        }
                    }
                }
                WhereCondition::FunctionCall { .. } | WhereCondition::PathProperty { .. } => {
                    // Already handled in validate_where_clause
                }
            }
        }
    }

    // Validate return clause
    if let Some(return_clause) = &query.return_clause {
        for item in &return_clause.items {
            if let Some((var, prop)) = item.split_once('.') {
                if let Some(var_info) = ctx.var_types.get(var) {
                    match var_info {
                        VarInfo::Node { label } => {
                            if !ctx.schema.has_node_property(label, prop) {
                                ctx.errors.push(format!(
                                    "Property '{}' not in schema for label '{}'",
                                    prop, label
                                ));
                            }
                        }
                        VarInfo::Relationship { rel_type } => {
                            if !ctx.schema.has_relationship_property(rel_type, prop) {
                                ctx.errors.push(format!(
                                    "Property '{}' not in schema for relationship type '{}'",
                                    prop, rel_type
                                ));
                            }
                        }
                        VarInfo::Path => {
                            // Path property validation if needed
                        }
                    }
                } else {
                    ctx.errors.push(format!("Variable '{}' not defined", var));
                }
            }
        }
    }
}

/// Validate one MatchElement (Pattern or Quantified)
fn validate_match_element(el: &MatchElement, schema: &DbSchema, ctx: &mut ValidationContext) {
    match el {
        MatchElement::Pattern(pat) => validate_pattern(pat, schema, ctx),
        MatchElement::QuantifiedPathPattern(qpp) => {
            // Validate the pattern
            validate_pattern(&qpp.pattern, schema, ctx);

            // Validate the WHERE clause if present
            if let Some(where_clause) = &qpp.where_clause {
                validate_where_clause(where_clause, schema, ctx);
            }

            // Record path variable if present
            if let Some(path_var) = &qpp.path_variable {
                ctx.var_types.insert(path_var.clone(), VarInfo::Path);
            }
        }
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
                        ctx.var_types.insert(
                            var.clone(),
                            VarInfo::Node {
                                label: label.clone(),
                            },
                        );
                        println!(
                            "Recorded node variable: {} -> {:?}",
                            var,
                            ctx.var_types.get(var)
                        ); // Debug: Print recorded node variable
                    } else {
                        // Record node variable without label
                        ctx.var_types.insert(
                            var.clone(),
                            VarInfo::Node {
                                label: "".to_string(),
                            },
                        );
                        println!(
                            "Recorded node variable without label: {} -> {:?}",
                            var,
                            ctx.var_types.get(var)
                        ); // Debug: Print recorded node variable
                    }
                }
                validate_node(node, schema, ctx)
            }
            PatternElement::Relationship(rel) => {
                // Record variable if present
                if let RelationshipPattern::Regular(details) = rel {
                    if let Some(var) = &details.variable {
                        if let Some(rel_type) = &details.rel_type {
                            ctx.var_types.insert(
                                var.clone(),
                                VarInfo::Relationship {
                                    rel_type: rel_type.clone(),
                                },
                            );
                            println!(
                                "Recorded relationship variable: {} -> {:?}",
                                var,
                                ctx.var_types.get(var)
                            ); // Debug: Print recorded relationship variable
                        } else {
                            // Record relationship variable without type
                            ctx.var_types.insert(
                                var.clone(),
                                VarInfo::Relationship {
                                    rel_type: "".to_string(),
                                },
                            );
                            println!(
                                "Recorded relationship variable without type: {} -> {:?}",
                                var,
                                ctx.var_types.get(var)
                            ); // Debug: Print recorded relationship variable
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

/// Validate a WHERE clause
fn validate_where_clause(
    where_clause: &WhereClause,
    schema: &DbSchema,
    ctx: &mut ValidationContext,
) {
    for condition in &where_clause.conditions {
        match condition {
            WhereCondition::Comparison {
                left,
                operator: _,
                right,
            } => {
                validate_property_access(left, schema, ctx);
                // Validate the type of the right-hand side literal
                if let Some((var, prop)) = left.split_once('.') {
                    if let Some(var_info) = ctx.var_types.get(var) {
                        match var_info {
                            VarInfo::Node { label } => {
                                if let Some(prop_info) = schema.get_node_property(label, prop) {
                                    // Check if the right-hand side literal matches the property type
                                    let is_quoted = right.starts_with('\'')
                                        && right.ends_with('\'')
                                        && right.len() >= 2;
                                    let unquoted = if is_quoted {
                                        &right[1..right.len() - 1]
                                    } else {
                                        right.as_str()
                                    };
                                    match prop_info.neo4j_type {
                                        PropertyType::STRING => {
                                            if !is_quoted {
                                                ctx.errors.push(format!(
                                                    "Type mismatch: property '{}' is STRING but got non-string literal",
                                                    prop
                                                ));
                                            }
                                        }
                                        PropertyType::INTEGER => {
                                            if is_quoted || unquoted.parse::<i64>().is_err() {
                                                ctx.errors.push(format!(
                                                    "Type mismatch: property '{}' is INTEGER but got string or non-integer literal",
                                                    prop
                                                ));
                                            }
                                        }
                                        PropertyType::FLOAT => {
                                            if is_quoted || unquoted.parse::<f64>().is_err() {
                                                ctx.errors.push(format!(
                                                    "Type mismatch: property '{}' is FLOAT but got string or non-float literal",
                                                    prop
                                                ));
                                            }
                                        }
                                        PropertyType::BOOLEAN => {
                                            if is_quoted
                                                || !(unquoted == "true" || unquoted == "false")
                                            {
                                                ctx.errors.push(format!(
                                                    "Type mismatch: property '{}' is BOOLEAN but got string or non-boolean literal",
                                                    prop
                                                ));
                                            }
                                        }
                                        PropertyType::DATETIME => {
                                            // Accept both quoted and unquoted for now, but could be stricter
                                            // TODO: Add stricter datetime literal validation if needed
                                        }
                                        PropertyType::POINT => {
                                            // Not supported in WHERE for now
                                        }
                                            // PropertyType::ENUM(_) => {
                                            //     // Not supported in this check for now
                                            // }
                                        }
                                }
                            }
                            VarInfo::Relationship { rel_type } => {
                                if let Some(prop_info) =
                                    schema.get_relationship_property(rel_type, prop)
                                {
                                    let is_quoted = right.starts_with('\'')
                                        && right.ends_with('\'')
                                        && right.len() >= 2;
                                    let unquoted = if is_quoted {
                                        &right[1..right.len() - 1]
                                    } else {
                                        right.as_str()
                                    };
                                    match prop_info.neo4j_type {
                                        PropertyType::STRING => {
                                            if !is_quoted {
                                                ctx.errors.push(format!(
                                                    "Type mismatch: property '{}' is STRING but got non-string literal",
                                                    prop
                                                ));
                                            }
                                        }
                                        PropertyType::INTEGER => {
                                            if is_quoted || unquoted.parse::<i64>().is_err() {
                                                ctx.errors.push(format!(
                                                    "Type mismatch: property '{}' is INTEGER but got string or non-integer literal",
                                                    prop
                                                ));
                                            }
                                        }
                                        PropertyType::FLOAT => {
                                            if is_quoted || unquoted.parse::<f64>().is_err() {
                                                ctx.errors.push(format!(
                                                    "Type mismatch: property '{}' is FLOAT but got string or non-float literal",
                                                    prop
                                                ));
                                            }
                                        }
                                        PropertyType::BOOLEAN => {
                                            if is_quoted
                                                || !(unquoted == "true" || unquoted == "false")
                                            {
                                                ctx.errors.push(format!(
                                                    "Type mismatch: property '{}' is BOOLEAN but got string or non-boolean literal",
                                                    prop
                                                ));
                                            }
                                        }
                                        PropertyType::DATETIME => {
                                            // Accept both quoted and unquoted for now, but could be stricter
                                            // TODO: Add stricter datetime literal validation if needed
                                        }
                                        PropertyType::POINT => {
                                            // Not supported in WHERE for now
                                        }
                                        // PropertyType::ENUM(_) => {
                                        //     // Not supported in this check for now
                                        // }
                                    }
                                }
                            }
                            VarInfo::Path => {
                                // Path property validation if needed
                            }
                        }
                    }
                }
            }
            WhereCondition::FunctionCall {
                function,
                arguments,
            } => {
                // Validate function arguments
                for arg in arguments {
                    validate_property_access(arg, schema, ctx);
                }

                // Validate specific functions
                match function.as_str() {
                    "point.distance" => {
                        if arguments.len() != 2 {
                            ctx.errors
                                .push("point.distance requires exactly 2 arguments".to_string());
                        }
                        // Check that both arguments are point properties
                        for arg in arguments {
                            if !arg.contains('.') {
                                ctx.errors.push(
                                    "point.distance requires point properties from nodes"
                                        .to_string(),
                                );
                            }
                        }
                    }
                    _ => {
                        // TODO: Add validation for other functions
                    }
                }
            }
            WhereCondition::PathProperty { path_var, property } => {
                if let Some(var_info) = ctx.var_types.get(path_var) {
                    match var_info {
                        VarInfo::Path => {
                            // Validate path properties
                            match property.as_str() {
                                "length" | "nodes" | "relationships" => {
                                    // These are valid path properties
                                }
                                _ => {
                                    ctx.errors
                                        .push(format!("Invalid path property '{}'", property));
                                }
                            }
                        }
                        _ => {
                            ctx.errors
                                .push(format!("Variable '{}' is not a path variable", path_var));
                        }
                    }
                } else {
                    ctx.errors
                        .push(format!("Variable '{}' not defined", path_var));
                }
            }
        }
    }
}

/// Validate a property access expression
fn validate_property_access(expr: &str, schema: &DbSchema, ctx: &mut ValidationContext) {
    if let Some((var, prop)) = expr.split_once('.') {
        if let Some(var_info) = ctx.var_types.get(var) {
            match var_info {
                VarInfo::Node { label } => {
                    if !schema.has_node_property(label, prop) {
                        ctx.errors.push(format!(
                            "Property '{}' not in schema for label '{}'",
                            prop, label
                        ));
                    }
                }
                VarInfo::Relationship { rel_type } => {
                    if !schema.has_relationship_property(rel_type, prop) {
                        ctx.errors.push(format!(
                            "Property '{}' not in schema for relationship type '{}'",
                            prop, rel_type
                        ));
                    }
                }
                VarInfo::Path => {
                    // Validate path properties
                    match prop {
                        "length" | "nodes" | "relationships" => {
                            // These are valid path properties
                        }
                        _ => {
                            ctx.errors.push(format!("Invalid path property '{}'", prop));
                        }
                    }
                }
            }
        } else {
            ctx.errors.push(format!("Variable '{}' not defined", var));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_query() {
        let query = "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since";
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

    #[test]
    fn test_quantified_path_pattern() {
        let query = "MATCH (a:Station { name: 'Denmark Hill' })<-[:CALLS_AT]-(d:Stop)
                    ((:Stop)-[:NEXT]->(:Stop)){1,3}
                    (a:Stop)-[:CALLS_AT]->(:Station { name: 'Clapham Junction' })
                    RETURN d.departs AS departureTime, a.arrives AS arrivalTime";
        let schema = DbSchema {
            node_props: std::collections::HashMap::new(),
            rel_props: std::collections::HashMap::new(),
            relationships: vec![],
            metadata: Default::default(),
        };
        let result = validate_cypher_with_schema(query, &schema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_variable_length_relationship() {
        let query = "MATCH (d:Station { name: 'Denmark Hill' })<-[:CALLS_AT]-
                    (n:Stop)-[:NEXT]->{1,10}(m:Stop)-[:CALLS_AT]->
                    (a:Station { name: 'Clapham Junction' })
                    WHERE m.arrives < time('17:18')
                    RETURN n.departs AS departureTime";
        let schema = DbSchema {
            node_props: std::collections::HashMap::new(),
            rel_props: std::collections::HashMap::new(),
            relationships: vec![],
            metadata: Default::default(),
        };
        let result = validate_cypher_with_schema(query, &schema);
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_variable_with_predicate() {
        let mut schema = DbSchema::new();
        schema
            .add_node_property(
                "Station",
                &DbSchemaProperty {
                    name: "name".to_string(),
                    neo4j_type: PropertyType::STRING,
                    ..Default::default()
                },
            )
            .unwrap();
        schema
            .add_node_property(
                "Station",
                &DbSchemaProperty {
                    name: "location".to_string(),
                    neo4j_type: PropertyType::POINT,
                    ..Default::default()
                },
            )
            .unwrap();
        schema
            .add_relationship_property(
                "LINK",
                &DbSchemaProperty {
                    name: "distance".to_string(),
                    neo4j_type: PropertyType::FLOAT,
                    ..Default::default()
                },
            )
            .unwrap();

        let query = "MATCH (bfr:Station {name: 'London Blackfriars'}),
                    (ndl:Station {name: 'North Dulwich'})
                    MATCH p = (bfr)
                    ((a)-[:LINK]-(b:Station)
                    WHERE point.distance(a.location, ndl.location) >
                    point.distance(b.location, ndl.location))+ (ndl)
                    RETURN reduce(acc = 0, r in relationships(p) | round(acc + r.distance, 2))
                    AS distance";
        let result = validate_cypher_with_schema(query, &schema);
        assert!(result.is_ok());
    }
}
