use crate::errors::CypherGuardValidationError;
use crate::parser::ast::*;
use crate::schema::DbSchema;
use std::collections::{HashMap, HashSet};

/// Represents the extracted elements from a Cypher query that need validation
#[derive(Debug, Clone)]
pub struct QueryElements {
    pub node_labels: HashSet<String>,
    pub relationship_types: HashSet<String>,
    pub node_properties: HashMap<String, HashSet<String>>, // label -> set of property names
    pub relationship_properties: HashMap<String, HashSet<String>>, // rel_type -> set of property names
    pub property_accesses: Vec<PropertyAccess>,                    // Property access with context
    pub defined_variables: HashSet<String>, // Variables that are defined (from MATCH, UNWIND, etc.)
    pub referenced_variables: HashSet<String>, // Variables that are referenced (from WITH, WHERE, RETURN, etc.)
}

#[derive(Debug, Clone)]
pub struct PropertyAccess {
    pub variable: String,
    pub property: String,
    pub context: PropertyContext,
}

#[derive(Debug, Clone)]
pub enum PropertyContext {
    Where,
    Return,
    With,
}

impl QueryElements {
    pub fn new() -> Self {
        Self {
            node_labels: HashSet::new(),
            relationship_types: HashSet::new(),
            node_properties: HashMap::new(),
            relationship_properties: HashMap::new(),
            property_accesses: Vec::new(),
            defined_variables: HashSet::new(),
            referenced_variables: HashSet::new(),
        }
    }

    /// Add a node label to the set
    pub fn add_node_label(&mut self, label: String) {
        self.node_labels.insert(label);
    }

    /// Add a relationship type to the set
    pub fn add_relationship_type(&mut self, rel_type: String) {
        self.relationship_types.insert(rel_type);
    }

    /// Add a node property to the set
    pub fn add_node_property(&mut self, label: String, property: String) {
        self.node_properties
            .entry(label)
            .or_default()
            .insert(property);
    }

    /// Add a relationship property to the set
    pub fn add_relationship_property(&mut self, rel_type: String, property: String) {
        self.relationship_properties
            .entry(rel_type)
            .or_default()
            .insert(property);
    }

    /// Add a variable to the set
    pub fn add_variable(&mut self, variable: String) {
        self.referenced_variables.insert(variable);
    }

    /// Add a variable that is defined
    pub fn add_defined_variable(&mut self, variable: String) {
        self.defined_variables.insert(variable);
    }

    /// Add property access with context
    pub fn add_property_access(&mut self, access: PropertyAccess) {
        self.property_accesses.push(access);
    }

    /// Add an undefined variable reference
    #[allow(dead_code)]
    pub fn add_undefined_variable(&mut self, variable: String) {
        self.referenced_variables.insert(variable);
    }
}

/// Extract all elements from a parsed query that need validation
pub fn extract_query_elements(query: &Query) -> QueryElements {
    let mut elements = QueryElements::new();

    // Extract from MATCH clauses
    for match_clause in &query.match_clauses {
        for element in &match_clause.elements {
            extract_from_match_element(element, &mut elements);
        }
    }

    // Extract from MERGE clauses
    for merge_clause in &query.merge_clauses {
        for element in &merge_clause.elements {
            extract_from_match_element(element, &mut elements);
        }
    }

    // Extract from CREATE clauses
    for create_clause in &query.create_clauses {
        for element in &create_clause.elements {
            extract_from_match_element(element, &mut elements);
        }
    }

    // Extract from WHERE clauses
    for where_clause in &query.where_clauses {
        for condition in &where_clause.conditions {
            extract_from_where_condition(condition, &mut elements);
        }
    }

    // Extract from RETURN clauses
    for return_clause in &query.return_clauses {
        for item in &return_clause.items {
            extract_from_return_item(item, &mut elements);
        }
    }

    // Extract from WITH clauses
    for with_clause in &query.with_clauses {
        for item in &with_clause.items {
            extract_from_with_item(item, &mut elements);
        }
    }

    // Extract from UNWIND clauses
    for unwind_clause in &query.unwind_clauses {
        elements.add_defined_variable(unwind_clause.variable.clone());
        // Optionally, could track type info here in the future
    }

    elements
}

