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
}

impl QueryElements {
    pub fn new() -> Self {
        Self {
            node_labels: HashSet::new(),
            relationship_types: HashSet::new(),
            node_properties: HashMap::new(),
            relationship_properties: HashMap::new(),
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
}

/// Extract all elements from a parsed query that need validation
pub fn extract_query_elements(query: &Query) -> QueryElements {
    let mut elements = QueryElements::new();

    // Extract from MATCH clause
    if let Some(match_clause) = &query.match_clause {
        for element in &match_clause.elements {
            extract_from_match_element(element, &mut elements);
        }
    }

    // TODO: Extract from other clauses (WHERE, RETURN, etc.)

    elements
}

/// Extract elements from a single match element
fn extract_from_match_element(element: &MatchElement, elements: &mut QueryElements) {
    for pattern_element in &element.pattern {
        match pattern_element {
            PatternElement::Node(node) => {
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
                // Recursively extract from the inner pattern
                for pattern_element in &qpp.pattern {
                    match pattern_element {
                        PatternElement::Node(node) => {
                            if let Some(label) = &node.label {
                                elements.add_node_label(label.clone());
                            }
                        }
                        PatternElement::Relationship(rel) => {
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

/// Validate extracted query elements against the schema
pub fn validate_query_elements(
    elements: &QueryElements,
    schema: &DbSchema,
) -> Vec<CypherGuardValidationError> {
    let mut errors = Vec::new();

    // Validate node labels
    for label in &elements.node_labels {
        if !schema.has_label(label) {
            errors.push(CypherGuardValidationError::invalid_label(label.clone()));
        }
    }

    // Validate relationship types
    for rel_type in &elements.relationship_types {
        if !schema.has_relationship_type(rel_type) {
            errors.push(CypherGuardValidationError::invalid_relationship(
                rel_type.clone(),
            ));
        }
    }

    // Validate node properties
    for (label, properties) in &elements.node_properties {
        for property in properties {
            if !schema.has_node_property(label, property) {
                errors.push(CypherGuardValidationError::invalid_property_name(
                    property.clone(),
                ));
            }
        }
    }

    // Validate relationship properties
    for (rel_type, properties) in &elements.relationship_properties {
        for property in properties {
            if !schema.has_relationship_property(rel_type, property) {
                errors.push(CypherGuardValidationError::invalid_property_name(
                    property.clone(),
                ));
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_query;
    use crate::schema::{DbSchema, DbSchemaProperty, DbSchemaRelationshipPattern, PropertyType};

    fn create_test_schema() -> DbSchema {
        let mut schema = DbSchema::new();

        // Add node labels
        schema.add_label("Person").unwrap();
        schema.add_label("Movie").unwrap();

        // Add node properties
        schema
            .add_node_property(
                "Person",
                &DbSchemaProperty::new("name", PropertyType::STRING),
            )
            .unwrap();
        schema
            .add_node_property(
                "Person",
                &DbSchemaProperty::new("age", PropertyType::INTEGER),
            )
            .unwrap();
        schema
            .add_node_property(
                "Movie",
                &DbSchemaProperty::new("title", PropertyType::STRING),
            )
            .unwrap();

        // Add relationship types
        schema
            .add_relationship(&DbSchemaRelationshipPattern {
                start: "Person".to_string(),
                end: "Person".to_string(),
                rel_type: "KNOWS".to_string(),
            })
            .unwrap();
        schema
            .add_relationship(&DbSchemaRelationshipPattern {
                start: "Person".to_string(),
                end: "Movie".to_string(),
                rel_type: "ACTED_IN".to_string(),
            })
            .unwrap();

        // Add relationship properties
        schema
            .add_relationship_property(
                "KNOWS",
                &DbSchemaProperty::new("since", PropertyType::STRING),
            )
            .unwrap();
        schema
            .add_relationship_property(
                "ACTED_IN",
                &DbSchemaProperty::new("role", PropertyType::STRING),
            )
            .unwrap();

        schema
    }

    #[test]
    fn test_extract_simple_node() {
        let query = Query {
            match_clause: Some(MatchClause {
                elements: vec![MatchElement {
                    path_var: None,
                    pattern: vec![PatternElement::Node(NodePattern {
                        variable: Some("p".to_string()),
                        label: Some("Person".to_string()),
                        properties: None,
                    })],
                }],
                is_optional: false,
            }),
            merge_clause: None,
            create_clause: None,
            with_clause: None,
            where_clause: None,
            return_clause: None,
        };

        let elements = extract_query_elements(&query);

        assert!(elements.node_labels.contains("Person"));
        assert_eq!(elements.node_labels.len(), 1);
        assert!(elements.relationship_types.is_empty());
    }

    #[test]
    fn test_extract_node_with_properties() {
        let query = Query {
            match_clause: Some(MatchClause {
                elements: vec![MatchElement {
                    path_var: None,
                    pattern: vec![PatternElement::Node(NodePattern {
                        variable: Some("p".to_string()),
                        label: Some("Person".to_string()),
                        properties: Some(vec![
                            Property {
                                key: "name".to_string(),
                                value: PropertyValue::String("Alice".to_string()),
                            },
                            Property {
                                key: "age".to_string(),
                                value: PropertyValue::Number(30),
                            },
                        ]),
                    })],
                }],
                is_optional: false,
            }),
            merge_clause: None,
            create_clause: None,
            with_clause: None,
            where_clause: None,
            return_clause: None,
        };

        let elements = extract_query_elements(&query);

        assert!(elements.node_labels.contains("Person"));
        assert!(elements
            .node_properties
            .get("Person")
            .unwrap()
            .contains("name"));
        assert!(elements
            .node_properties
            .get("Person")
            .unwrap()
            .contains("age"));
    }

    #[test]
    fn test_extract_relationship() {
        use crate::parser::ast::{Direction, RelationshipDetails};

        let query = Query {
            match_clause: Some(MatchClause {
                elements: vec![MatchElement {
                    path_var: None,
                    pattern: vec![
                        PatternElement::Node(NodePattern {
                            variable: Some("a".to_string()),
                            label: Some("Person".to_string()),
                            properties: None,
                        }),
                        PatternElement::Relationship(RelationshipPattern::Regular(
                            RelationshipDetails {
                                variable: Some("r".to_string()),
                                direction: Direction::Right,
                                properties: None,
                                rel_type: Some("KNOWS".to_string()),
                                length: None,
                                where_clause: None,
                                quantifier: None,
                                is_optional: false,
                            },
                        )),
                        PatternElement::Node(NodePattern {
                            variable: Some("b".to_string()),
                            label: Some("Person".to_string()),
                            properties: None,
                        }),
                    ],
                }],
                is_optional: false,
            }),
            merge_clause: None,
            create_clause: None,
            with_clause: None,
            where_clause: None,
            return_clause: None,
        };

        let elements = extract_query_elements(&query);

        assert!(elements.node_labels.contains("Person"));
        assert!(elements.relationship_types.contains("KNOWS"));
        assert_eq!(elements.node_labels.len(), 1); // Should deduplicate
        assert_eq!(elements.relationship_types.len(), 1);
    }

    #[test]
    fn test_validate_valid_elements() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_node_label("Person".to_string());
        elements.add_relationship_type("KNOWS".to_string());
        elements.add_node_property("Person".to_string(), "name".to_string());
        elements.add_relationship_property("KNOWS".to_string(), "since".to_string());

        let errors = validate_query_elements(&elements, &schema);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_invalid_node_label() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_node_label("InvalidLabel".to_string());

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidLabel(_)
        ));
    }

    #[test]
    fn test_validate_invalid_relationship_type() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_relationship_type("INVALID_REL".to_string());

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidRelationship(_)
        ));
    }

    #[test]
    fn test_validate_invalid_node_property() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_node_label("Person".to_string());
        elements.add_node_property("Person".to_string(), "invalid_property".to_string());

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidPropertyName(_)
        ));
    }

    #[test]
    fn test_validate_invalid_relationship_property() {
        let schema = create_test_schema();
        let mut elements = QueryElements::new();

        elements.add_relationship_type("KNOWS".to_string());
        elements.add_relationship_property("KNOWS".to_string(), "invalid_property".to_string());

        let errors = validate_query_elements(&elements, &schema);
        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidPropertyName(_)
        ));
    }

    #[test]
    fn test_integration_with_parser() {
        let schema = create_test_schema();

        // Test a valid query
        let query_str = "MATCH (p:Person)-[r:KNOWS]->(q:Person) RETURN p.name";
        let query = parse_query(query_str).expect("Should parse successfully");

        let elements = extract_query_elements(&query);
        let errors = validate_query_elements(&elements, &schema);

        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors
        );
        assert!(elements.node_labels.contains("Person"));
        assert!(elements.relationship_types.contains("KNOWS"));
    }

