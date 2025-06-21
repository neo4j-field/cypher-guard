mod errors;
pub mod parser {
    pub mod ast;
    pub mod clauses;
    pub mod components;
    pub mod patterns;
    pub mod utils;
}
mod schema;

use errors::convert_nom_error;
pub use errors::{
    CypherGuardError, CypherGuardParsingError, CypherGuardSchemaError, CypherGuardValidationError,
};
pub use schema::{
    DbSchema, DbSchemaConstraint, DbSchemaIndex, DbSchemaMetadata, DbSchemaProperty,
    DbSchemaRelationshipPattern, PropertyType,
};

use parser::ast::*;
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

/// Parse a Cypher query with custom error handling
pub fn parse_query(query: &str) -> std::result::Result<Query, CypherGuardParsingError> {
    match parser::clauses::parse_query(query) {
        Ok((_, ast)) => Ok(ast),
        Err(e) => Err(convert_nom_error(e, "query", query)),
    }
}

/// Validate full query with schema: returns true if valid, or error on parse failure
pub fn validate_cypher_with_schema(query: &str, schema: &DbSchema) -> Result<bool> {
    println!("Validating query: {}", query);
    println!("Schema: {:?}", schema);
    let ast = parse_query(query).map_err(|e| {
        println!("Parser error: {:?}", e);
        CypherGuardError::Parsing(e)
    })?;
    println!("AST: {:?}", ast);
    let mut ctx = ValidationContext {
        errors: Vec::new(),
        current_aliases: HashSet::new(),
        var_types: HashMap::new(),
        schema,
    };
    validate_query(&ast, &mut ctx);
    if ctx.errors.is_empty() {
        Ok(true)
    } else {
        Err(CypherGuardError::InvalidQuery(ctx.errors.join(", ")))
    }
}