/// Extract elements from a single match element
fn extract_from_match_element(element: &MatchElement, elements: &mut QueryElements) {
    // Extract the path variable if it exists
    if let Some(path_var) = &element.path_var {
        elements.add_defined_variable(path_var.clone());
    }

    for pattern_element in &element.pattern {
        match pattern_element {
            PatternElement::Node(node) => {
                // Extract variable from node
                if let Some(variable) = &node.variable {
                    elements.add_defined_variable(variable.clone());
                }

                if let Some(label) = &node.label {
                    elements.add_node_label(label.clone());

                    // Extract properties from node pattern
                    if let Some(properties) = &node.properties {
                        for prop in properties {
                            elements.add_node_property(label.clone(), prop.key.clone());
                        }
                    }
                }
            }
            PatternElement::Relationship(rel) => {
                // Extract variable from relationship
                match rel {
                    RelationshipPattern::Regular(details)
                    | RelationshipPattern::OptionalRelationship(details) => {
                        if let Some(variable) = &details.variable {
                            elements.add_defined_variable(variable.clone());
                        }
                    }
                }

                if let Some(rel_type) = rel.rel_type() {
                    elements.add_relationship_type(rel_type.to_string());

                    // Extract properties from relationship pattern
                    if let Some(properties) = rel.properties() {
                        for prop in properties {
                            elements
                                .add_relationship_property(rel_type.to_string(), prop.key.clone());
                        }
                    }
                }
            }
            PatternElement::QuantifiedPathPattern(qpp) => {
                // Extract path variable if it exists
                if let Some(path_var) = &qpp.path_variable {
                    elements.add_defined_variable(path_var.clone());
                }

                // Recursively extract from the inner pattern
                for pattern_element in &qpp.pattern {
                    match pattern_element {
                        PatternElement::Node(node) => {
                            if let Some(variable) = &node.variable {
                                elements.add_defined_variable(variable.clone());
                            }
                            if let Some(label) = &node.label {
                                elements.add_node_label(label.clone());
                            }
                        }
                        PatternElement::Relationship(rel) => {
                            match rel {
                                RelationshipPattern::Regular(details)
                                | RelationshipPattern::OptionalRelationship(details) => {
                                    if let Some(variable) = &details.variable {
                                        elements.add_defined_variable(variable.clone());
                                    }
                                }
                            }
                            if let Some(rel_type) = rel.rel_type() {
                                elements.add_relationship_type(rel_type.to_string());
                            }
                        }
                        PatternElement::QuantifiedPathPattern(_) => {
                            // Handle nested QPPs if needed
                        }
                    }
                }
            }
        }
    }
}

/// Extract elements from a WHERE condition
fn extract_from_where_condition(condition: &WhereCondition, elements: &mut QueryElements) {
    match condition {
        WhereCondition::Comparison { left, right, .. } => {
            extract_property_access_from_string(left, elements, PropertyContext::Where);
            extract_property_access_from_string(right, elements, PropertyContext::Where);
        }
        WhereCondition::FunctionCall { arguments, .. } => {
            for arg in arguments {
                extract_property_access_from_string(arg, elements, PropertyContext::Where);
            }
        }
        WhereCondition::PathProperty { path_var, property } => {
            elements.add_variable(path_var.clone());
            elements.add_property_access(PropertyAccess {
                variable: path_var.clone(),
                property: property.clone(),
                context: PropertyContext::Where,
            });
        }
        WhereCondition::And(left, right) => {
            extract_from_where_condition(left, elements);
            extract_from_where_condition(right, elements);
        }
        WhereCondition::Or(left, right) => {
            extract_from_where_condition(left, elements);
            extract_from_where_condition(right, elements);
        }
        WhereCondition::Not(condition) => {
            extract_from_where_condition(condition, elements);
        }
        WhereCondition::Parenthesized(condition) => {
            extract_from_where_condition(condition, elements);
        }
    }
}