    #[test]
    fn test_integration_invalid_label() {
        let schema = create_test_schema();

        // Test a query with invalid label
        let query_str = "MATCH (p:InvalidLabel) RETURN p";
        let query = parse_query(query_str).expect("Should parse successfully");

        let elements = extract_query_elements(&query);
        let errors = validate_query_elements(&elements, &schema);

        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidLabel(_)
        ));
    }

    #[test]
    fn test_integration_invalid_relationship() {
        let schema = create_test_schema();

        // Test a query with invalid relationship type
        let query_str = "MATCH (p:Person)-[r:INVALID_REL]->(q:Person) RETURN p";
        let query = parse_query(query_str).expect("Should parse successfully");

        let elements = extract_query_elements(&query);
        let errors = validate_query_elements(&elements, &schema);

        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidRelationship(_)
        ));
    }

    #[test]
    fn test_integration_node_with_properties() {
        let schema = create_test_schema();

        // Test a query with node properties
        let query_str = "MATCH (p:Person {name: 'Alice', age: 30}) RETURN p";
        let query = parse_query(query_str).expect("Should parse successfully");

        let elements = extract_query_elements(&query);
        let errors = validate_query_elements(&elements, &schema);

        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors
        );
        assert!(elements
            .node_properties
            .get("Person")
            .unwrap()
            .contains("name"));
        assert!(elements
            .node_properties
            .get("Person")
            .unwrap()
            .contains("age"));
    }

    #[test]
    fn test_integration_invalid_node_property() {
        let schema = create_test_schema();

        // Test a query with invalid node property
        let query_str = "MATCH (p:Person {invalid_prop: 'value'}) RETURN p";
        let query = parse_query(query_str).expect("Should parse successfully");

        let elements = extract_query_elements(&query);
        let errors = validate_query_elements(&elements, &schema);

        assert_eq!(errors.len(), 1);
        assert!(matches!(
            errors[0],
            CypherGuardValidationError::InvalidPropertyName(_)
        ));
    }

    #[test]
    fn test_error_integration_with_cypher_guard_error() {
        let schema = create_test_schema();

        // Test a query with multiple validation errors
        let query_str =
            "MATCH (p:InvalidLabel)-[r:INVALID_REL]->(q:Person {invalid_prop: 'value'}) RETURN p";
        let query = parse_query(query_str).expect("Should parse successfully");

        let elements = extract_query_elements(&query);
        let validation_errors = validate_query_elements(&elements, &schema);

        // Should have 3 errors: invalid label, invalid relationship, invalid property
        assert_eq!(validation_errors.len(), 3);

        // Check that we can convert these to CypherGuardError
        let cypher_errors: Vec<crate::errors::CypherGuardError> =
            validation_errors.into_iter().map(|e| e.into()).collect();

        assert_eq!(cypher_errors.len(), 3);

        // Verify all are validation errors
        for error in &cypher_errors {
            assert!(error.is_validation());
        }

        // Test error messages
        let error_messages: Vec<String> = cypher_errors.iter().map(|e| e.to_string()).collect();

        // Should contain validation error messages
        assert!(error_messages
            .iter()
            .any(|msg| msg.contains("Invalid label")));
        assert!(error_messages
            .iter()
            .any(|msg| msg.contains("Invalid relationship")));
        assert!(error_messages
            .iter()
            .any(|msg| msg.contains("Invalid property name")));
    }
}
