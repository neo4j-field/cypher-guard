mod parser;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
/// Enum representing possible property types in the schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum PropertyType {
    /// String property
    String,
    /// Integer property
    Integer,
    /// Float property
    Float,
    /// Boolean property
    Boolean,
    /// DateTime property
    DateTime,
    /// Custom enum type (referenced by name)
    Enum(String),
}

/// Structure representing a user-defined enum type for properties.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumType {
    /// Name of the enum type
    pub name: String,
    /// Allowed values for the enum
    pub values: Vec<String>,
}

/// Main schema struct for the graph database.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DbSchema {
    /// Node labels
    pub labels: Vec<String>,
    /// Relationship types
    pub rel_types: Vec<String>,
    /// Properties (name -> type)
    pub properties: HashMap<String, PropertyType>,
    /// Custom enums (name -> EnumType)
    pub enums: HashMap<String, EnumType>,
}

impl DbSchema {
    /// Create a new, empty schema
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            rel_types: Vec::new(),
            properties: HashMap::new(),
            enums: HashMap::new(),
        }
    }

    /// Add a label to the schema
    pub fn add_label(&mut self, label: String) -> Result<()> {
        if self.labels.contains(&label) {
            return Err(CypherGuardError::SchemaError);
        }
        self.labels.push(label);
        Ok(())
    }

    /// Remove a label from the schema
    pub fn remove_label(&mut self, label: &str) -> Result<()> {
        if let Some(pos) = self.labels.iter().position(|l| l == label) {
            self.labels.remove(pos);
            Ok(())
        } else {
            Err(CypherGuardError::SchemaError)
        }
    }

    /// Add a relationship type to the schema
    pub fn add_relationship_type(&mut self, rel_type: String) -> Result<()> {
        if self.rel_types.contains(&rel_type) {
            return Err(CypherGuardError::SchemaError);
        }
        self.rel_types.push(rel_type);
        Ok(())
    }

    /// Remove a relationship type from the schema
    pub fn remove_relationship_type(&mut self, rel_type: &str) -> Result<()> {
        if let Some(pos) = self.rel_types.iter().position(|r| r == rel_type) {
            self.rel_types.remove(pos);
            Ok(())
        } else {
            Err(CypherGuardError::SchemaError)
        }
    }

    /// Add a property to the schema
    pub fn add_property(&mut self, name: String, prop_type: PropertyType) -> Result<()> {
        if self.properties.contains_key(&name) {
            return Err(CypherGuardError::SchemaError);
        }
        self.properties.insert(name, prop_type);
        Ok(())
    }

    /// Remove a property from the schema
    pub fn remove_property(&mut self, name: &str) -> Result<()> {
        if self.properties.remove(name).is_some() {
            Ok(())
        } else {
            Err(CypherGuardError::SchemaError)
        }
    }

    /// Add a custom enum to the schema
    pub fn add_enum(&mut self, name: String, enum_type: EnumType) -> Result<()> {
        if self.enums.contains_key(&name) {
            return Err(CypherGuardError::SchemaError);
        }
        self.enums.insert(name, enum_type);
        Ok(())
    }

    /// Remove a custom enum from the schema
    pub fn remove_enum(&mut self, name: &str) -> Result<()> {
        if self.enums.remove(name).is_some() {
            Ok(())
        } else {
            Err(CypherGuardError::SchemaError)
        }
    }

    /// Check if a label exists
    pub fn has_label(&self, label: &str) -> bool {
        self.labels.contains(&label.to_string())
    }

    /// Check if a relationship type exists
    pub fn has_relationship_type(&self, rel_type: &str) -> bool {
        self.rel_types.contains(&rel_type.to_string())
    }

    /// Check if a property exists
    pub fn has_property(&self, name: &str) -> bool {
        self.properties.contains_key(name)
    }

    /// Check if a custom enum exists
    pub fn has_enum(&self, name: &str) -> bool {
        self.enums.contains_key(name)
    }

    /// Validate the schema for consistency and integrity.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        // Check for duplicate names across labels, rel_types, enums
        let mut all_names = self.labels.clone();
        all_names.extend(self.rel_types.iter().cloned());
        all_names.extend(self.enums.keys().cloned());
        all_names.sort();
        for w in all_names.windows(2) {
            if w[0] == w[1] {
                errors.push(format!("Duplicate name found across schema elements: {}", w[0]));
            }
        }

        // Check property names for snake_case (simple check)
        for name in self.properties.keys() {
            if !name.chars().all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit()) {
                errors.push(format!("Property name '{}' should be snake_case (lowercase, digits, underscores)", name));
            }
        }

        // Check that enum properties reference valid enums
        for (name, prop_type) in &self.properties {
            if let PropertyType::Enum(enum_name) = prop_type {
                if !self.enums.contains_key(enum_name) {
                    errors.push(format!("Property '{}' references undefined enum type '{}'.", name, enum_name));
                }
            }
        }

        errors
    }

    /// Load a DbSchema from a JSON string
    pub fn from_json_str(s: &str) -> Result<Self> {
        serde_json::from_str(s).map_err(|e| CypherGuardError::SchemaError)
    }

    /// Serialize a DbSchema to a JSON string
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|_| CypherGuardError::SchemaError)
    }
}