/// Extract elements from a RETURN item
fn extract_from_return_item(item: &str, elements: &mut QueryElements) {
    extract_property_access_from_string(item, elements, PropertyContext::Return);
}

/// Extract elements from a WITH item
fn extract_from_with_item(item: &WithItem, elements: &mut QueryElements) {
    extract_from_with_expression(&item.expression, elements);
}

/// Extract elements from a WITH expression
fn extract_from_with_expression(expression: &WithExpression, elements: &mut QueryElements) {
    match expression {
        WithExpression::Identifier(var) => {
            elements.add_variable(var.clone());
        }
        WithExpression::PropertyAccess { variable, property } => {
            elements.add_variable(variable.clone());
            elements.add_property_access(PropertyAccess {
                variable: variable.clone(),
                property: property.clone(),
                context: PropertyContext::With,
            });
        }
        WithExpression::FunctionCall { args, .. } => {
            for arg in args {
                extract_from_with_expression(arg, elements);
            }
        }
        WithExpression::Wildcard => {
            // No specific extraction needed for wildcard
        }
    }
}

/// Extract property access from a string (e.g., "a.name", "r.role")
fn extract_property_access_from_string(
    s: &str,
    elements: &mut QueryElements,
    context: PropertyContext,
) {
    // Simple pattern matching for property access: variable.property
    if let Some(dot_pos) = s.find('.') {
        let variable = s[..dot_pos].trim();
        let property = s[dot_pos + 1..].trim();

        if !variable.is_empty() && !property.is_empty() {
            elements.add_variable(variable.to_string());
            elements.add_property_access(PropertyAccess {
                variable: variable.to_string(),
                property: property.to_string(),
                context,
            });
        }
    } else {
        // Check if it's just a variable reference
        let trimmed = s.trim();
        if !trimmed.is_empty() && !trimmed.contains(' ') {
            elements.add_variable(trimmed.to_string());
        }
    }
}

/// Validate extracted query elements against the schema
pub fn validate_query_elements(
    elements: &QueryElements,
    schema: &DbSchema,
) -> Vec<CypherGuardValidationError> {
    let mut errors = Vec::new();

    // Validate node labels
    for label in &elements.node_labels {
        if !schema.has_label(label) {
            errors.push(CypherGuardValidationError::InvalidNodeLabel(label.clone()));
        }
    }

    // Validate relationship types
    for rel_type in &elements.relationship_types {
        if !schema.has_relationship_type(rel_type) {
            errors.push(CypherGuardValidationError::InvalidRelationshipType(
                rel_type.clone(),
            ));
        }
    }

    // Validate node properties
    for (label, properties) in &elements.node_properties {
        if !schema.has_label(label) {
            errors.push(CypherGuardValidationError::InvalidNodeLabel(label.clone()));
            continue;
        }
        for property in properties {
            if !schema.has_node_property(label, property) {
                errors.push(CypherGuardValidationError::InvalidNodeProperty {
                    label: label.clone(),
                    property: property.clone(),
                });
            }
        }
    }

    // Validate relationship properties
    for (rel_type, properties) in &elements.relationship_properties {
        if !schema.has_relationship_type(rel_type) {
            errors.push(CypherGuardValidationError::InvalidRelationshipType(
                rel_type.clone(),
            ));
            continue;
        }
        for property in properties {
            if !schema.has_relationship_property(rel_type, property) {
                errors.push(CypherGuardValidationError::InvalidRelationshipProperty {
                    rel_type: rel_type.clone(),
                    property: property.clone(),
                });
            }
        }
    }

    // Validate property access from all contexts
    for access in &elements.property_accesses {
        let context_str = match access.context {
            PropertyContext::Where => "WHERE",
            PropertyContext::Return => "RETURN",
            PropertyContext::With => "WITH",
        };

        // For now, we'll check if the property exists anywhere in the schema
        // In a more sophisticated implementation, we would track variable types
        // and check if the property exists for that specific type
        let mut found = false;

        // Check if the property exists in any node label
        for properties in schema.node_props.values() {
            if properties.iter().any(|p| p.name == access.property) {
                found = true;
                break;
            }
        }

        // If not found in nodes, check relationship properties
        if !found {
            for properties in schema.rel_props.values() {
                if properties.iter().any(|p| p.name == access.property) {
                    found = true;
                    break;
                }
            }
        }

        if !found {
            errors.push(CypherGuardValidationError::InvalidPropertyAccess {
                variable: access.variable.clone(),
                property: access.property.clone(),
                context: context_str.to_string(),
            });
        }
    }

    // Validate undefined variables
    for variable in &elements.referenced_variables {
        if !elements.defined_variables.contains(variable) {
            errors.push(CypherGuardValidationError::UndefinedVariable(
                variable.clone(),
            ));
        }
    }

    errors
}