/// Main validation routine: parse and traverse the AST
pub fn get_cypher_validation_errors(query: &str, schema: &DbSchema) -> Vec<String> {
    match parse_query(query) {
        Ok(ast) => {
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

            // Validate WITH clause if present
            if let Some(with_clause) = &ast.with_clause {
                validate_with_clause(with_clause, &mut ctx);
            }

            // Validate WHERE clause if present
            if let Some(where_clause) = &ast.where_clause {
                let _ = validate_where_clause(where_clause, schema, &mut ctx);
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
    println!("DEBUG: Entered validate_query");
    // Validate match clause
    if let Some(match_clause) = &query.match_clause {
        for el in &match_clause.elements {
            validate_match_element(el, ctx.schema, ctx);
        }
    }

    // Validate WITH clause if present
    if let Some(with_clause) = &query.with_clause {
        validate_with_clause(with_clause, ctx);
    }

    // Validate WHERE clause if present
    if let Some(where_clause) = &query.where_clause {
        for condition in &where_clause.conditions {
            match condition {
                WhereCondition::Comparison {
                    left,
                    operator: _,
                    right,
                } => {
                    println!(
                        "[validate_query] Comparison arm: left='{}', right='{}'",
                        left, right
                    );
                    // Validate property access
                    if let Some((var, prop)) = left.split_once('.') {
                        println!("DEBUG: Found property access: {}.{}", var, prop);
                        if let Some(var_info) = ctx.var_types.get(var) {
                            match var_info {
                                VarInfo::Node { label } => {
                                    if let Some(prop_info) =
                                        ctx.schema.get_node_property(label, prop)
                                    {
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
                                            PropertyType::DateTime => {
                                                // Accept both quoted and unquoted for now, but could be stricter
                                                // TODO: Add stricter DATE_TIME literal validation if needed
                                            }
                                            PropertyType::POINT => {
                                                // Not supported in WHERE for now
                                            } // PropertyType::ENUM(_) => {
                                            //     // Not supported in this check for now
                                            // }
                                            PropertyType::LIST => {
                                                // Not supported in WHERE for now
                                            }
                                        }
                                    }
                                }
                                VarInfo::Relationship { rel_type } => {
                                    if let Some(prop_info) =
                                        ctx.schema.get_relationship_property(rel_type, prop)
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
                                            PropertyType::DateTime => {
                                                // Accept both quoted and unquoted for now, but could be stricter
                                                // TODO: Add stricter DATE_TIME literal validation if needed
                                            }
                                            PropertyType::POINT => {
                                                // Not supported in WHERE for now
                                            } // PropertyType::ENUM(_) => {
                                            //     // Not supported in this check for now
                                            // }
                                            PropertyType::LIST => {
                                                // Not supported in WHERE for now
                                            }
                                        }
                                    }
                                }
                                VarInfo::Path => {
                                    println!("DEBUG: Path property validation not implemented");
                                }
                            }
                        } else {
                            println!("DEBUG: Variable not found: {}", var);
                            ctx.errors.push("Validation failed".to_string());
                            return;
                        }
                    }
                }
                WhereCondition::FunctionCall { .. } | WhereCondition::PathProperty { .. } => {
                    // Already handled in validate_where_clause
                }
                WhereCondition::And(left, right) => {
                    validate_where_condition(left, ctx.schema, ctx).unwrap_or(false);
                    validate_where_condition(right, ctx.schema, ctx).unwrap_or(false);
                }
                WhereCondition::Or(left, right) => {
                    validate_where_condition(left, ctx.schema, ctx).unwrap_or(false);
                    validate_where_condition(right, ctx.schema, ctx).unwrap_or(false);
                }
                WhereCondition::Not(cond) => {
                    validate_where_condition(cond, ctx.schema, ctx).unwrap_or(false);
                }
                WhereCondition::Parenthesized(cond) => {
                    validate_where_condition(cond, ctx.schema, ctx).unwrap_or(false);
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

/// Validate a WITH clause
fn validate_with_clause(with_clause: &WithClause, ctx: &mut ValidationContext) {
    println!("DEBUG: Entered validate_with_clause");
    println!("DEBUG: Starting WITH clause validation");
    println!("DEBUG: Current variable scope: {:?}", ctx.var_types);
    println!("DEBUG: WITH clause items: {:?}", with_clause.items);

    // First, check all items for existence in the current scope
    let mut new_var_types = HashMap::new();
    let mut seen_aliases = HashSet::new();
    let mut has_errors = false;

    for item in &with_clause.items {
        println!("DEBUG: Processing WITH item: {:?}", item);
        match &item.expression {
            WithExpression::Identifier(var) => {
                println!("DEBUG: Checking identifier: {}", var);
                println!(
                    "DEBUG: Current var_types keys: {:?}",
                    ctx.var_types.keys().collect::<Vec<_>>()
                );
                if !ctx.var_types.contains_key(var) {
                    println!("DEBUG: Variable {} not found in scope", var);
                    ctx.errors
                        .push(format!("Variable '{}' not defined in previous scope", var));
                    println!("DEBUG: ctx.errors after push: {:?}", ctx.errors);
                    has_errors = true;
                } else if let Some(var_info) = ctx.var_types.get(var) {
                    println!("DEBUG: Found variable info: {:?}", var_info);
                    if let Some(alias) = &item.alias {
                        println!("DEBUG: Adding alias {} with info {:?}", alias, var_info);
                        new_var_types.insert(alias.clone(), var_info.clone());
                        seen_aliases.insert(alias.clone());
                    } else {
                        println!("DEBUG: Adding variable {} with info {:?}", var, var_info);
                        new_var_types.insert(var.clone(), var_info.clone());
                    }
                }
            }
            WithExpression::PropertyAccess { variable, property } => {
                println!("DEBUG: Checking property access: {}.{}", variable, property);
                if let Some(var_info) = ctx.var_types.get(variable) {
                    println!("DEBUG: Found variable info: {:?}", var_info);
                    if let Some(alias) = &item.alias {
                        println!("DEBUG: Adding alias {} with info {:?}", alias, var_info);
                        new_var_types.insert(alias.clone(), var_info.clone());
                        seen_aliases.insert(alias.clone());
                    }
                } else {
                    println!("DEBUG: Variable {} not found in scope", variable);
                    ctx.errors.push(format!(
                        "Variable '{}' not defined in previous scope",
                        variable
                    ));
                    has_errors = true;
                }
            }
            WithExpression::FunctionCall { name, args } => {
                println!("DEBUG: Checking function call: {}({:?})", name, args);
                let mut args_valid = true;
                // For now, we'll just validate that all arguments exist in scope
                for arg in args {
                    if let WithExpression::Identifier(var) = arg {
                        println!("DEBUG: Checking function argument: {}", var);
                        if !ctx.var_types.contains_key(var) {
                            println!("DEBUG: Argument {} not found in scope", var);
                            ctx.errors
                                .push(format!("Argument '{}' not defined in previous scope", var));
                            args_valid = false;
                        }
                    }
                }
                if args_valid {
                    if let Some(alias) = &item.alias {
                        println!("DEBUG: Adding function result alias: {}", alias);
                        // For function calls, we'll create a new variable type
                        new_var_types.insert(
                            alias.clone(),
                            VarInfo::Node {
                                label: "".to_string(),
                            },
                        );
                        seen_aliases.insert(alias.clone());
                    }
                } else {
                    has_errors = true;
                }
            }
            WithExpression::Wildcard => {
                println!("DEBUG: Processing wildcard");
                new_var_types.extend(ctx.var_types.clone());
            }
        }
    }

    println!("DEBUG: New variable scope: {:?}", new_var_types);
    println!("DEBUG: Current errors: {:?}", ctx.errors);
    println!("DEBUG: Has errors: {}", has_errors);

    // Only update the variable scope if we didn't find any errors
    if !has_errors {
        println!("DEBUG: Updating variable scope");
        ctx.var_types = new_var_types;
    } else {
        println!("DEBUG: Not updating variable scope due to errors");
    }
}

/// Validate one MatchElement
fn validate_match_element(el: &MatchElement, schema: &DbSchema, ctx: &mut ValidationContext) {
    // Validate the pattern
    validate_pattern(&el.pattern, schema, ctx);

    // Record path variable if present
    if let Some(path_var) = &el.path_var {
        ctx.var_types.insert(path_var.clone(), VarInfo::Path);
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
            PatternElement::QuantifiedPathPattern(qpp) => {
                // Recursively validate the inner pattern
                validate_pattern(&qpp.pattern, schema, ctx);
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
    if let Some(rel_type) = rel.rel_type() {
        if !schema.has_relationship_type(rel_type) {
            ctx.errors
                .push(format!("Relationship type '{}' not in schema", rel_type));
        } else {
            // Validate relationship direction
            let direction = rel.direction();
            let relationships = schema
                .relationships
                .iter()
                .filter(|r| r.rel_type == rel_type)
                .collect::<Vec<_>>();

            if !relationships.is_empty() {
                let valid_direction = match direction {
                    Direction::Right => relationships.iter().any(|r| r.start != r.end),
                    Direction::Left => relationships.iter().any(|r| r.start != r.end),
                    Direction::Undirected => relationships.iter().any(|r| r.start == r.end),
                };

                if !valid_direction {
                    ctx.errors.push(format!(
                        "Invalid direction for relationship type '{}'. Expected {}",
                        rel_type,
                        match direction {
                            Direction::Right => "right-directed",
                            Direction::Left => "left-directed",
                            Direction::Undirected => "undirected",
                        }
                    ));
                }
            }
        }

        if let Some(props) = rel.properties() {
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

/// Validate a WHERE clause
fn validate_where_clause(
    where_clause: &WhereClause,
    _schema: &DbSchema,
    ctx: &mut ValidationContext,
) -> Result<bool> {
    println!("DEBUG: Starting validate_where_clause");
    for condition in &where_clause.conditions {
        println!("DEBUG: Processing condition: {:?}", condition);
        match condition {
            WhereCondition::Comparison {
                left,
                operator: _,
                right,
            } => {
                println!(
                    "[validate_where_clause] Comparison arm: left='{}', right='{}'",
                    left, right
                );
                // Validate property access
                if let Some((var, prop)) = left.split_once('.') {
                    println!("DEBUG: Found property access: {}.{}", var, prop);
                    if let Some(var_info) = ctx.var_types.get(var) {
                        match var_info {
                            VarInfo::Node { label } => {
                                if let Some(prop_info) = ctx.schema.get_node_property(label, prop) {
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
                                        PropertyType::DateTime => {
                                            // Accept both quoted and unquoted for now, but could be stricter
                                            // TODO: Add stricter DATE_TIME literal validation if needed
                                        }
                                        PropertyType::POINT => {
                                            // Not supported in WHERE for now
                                        } // PropertyType::ENUM(_) => {
                                        //     // Not supported in this check for now
                                        // }
                                        PropertyType::LIST => {
                                            // Not supported in WHERE for now
                                        }
                                    }
                                }
                            }
                            VarInfo::Relationship { rel_type } => {
                                if let Some(prop_info) =
                                    ctx.schema.get_relationship_property(rel_type, prop)
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
                                        PropertyType::DateTime => {
                                            // Accept both quoted and unquoted for now, but could be stricter
                                            // TODO: Add stricter DATE_TIME literal validation if needed
                                        }
                                        PropertyType::POINT => {
                                            // Not supported in WHERE for now
                                        } // PropertyType::ENUM(_) => {
                                        //     // Not supported in this check for now
                                        // }
                                        PropertyType::LIST => {
                                            // Not supported in WHERE for now
                                        }
                                    }
                                }
                            }
                            VarInfo::Path => {
                                println!("DEBUG: Path property validation not implemented");
                            }
                        }
                    } else {
                        println!("DEBUG: Variable not found: {}", var);
                        ctx.errors.push("Validation failed".to_string());
                        return Ok(false);
                    }
                }
            }
            WhereCondition::FunctionCall { .. } => {
                // Already handled in validate_where_condition
            }
            WhereCondition::PathProperty { path_var, property } => {
                println!("DEBUG: Processing path property: {}.{}", path_var, property);
                if let Some(var_info) = ctx.var_types.get(path_var) {
                    match var_info {
                        VarInfo::Path => {
                            println!("DEBUG: Validating path property: {}", property);
                            match property.as_str() {
                                "length" | "nodes" | "relationships" => {
                                    println!("DEBUG: Valid path property");
                                    return Ok(true);
                                }
                                _ => {
                                    println!("DEBUG: Invalid path property");
                                    return Ok(false);
                                }
                            }
                        }
                        _ => {
                            println!("DEBUG: Not a path variable");
                            return Ok(false);
                        }
                    }
                } else {
                    println!("DEBUG: Path variable not found");
                    return Ok(false);
                }
            }
            WhereCondition::And(left, right) => {
                println!("DEBUG: Processing AND condition");
                let left_result = validate_where_condition(left, ctx.schema, ctx)?;
                let right_result = validate_where_condition(right, ctx.schema, ctx)?;
                println!("DEBUG: AND result: {} && {}", left_result, right_result);
                return Ok(left_result && right_result);
            }
            WhereCondition::Or(left, right) => {
                println!("DEBUG: Processing OR condition");
                let left_result = validate_where_condition(left, ctx.schema, ctx)?;
                let right_result = validate_where_condition(right, ctx.schema, ctx)?;
                println!("DEBUG: OR result: {} || {}", left_result, right_result);
                return Ok(left_result || right_result);
            }
            WhereCondition::Not(cond) => {
                println!("DEBUG: Processing NOT condition");
                let result = validate_where_condition(cond, ctx.schema, ctx)?;
                println!("DEBUG: NOT result: !{}", result);
                return Ok(!result);
            }
            WhereCondition::Parenthesized(cond) => {
                println!("DEBUG: Processing parenthesized condition");
                return validate_where_condition(cond, ctx.schema, ctx);
            }
        }
    }
    println!("DEBUG: All conditions valid");
    Ok(true)
}

/// Validate a property access expression
fn validate_property_access(
    expr: &str,
    schema: &DbSchema,
    ctx: &mut ValidationContext,
) -> Result<()> {
    if let Some((var, prop)) = expr.split_once('.') {
        if let Some(var_info) = ctx.var_types.get(var) {
            match var_info {
                VarInfo::Node { label } => {
                    if !schema.has_node_property(label, prop) {
                        return Err(CypherGuardError::Validation(
                            CypherGuardValidationError::invalid_property_name(prop),
                        ));
                    }
                }
                VarInfo::Relationship { rel_type } => {
                    if !schema.has_relationship_property(rel_type, prop) {
                        return Err(CypherGuardError::Validation(
                            CypherGuardValidationError::invalid_property_name(prop),
                        ));
                    }
                }
                VarInfo::Path => {
                    // Path property validation if needed
                }
            }
        } else {
            return Err(CypherGuardError::Validation(
                CypherGuardValidationError::undefined_variable(var),
            ));
        }
    }
    Ok(())
}

/// Validate a WHERE condition
fn validate_where_condition(
    condition: &WhereCondition,
    schema: &DbSchema,
    ctx: &mut ValidationContext,
) -> Result<bool> {
    println!("DEBUG: Starting validate_where_condition");
    match condition {
        WhereCondition::Comparison {
            left,
            operator: _,
            right,
        } => {
            println!(
                "[validate_where_condition] Comparison arm: left='{}', right='{}'",
                left, right
            );
            // Validate property access
            if let Some((var, prop)) = left.split_once('.') {
                println!("DEBUG: Found property access: {}.{}", var, prop);
                if let Some(var_info) = ctx.var_types.get(var) {
                    match var_info {
                        VarInfo::Node { label } => {
                            println!("DEBUG: Checking node property: {}.{}", label, prop);
                            if let Some(prop_info) = ctx.schema.get_node_property(label, prop) {
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
                                            return Ok(false);
                                        }
                                    }
                                    PropertyType::INTEGER => {
                                        if is_quoted || unquoted.parse::<i64>().is_err() {
                                            ctx.errors.push(format!(
                                                "Type mismatch: property '{}' is INTEGER but got string or non-integer literal",
                                                prop
                                            ));
                                            return Ok(false);
                                        }
                                    }
                                    PropertyType::FLOAT => {
                                        if is_quoted || unquoted.parse::<f64>().is_err() {
                                            ctx.errors.push(format!(
                                                "Type mismatch: property '{}' is FLOAT but got string or non-float literal",
                                                prop
                                            ));
                                            return Ok(false);
                                        }
                                    }
                                    PropertyType::BOOLEAN => {
                                        if is_quoted || !(unquoted == "true" || unquoted == "false")
                                        {
                                            ctx.errors.push(format!(
                                                "Type mismatch: property '{}' is BOOLEAN but got string or non-boolean literal",
                                                prop
                                            ));
                                            return Ok(false);
                                        }
                                    }
                                    PropertyType::DateTime => {
                                        // Accept both quoted and unquoted for now
                                    }
                                    PropertyType::POINT => {
                                        // Not supported in WHERE for now
                                    }
                                    PropertyType::LIST => {
                                        // Not supported in WHERE for now
                                    }
                                }
                            } else {
                                println!("DEBUG: Invalid node property");
                                return Ok(false);
                            }
                        }
                        VarInfo::Relationship { rel_type } => {
                            println!(
                                "DEBUG: Checking relationship property: {}.{}",
                                rel_type, prop
                            );
                            if let Some(prop_info) =
                                ctx.schema.get_relationship_property(rel_type, prop)
                            {
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
                                            return Ok(false);
                                        }
                                    }
                                    PropertyType::INTEGER => {
                                        if is_quoted || unquoted.parse::<i64>().is_err() {
                                            ctx.errors.push(format!(
                                                "Type mismatch: property '{}' is INTEGER but got string or non-integer literal",
                                                prop
                                            ));
                                            return Ok(false);
                                        }
                                    }
                                    PropertyType::FLOAT => {
                                        if is_quoted || unquoted.parse::<f64>().is_err() {
                                            ctx.errors.push(format!(
                                                "Type mismatch: property '{}' is FLOAT but got string or non-float literal",
                                                prop
                                            ));
                                            return Ok(false);
                                        }
                                    }
                                    PropertyType::BOOLEAN => {
                                        if is_quoted || !(unquoted == "true" || unquoted == "false")
                                        {
                                            ctx.errors.push(format!(
                                                "Type mismatch: property '{}' is BOOLEAN but got string or non-boolean literal",
                                                prop
                                            ));
                                            return Ok(false);
                                        }
                                    }
                                    PropertyType::DateTime => {
                                        // Accept both quoted and unquoted for now
                                    }
                                    PropertyType::POINT => {
                                        // Not supported in WHERE for now
                                    }
                                    PropertyType::LIST => {
                                        // Not supported in WHERE for now
                                    }
                                }
                            } else {
                                println!("DEBUG: Invalid relationship property");
                                return Ok(false);
                            }
                        }
                        VarInfo::Path => {
                            println!("DEBUG: Path property validation not implemented");
                            return Ok(false);
                        }
                    }
                } else {
                    println!("DEBUG: Variable not found: {}", var);
                    return Ok(false);
                }
            }
            Ok(true)
        }
        WhereCondition::FunctionCall {
            function,
            arguments,
        } => {
            println!(
                "DEBUG: Processing function call: {}({:?})",
                function, arguments
            );
            // Validate function arguments
            for arg in arguments {
                println!("DEBUG: Validating function argument: {}", arg);
                validate_property_access(arg, schema, ctx)?;
            }
            Ok(true)
        }
        WhereCondition::PathProperty { path_var, property } => {
            println!("DEBUG: Processing path property: {}.{}", path_var, property);
            if let Some(var_info) = ctx.var_types.get(path_var) {
                match var_info {
                    VarInfo::Path => {
                        println!("DEBUG: Validating path property: {}", property);
                        match property.as_str() {
                            "length" | "nodes" | "relationships" => {
                                println!("DEBUG: Valid path property");
                                Ok(true)
                            }
                            _ => {
                                println!("DEBUG: Invalid path property");
                                Ok(false)
                            }
                        }
                    }
                    _ => {
                        println!("DEBUG: Not a path variable");
                        Ok(false)
                    }
                }
            } else {
                println!("DEBUG: Path variable not found");
                Ok(false)
            }
        }
        WhereCondition::And(left, right) => {
            println!("DEBUG: Processing AND condition");
            let left_result = validate_where_condition(left, schema, ctx)?;
            let right_result = validate_where_condition(right, schema, ctx)?;
            println!("DEBUG: AND result: {} && {}", left_result, right_result);
            Ok(left_result && right_result)
        }
        WhereCondition::Or(left, right) => {
            println!("DEBUG: Processing OR condition");
            let left_result = validate_where_condition(left, schema, ctx)?;
            let right_result = validate_where_condition(right, schema, ctx)?;
            println!("DEBUG: OR result: {} || {}", left_result, right_result);
            Ok(left_result || right_result)
        }
        WhereCondition::Not(cond) => {
            println!("DEBUG: Processing NOT condition");
            let result = validate_where_condition(cond, schema, ctx)?;
            println!("DEBUG: NOT result: !{}", result);
            Ok(!result)
        }
        WhereCondition::Parenthesized(cond) => {
            println!("DEBUG: Processing parenthesized condition");
            validate_where_condition(cond, schema, ctx)
        }
    }
}

impl<'a> ValidationContext<'a> {
    pub fn new(schema: &'a DbSchema) -> Self {
        Self {
            errors: Vec::new(),
            current_aliases: HashSet::new(),
            var_types: HashMap::new(),
            schema,
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