impl fmt::Display for DbSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Labels: {:?}", self.labels)?;
        writeln!(f, "Relationship Types: {:?}", self.rel_types)?;
        writeln!(f, "Properties:")?;
        for (name, typ) in &self.properties {
            writeln!(f, "  {}: {:?}", name, typ)?;
        }
        writeln!(f, "Enums:")?;
        for (name, enum_type) in &self.enums {
            writeln!(f, "  {}: {:?}", name, enum_type)?;
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CypherGuardError {
    #[error("Invalid query")] 
    InvalidQuery,
    #[error("Schema error")] 
    SchemaError,
}

pub type Result<T> = std::result::Result<T, CypherGuardError>;

/// Placeholder validation function
pub fn validate_cypher(query: &str) -> Result<bool> {
    // TODO: Implement validation logic
    Ok(true)
}
pub fn validate_cypher_with_schema(query: &str, schema: &DbSchema) -> Result<bool> {
    match parser::query(query) {
        Ok((_, ast)) => {
            let errors = get_cypher_validation_errors(query, schema);
            Ok(errors.is_empty())
        },
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
                                            if !schema.has_property(&prop.key) {
                                                errors.push(format!("Property '{}' not in schema", prop.key));
                                            }
                                        }
                                    }
                                }
                                parser::PatternElement::Relationship(rel) => {
                                    if let Some(rel_type) = &rel.rel_type {
                                        if !schema.has_relationship_type(rel_type) {
                                            errors.push(format!("Relationship type '{}' not in schema", rel_type));
                                        }
                                    }
                                    if let Some(props) = &rel.properties {
                                        for prop in props {
                                            if !schema.has_property(&prop.key) {
                                                errors.push(format!("Property '{}' not in schema", prop.key));
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
                                            if !schema.has_property(&prop.key) {
                                                errors.push(format!("Property '{}' not in schema", prop.key));
                                            }
                                        }
                                    }
                                }
                                parser::PatternElement::Relationship(rel) => {
                                    if let Some(rel_type) = &rel.rel_type {
                                        if !schema.has_relationship_type(rel_type) {
                                            errors.push(format!("Relationship type '{}' not in schema", rel_type));
                                        }
                                    }
                                    if let Some(props) = &rel.properties {
                                        for prop in props {
                                            if !schema.has_property(&prop.key) {
                                                errors.push(format!("Property '{}' not in schema", prop.key));
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



#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_property_type_serialization() {
        let types = vec![
            PropertyType::String,
            PropertyType::Integer,
            PropertyType::Float,
            PropertyType::Boolean,
            PropertyType::DateTime,
            PropertyType::Enum("ColorEnum".to_string()),
        ];
        for t in types {
            let serialized = serde_json::to_string(&t).unwrap();
            let deserialized: PropertyType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(t, deserialized);
        }
    }

    #[test]
    fn test_enum_type_serialization() {
        let enum_type = EnumType {
            name: "ColorEnum".to_string(),
            values: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        };
        let serialized = serde_json::to_string(&enum_type).unwrap();
        let deserialized: EnumType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(enum_type, deserialized);
    }

    #[test]
    fn test_db_schema_creation_and_serialization() {
        let mut schema = DbSchema::new();
        schema.labels.push("Person".to_string());
        schema.rel_types.push("KNOWS".to_string());
        schema.properties.insert("age".to_string(), PropertyType::Integer);
        let color_enum = EnumType {
            name: "ColorEnum".to_string(),
            values: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        };
        schema.enums.insert("ColorEnum".to_string(), color_enum.clone());
        let serialized = serde_json::to_string(&schema).unwrap();
        let deserialized: DbSchema = serde_json::from_str(&serialized).unwrap();
        assert_eq!(schema, deserialized);
        assert_eq!(deserialized.labels, vec!["Person"]);
        assert_eq!(deserialized.rel_types, vec!["KNOWS"]);
        assert_eq!(deserialized.properties["age"], PropertyType::Integer);
        assert_eq!(deserialized.enums["ColorEnum"], color_enum);
    }

    #[test]
    fn test_add_and_remove_label() {
        let mut schema = DbSchema::new();
        assert!(schema.add_label("Person".to_string()).is_ok());
        assert!(schema.has_label("Person"));
        // Duplicate label
        assert!(schema.add_label("Person".to_string()).is_err());
        // Remove label
        assert!(schema.remove_label("Person").is_ok());
        assert!(!schema.has_label("Person"));
        // Remove non-existent label
        assert!(schema.remove_label("Person").is_err());
    }

    #[test]
    fn test_add_and_remove_relationship_type() {
        let mut schema = DbSchema::new();
        assert!(schema.add_relationship_type("KNOWS".to_string()).is_ok());
        assert!(schema.has_relationship_type("KNOWS"));
        // Duplicate
        assert!(schema.add_relationship_type("KNOWS".to_string()).is_err());
        // Remove
        assert!(schema.remove_relationship_type("KNOWS").is_ok());
        assert!(!schema.has_relationship_type("KNOWS"));
        // Remove non-existent
        assert!(schema.remove_relationship_type("KNOWS").is_err());
    }

    #[test]
    fn test_add_and_remove_property() {
        let mut schema = DbSchema::new();
        assert!(schema.add_property("age".to_string(), PropertyType::Integer).is_ok());
        assert!(schema.has_property("age"));
        // Duplicate
        assert!(schema.add_property("age".to_string(), PropertyType::Integer).is_err());
        // Remove
        assert!(schema.remove_property("age").is_ok());
        assert!(!schema.has_property("age"));
        // Remove non-existent
        assert!(schema.remove_property("age").is_err());
    }

    #[test]
    fn test_add_and_remove_enum() {
        let mut schema = DbSchema::new();
        let enum_type = EnumType {
            name: "ColorEnum".to_string(),
            values: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        };
        assert!(schema.add_enum("ColorEnum".to_string(), enum_type.clone()).is_ok());
        assert!(schema.has_enum("ColorEnum"));
        // Duplicate
        assert!(schema.add_enum("ColorEnum".to_string(), enum_type.clone()).is_err());
        // Remove
        assert!(schema.remove_enum("ColorEnum").is_ok());
        assert!(!schema.has_enum("ColorEnum"));
        // Remove non-existent
        assert!(schema.remove_enum("ColorEnum").is_err());
    }

    #[test]
    fn test_schema_validation_no_errors() {
        let mut schema = DbSchema::new();
        schema.add_label("person".to_string()).unwrap();
        schema.add_relationship_type("knows".to_string()).unwrap();
        schema.add_property("age".to_string(), PropertyType::Integer).unwrap();
        let color_enum = EnumType {
            name: "color_enum".to_string(),
            values: vec!["red".to_string(), "green".to_string(), "blue".to_string()],
        };
        schema.add_enum("color_enum".to_string(), color_enum).unwrap();
        schema.add_property("favorite_color".to_string(), PropertyType::Enum("color_enum".to_string())).unwrap();
        let errors = schema.validate();
        assert!(errors.is_empty(), "Expected no validation errors, got: {:?}", errors);
    }

    #[test]
    fn test_schema_validation_duplicate_names() {
        let mut schema = DbSchema::new();
        schema.add_label("person".to_string()).unwrap();
        schema.add_relationship_type("person".to_string()).unwrap(); // duplicate name
        let errors = schema.validate();
        assert!(errors.iter().any(|e| e.contains("Duplicate name")), "Expected duplicate name error");
    }

    #[test]
    fn test_schema_validation_bad_property_name() {
        let mut schema = DbSchema::new();
        schema.add_property("BadName".to_string(), PropertyType::String).unwrap();
        let errors = schema.validate();
        assert!(errors.iter().any(|e| e.contains("snake_case")), "Expected snake_case property name error");
    }

    #[test]
    fn test_schema_validation_missing_enum_reference() {
        let mut schema = DbSchema::new();
        schema.add_property("favorite_color".to_string(), PropertyType::Enum("missing_enum".to_string())).unwrap();
        let errors = schema.validate();
        assert!(errors.iter().any(|e| e.contains("undefined enum type")), "Expected undefined enum type error");
    }

    #[test]
    fn test_from_json_str_valid() {
        let json = r#"{
            "labels": ["person"],
            "rel_types": ["knows"],
            "properties": {"age": {"type": "Integer"}},
            "enums": {}
        }"#;
        let schema = DbSchema::from_json_str(json);
        assert!(schema.is_ok());
        let schema = schema.unwrap();
        assert!(schema.has_label("person"));
        assert!(schema.has_relationship_type("knows"));
        assert!(schema.has_property("age"));
    }

    #[test]
    fn test_from_json_str_invalid() {
        let json = "not valid json";
        let schema = DbSchema::from_json_str(json);
        assert!(schema.is_err());
    }

    #[test]
    fn test_to_json_string() {
        let mut schema = DbSchema::new();
        schema.add_label("person".to_string()).unwrap();
        let json = schema.to_json_string();
        assert!(json.is_ok());
        let json = json.unwrap();
        assert!(json.contains("person"));
    }
}