#[allow(dead_code)]
/// Main entry point for full Cypher query validation.
///
/// This function performs all clause-level, cross-clause, and schema-level validation.
/// As new Cypher features are added (e.g., clause order, type inference, parameter checks),
/// extend this function to keep validation logic centralized and maintainable.
pub fn validate_query(
    query: &Query,
    elements: &QueryElements,
    schema: &DbSchema,
) -> Vec<CypherGuardValidationError> {
    let mut errors = Vec::new();

    // Basic UNWIND validation: check type of UNWIND expression
    for unwind_clause in &query.unwind_clauses {
        use crate::parser::ast::UnwindExpression;
        match &unwind_clause.expression {
            UnwindExpression::List(_) => {
                // Lists are always valid for UNWIND
            }
            UnwindExpression::Identifier(_) => {
                // Identifiers are valid (they should be variables)
            }
            UnwindExpression::FunctionCall { .. } => {
                // Function calls are valid
            }
            UnwindExpression::Parameter(_) => {
                // Parameters are valid
            }
        }
    }

    // Existing validation logic
    errors.extend(validate_query_elements(elements, schema));
    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{PropertyValue, Query, UnwindClause, UnwindExpression};
    use crate::schema::{DbSchema, DbSchemaProperty, DbSchemaRelationshipPattern, PropertyType};

    fn create_test_schema() -> DbSchema {
        let mut schema = DbSchema::new();

        // Add node labels and properties
        schema.add_label("Person").unwrap();
        schema.add_label("Place").unwrap();

        let name_prop = DbSchemaProperty::new("name", PropertyType::STRING);
        let age_prop = DbSchemaProperty::new("age", PropertyType::INTEGER);
        let _height_prop = DbSchemaProperty::new("height", PropertyType::FLOAT);

        schema.add_node_property("Person", &name_prop).unwrap();
        schema.add_node_property("Person", &age_prop).unwrap();
        schema.add_node_property("Place", &name_prop).unwrap();

        // Add relationship types and properties
        let knows_rel = DbSchemaRelationshipPattern {
            start: "Person".to_string(),
            end: "Person".to_string(),
            rel_type: "KNOWS".to_string(),
        };
        let lives_in_rel = DbSchemaRelationshipPattern {
            start: "Person".to_string(),
            end: "Place".to_string(),
            rel_type: "LIVES_IN".to_string(),
        };

        schema.add_relationship(&knows_rel).unwrap();
        schema.add_relationship(&lives_in_rel).unwrap();

        let since_prop = DbSchemaProperty::new("since", PropertyType::STRING);
        schema
            .add_relationship_property("KNOWS", &since_prop)
            .unwrap();

        schema
    }

    #[test]
    fn test_extract_query_elements_basic() {
        let query = Query {
            match_clauses: vec![MatchClause {
                elements: vec![MatchElement {
                    path_var: Some("a".to_string()),
                    pattern: vec![PatternElement::Node(NodePattern {
                        variable: Some("a".to_string()),
                        label: Some("Person".to_string()),
                        properties: None,
                    })],
                }],
                is_optional: false,
            }],
            merge_clauses: vec![],
            create_clauses: vec![],
            with_clauses: vec![],
            where_clauses: vec![],
            return_clauses: vec![],
            unwind_clauses: vec![],
            call_clauses: vec![],
        };

        let elements = extract_query_elements(&query);

        assert!(elements.node_labels.contains("Person"));
        assert!(elements.defined_variables.contains("a"));
        assert_eq!(elements.property_accesses.len(), 0);
    }

    #[test]
    fn test_extract_query_elements_with_where() {
        let query = Query {
            match_clauses: vec![MatchClause {
                elements: vec![MatchElement {
                    path_var: Some("a".to_string()),
                    pattern: vec![PatternElement::Node(NodePattern {
                        variable: Some("a".to_string()),
                        label: Some("Person".to_string()),
                        properties: None,
                    })],
                }],
                is_optional: false,
            }],
            merge_clauses: vec![],
            create_clauses: vec![],
            with_clauses: vec![],
            where_clauses: vec![WhereClause {
                conditions: vec![WhereCondition::Comparison {
                    left: "a.age".to_string(),
                    operator: ">".to_string(),
                    right: "18".to_string(),
                }],
            }],
            return_clauses: vec![],
            unwind_clauses: vec![],
            call_clauses: vec![],
        };

        let elements = extract_query_elements(&query);

        assert!(elements.node_labels.contains("Person"));
        assert!(elements.referenced_variables.contains("a"));
        assert_eq!(elements.property_accesses.len(), 1);
        assert_eq!(elements.property_accesses[0].variable, "a");
        assert_eq!(elements.property_accesses[0].property, "age");
        assert!(matches!(
            elements.property_accesses[0].context,
            PropertyContext::Where
        ));
    }

    #[test]
    fn test_extract_query_elements_with_return() {
        let query = Query {
            match_clauses: vec![MatchClause {
                elements: vec![MatchElement {
                    path_var: Some("a".to_string()),
                    pattern: vec![PatternElement::Node(NodePattern {
                        variable: Some("a".to_string()),
                        label: Some("Person".to_string()),
                        properties: None,
                    })],
                }],
                is_optional: false,
            }],
            merge_clauses: vec![],
            create_clauses: vec![],
            with_clauses: vec![],
            where_clauses: vec![],
            return_clauses: vec![ReturnClause {
                items: vec!["a.name".to_string(), "a.age".to_string()],
            }],
            unwind_clauses: vec![],
            call_clauses: vec![],
        };

        let elements = extract_query_elements(&query);

        assert!(elements.node_labels.contains("Person"));
        assert!(elements.referenced_variables.contains("a"));
        assert_eq!(elements.property_accesses.len(), 2);

        let return_access: Vec<_> = elements
            .property_accesses
            .iter()
            .filter(|pa| matches!(pa.context, PropertyContext::Return))
            .collect();
        assert_eq!(return_access.len(), 2);
    }

    #[test]
    fn test_extract_query_elements_with_with() {
        let query = Query {
            match_clauses: vec![MatchClause {
                elements: vec![MatchElement {
                    path_var: Some("a".to_string()),
                    pattern: vec![PatternElement::Node(NodePattern {
                        variable: Some("a".to_string()),
                        label: Some("Person".to_string()),
                        properties: None,
                    })],
                }],
                is_optional: false,
            }],
            merge_clauses: vec![],
            create_clauses: vec![],
            with_clauses: vec![WithClause {
                items: vec![WithItem {
                    expression: WithExpression::PropertyAccess {
                        variable: "a".to_string(),
                        property: "name".to_string(),
                    },
                    alias: Some("person_name".to_string()),
                }],
            }],
            where_clauses: vec![],
            return_clauses: vec![],
            unwind_clauses: vec![],
            call_clauses: vec![],
        };

        let elements = extract_query_elements(&query);

        assert!(elements.node_labels.contains("Person"));
        assert!(elements.referenced_variables.contains("a"));
        assert_eq!(elements.property_accesses.len(), 1);
        assert_eq!(elements.property_accesses[0].variable, "a");
        assert_eq!(elements.property_accesses[0].property, "name");
        assert!(matches!(
            elements.property_accesses[0].context,
            PropertyContext::With
        ));
    }

    #[test]
    fn test_extract_query_elements_with_unwind() {
        let query = Query {
            match_clauses: vec![],
            merge_clauses: vec![],
            create_clauses: vec![],
            with_clauses: vec![],
            where_clauses: vec![],
            return_clauses: vec![],
            unwind_clauses: vec![UnwindClause {
                expression: UnwindExpression::List(vec![
                    PropertyValue::Number(1),
                    PropertyValue::Number(2),
                ]),
                variable: "x".to_string(),
            }],
            call_clauses: vec![],
        };
        let elements = extract_query_elements(&query);
        assert!(elements.defined_variables.contains("x"));
    }

    #[test]
    fn test_validate_query_elements_valid() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_node_label("Person".to_string());
        elements.add_defined_variable("a".to_string());
        elements.add_property_access(PropertyAccess {
            variable: "a".to_string(),
            property: "name".to_string(),
            context: PropertyContext::Return,
        });

        let errors = validate_query_elements(&elements, &schema);
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_validate_query_elements_invalid_node_label() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_node_label("InvalidLabel".to_string());

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidNodeLabel(_)
        ));
    }

    #[test]
    fn test_validate_query_elements_invalid_relationship_type() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_relationship_type("INVALID_REL".to_string());

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidRelationshipType(_)
        ));
    }

    #[test]
    fn test_validate_query_elements_invalid_node_property() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_node_property("Person".to_string(), "invalid_prop".to_string());

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidNodeProperty { .. }
        ));
    }

    #[test]
    fn test_validate_query_elements_invalid_relationship_property() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_relationship_property("KNOWS".to_string(), "invalid_prop".to_string());

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidRelationshipProperty { .. }
        ));
    }

    #[test]
    fn test_validate_query_elements_invalid_property_access() {
        let schema_json = r#"{
            "node_props": {
                "Person": [
                    {"name": "name", "neo4j_type": "STRING"},
                    {"name": "age", "neo4j_type": "INTEGER"}
                ]
            },
            "rel_props": {},
            "relationships": [],
            "metadata": {"index": [], "constraint": []}
        }"#;

        let schema = DbSchema::from_json_string(schema_json).unwrap();
        let _height_prop = DbSchemaProperty::new("height", PropertyType::FLOAT);

        let mut elements = QueryElements::new();
        elements.add_property_access(PropertyAccess {
            variable: "a".to_string(),
            property: "height".to_string(),
            context: PropertyContext::Return,
        });

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidPropertyAccess { .. }
        ));
    }

    #[test]
    fn test_validate_query_elements_multiple_errors() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_node_label("InvalidLabel".to_string());
        elements.add_relationship_type("INVALID_REL".to_string());
        elements.add_property_access(PropertyAccess {
            variable: "a".to_string(),
            property: "invalid_prop".to_string(),
            context: PropertyContext::Return,
        });

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 3);

        let error_types: Vec<_> = errors.iter().map(std::mem::discriminant).collect();
        assert!(error_types.contains(&std::mem::discriminant(
            &CypherGuardValidationError::InvalidNodeLabel("".to_string())
        )));
        assert!(error_types.contains(&std::mem::discriminant(
            &CypherGuardValidationError::InvalidRelationshipType("".to_string())
        )));
        assert!(error_types.contains(&std::mem::discriminant(
            &CypherGuardValidationError::InvalidPropertyAccess {
                variable: "".to_string(),
                property: "".to_string(),
                context: "".to_string()
            }
        )));
    }

    #[test]
    fn test_validate_query_elements_property_access_context() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        // Add valid property access in different contexts
        elements.add_property_access(PropertyAccess {
            variable: "a".to_string(),
            property: "name".to_string(),
            context: PropertyContext::Where,
        });
        elements.add_property_access(PropertyAccess {
            variable: "a".to_string(),
            property: "age".to_string(),
            context: PropertyContext::Return,
        });
        elements.add_property_access(PropertyAccess {
            variable: "r".to_string(),
            property: "since".to_string(),
            context: PropertyContext::With,
        });

        let errors = validate_query_elements(&elements, &schema);
        assert!(
            errors.is_empty(),
            "Expected no validation errors for valid property access, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_validate_query_elements_complex_where_condition() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        // Simulate complex WHERE condition extraction
        elements.add_property_access(PropertyAccess {
            variable: "a".to_string(),
            property: "age".to_string(),
            context: PropertyContext::Where,
        });
        elements.add_property_access(PropertyAccess {
            variable: "a".to_string(),
            property: "name".to_string(),
            context: PropertyContext::Where,
        });
        elements.add_property_access(PropertyAccess {
            variable: "r".to_string(),
            property: "since".to_string(),
            context: PropertyContext::Where,
        });

        let errors = validate_query_elements(&elements, &schema);
        assert!(
            errors.is_empty(),
            "Expected no validation errors for valid WHERE conditions, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_validate_query_elements_with_undefined_variables() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        // Add property access for undefined variables - this should still validate the property
        elements.add_property_access(PropertyAccess {
            variable: "undefined_var".to_string(),
            property: "name".to_string(),
            context: PropertyContext::Return,
        });

        let errors = validate_query_elements(&elements, &schema);
        // The property "name" exists in the schema, so no error should be generated
        assert_eq!(errors.len(), 0);

        // Now test with a property that doesn't exist
        elements.add_property_access(PropertyAccess {
            variable: "undefined_var".to_string(),
            property: "nonexistent_prop".to_string(),
            context: PropertyContext::Return,
        });

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidPropertyAccess { .. }
        ));
    }

    #[test]
    fn test_validate_unwind_expression_type() {
        let mut query = Query {
            match_clauses: vec![],
            merge_clauses: vec![],
            create_clauses: vec![],
            with_clauses: vec![],
            where_clauses: vec![],
            return_clauses: vec![],
            unwind_clauses: vec![UnwindClause {
                expression: UnwindExpression::Identifier("foo".to_string()),
                variable: "x".to_string(),
            }],
            call_clauses: vec![],
        };
        let elements = QueryElements::new();
        let schema = DbSchema::new();
        let errors = validate_query(&query, &elements, &schema);
        // All UNWIND expression types are now considered valid
        assert!(errors.is_empty());

        // Valid: list
        query.unwind_clauses = vec![UnwindClause {
            expression: UnwindExpression::List(vec![PropertyValue::Number(1)]),
            variable: "x".to_string(),
        }];
        let errors = validate_query(&query, &elements, &schema);
        assert!(errors.is_empty());

        // Valid: parameter
        query.unwind_clauses = vec![UnwindClause {
            expression: UnwindExpression::Parameter("foo".to_string()),
            variable: "x".to_string(),
        }];
        let errors = validate_query(&query, &elements, &schema);
        assert!(errors.is_empty());
    }
}
