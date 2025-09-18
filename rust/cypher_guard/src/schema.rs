use crate::errors::{CypherGuardError, CypherGuardSchemaError};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Enumeration of supported property types in Neo4j
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum PropertyType {
    STRING,
    INTEGER,
    FLOAT,
    BOOLEAN,
    POINT,
    DATE_TIME,
    LIST,
}

impl fmt::Display for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropertyType::STRING => write!(f, "STRING"),
            PropertyType::INTEGER => write!(f, "INTEGER"),
            PropertyType::FLOAT => write!(f, "FLOAT"),
            PropertyType::BOOLEAN => write!(f, "BOOLEAN"),
            PropertyType::POINT => write!(f, "POINT"),
            PropertyType::DATE_TIME => write!(f, "DATE_TIME"),
            PropertyType::LIST => write!(f, "LIST"),
        }
    }
}

impl PropertyType {
    pub fn from_string(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "STRING" | "STR" => Ok(PropertyType::STRING),
            "INTEGER" | "INT" => Ok(PropertyType::INTEGER),
            "FLOAT" => Ok(PropertyType::FLOAT),
            "BOOLEAN" | "BOOL" => Ok(PropertyType::BOOLEAN),
            "POINT" => Ok(PropertyType::POINT),
            "DATE_TIME" => Ok(PropertyType::DATE_TIME),
            "LIST" => Ok(PropertyType::LIST),
            _ => Err(CypherGuardError::Schema(
                CypherGuardSchemaError::InvalidPropertyType(format!(
                    "Invalid property type: {}",
                    s
                )),
            )),
        }
    }
}

/// Structure representing a property in the schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DbSchemaProperty {
    /// Name of the property
    pub name: String,
    /// Type of the property (Neo4j type)
    pub neo4j_type: PropertyType,
    /// Possible values for enum properties
    pub enum_values: Option<Vec<String>>,
    /// Minimum value for numeric properties
    pub min_value: Option<f64>,
    /// Maximum value for numeric properties
    pub max_value: Option<f64>,
    /// Number of distinct values in the database
    pub distinct_value_count: Option<i64>,
    /// Example values from the database
    pub example_values: Option<Vec<String>>,
}

impl Default for DbSchemaProperty {
    fn default() -> Self {
        Self {
            name: "prop".to_string(),
            neo4j_type: PropertyType::STRING,
            enum_values: None,
            min_value: None,
            max_value: None,
            distinct_value_count: None,
            example_values: None,
        }
    }
}

impl DbSchemaProperty {
    pub fn new(name: &str, neo4j_type: PropertyType) -> Self {
        Self {
            name: name.to_string(),
            neo4j_type,
            ..Default::default()
        }
    }

    pub fn with_enum_values(
        name: &str,
        neo4j_type: PropertyType,
        enum_values: Vec<String>,
    ) -> Self {
        Self {
            name: name.to_string(),
            neo4j_type,
            enum_values: Some(enum_values),
            ..Default::default()
        }
    }

    pub fn set_enum_values(&mut self, values: Vec<String>) {
        self.enum_values = Some(values);
    }

    pub fn set_min_value(&mut self, value: f64) {
        self.min_value = Some(value);
    }

    pub fn set_max_value(&mut self, value: f64) {
        self.max_value = Some(value);
    }

    pub fn set_distinct_value_count(&mut self, count: i64) {
        self.distinct_value_count = Some(count);
    }

    pub fn set_example_values(&mut self, values: Vec<String>) {
        self.example_values = Some(values);
    }

    pub fn get_enum_values(&self) -> Option<&Vec<String>> {
        self.enum_values.as_ref()
    }

    pub fn get_min_value(&self) -> Option<f64> {
        self.min_value
    }

    pub fn get_max_value(&self) -> Option<f64> {
        self.max_value
    }

    pub fn get_distinct_value_count(&self) -> Option<i64> {
        self.distinct_value_count
    }

    pub fn get_example_values(&self) -> Option<&Vec<String>> {
        self.example_values.as_ref()
    }
}

/// Structure representing a relationship pattern in the schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DbSchemaRelationshipPattern {
    /// Start node label
    pub start: String,
    /// End node label  
    pub end: String,
    /// Relationship type
    pub rel_type: String,
}

impl DbSchemaRelationshipPattern {
    pub fn new(start: &str, end: &str, rel_type: &str) -> Self {
        Self {
            start: start.to_string(),
            end: end.to_string(),
            rel_type: rel_type.to_string(),
        }
    }
}

/// Structure representing a constraint in the schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DbSchemaConstraint {
    /// Constraint ID
    pub id: i64,
    /// Constraint name
    pub name: String,
    /// Type of constraint (e.g., "UNIQUE", "EXISTS")
    pub constraint_type: String,
    /// Entity type (node or relationship)
    pub entity_type: String,
    /// Labels or relationship types affected
    pub labels: Vec<String>,
    /// Properties affected by the constraint
    pub properties: Vec<String>,
}

impl DbSchemaConstraint {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i64,
        name: String,
        constraint_type: String,
        entity_type: String,
        labels: Vec<String>,
        properties: Vec<String>,
    ) -> Self {
        Self {
            id,
            name,
            constraint_type,
            entity_type,
            labels,
            properties,
        }
    }
}

/// Structure representing an index in the schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DbSchemaIndex {
    /// Index label (node type)
    pub label: String,
    /// Properties included in the index
    pub properties: Vec<String>,
    /// Size of the index
    pub size: i64,
    /// Type of index (e.g., "BTREE", "TEXT")
    pub index_type: String,
}

impl DbSchemaIndex {
    pub fn new(label: String, properties: Vec<String>, size: i64, index_type: String) -> Self {
        Self {
            label,
            properties,
            size,
            index_type,
        }
    }
}

/// Structure containing metadata about constraints and indexes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DbSchemaMetadata {
    /// List of constraints in the database
    pub constraint: Vec<DbSchemaConstraint>,
    /// List of indexes in the database
    pub index: Vec<DbSchemaIndex>,
}

impl Default for DbSchemaMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl DbSchemaMetadata {
    pub fn new() -> Self {
        Self {
            constraint: Vec::new(),
            index: Vec::new(),
        }
    }
}

/// Main schema structure representing the complete database schema.
/// Follows the Neo4j GraphRAG library standard format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DbSchema {
    /// Node properties by node label (Neo4j GraphRAG standard format)
    pub node_props: HashMap<String, Vec<DbSchemaProperty>>,
    /// Relationship properties by relationship type
    pub rel_props: HashMap<String, Vec<DbSchemaProperty>>,
    /// Valid relationship patterns
    pub relationships: Vec<DbSchemaRelationshipPattern>,
    /// Schema metadata (constraints and indexes)
    pub metadata: DbSchemaMetadata,
}

impl Default for DbSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl DbSchema {
    /// Create a new, empty schema
    pub fn new() -> Self {
        Self {
            node_props: HashMap::new(),
            rel_props: HashMap::new(),
            relationships: Vec::new(),
            metadata: DbSchemaMetadata::new(),
        }
    }

    /// Load schema from JSON string
    pub fn from_json_string(json_str: &str) -> Result<Self> {
        match serde_json::from_str(json_str) {
            Ok(schema) => Ok(schema),
            Err(e) => Err(CypherGuardError::Schema(
                CypherGuardSchemaError::InvalidJson(format!("Failed to parse schema JSON: {}", e)),
            )),
        }
    }

    /// Convert schema to JSON string
    pub fn to_json_string(&self) -> Result<String> {
        match serde_json::to_string_pretty(self) {
            Ok(json) => Ok(json),
            Err(e) => Err(CypherGuardError::Schema(
                CypherGuardSchemaError::SerializationError(format!(
                    "Failed to serialize schema: {}",
                    e
                )),
            )),
        }
    }

    /// Add a new node label to the schema
    pub fn add_label(&mut self, label: &str) -> Result<()> {
        if self.has_label(label) {
            return Err(CypherGuardError::Schema(
                CypherGuardSchemaError::DuplicateLabel(format!("Label '{}' already exists", label)),
            ));
        }
        self.node_props.insert(label.to_string(), Vec::new());
        Ok(())
    }

    /// Remove a node label from the schema
    pub fn remove_label(&mut self, label: &str) -> Result<()> {
        if self.node_props.remove(label).is_none() {
            return Err(CypherGuardError::Schema(
                CypherGuardSchemaError::LabelNotFound(format!("Label '{}' not found", label)),
            ));
        }
        Ok(())
    }

    /// Add a property to a node label
    pub fn add_node_property(&mut self, label: &str, property: &DbSchemaProperty) -> Result<()> {
        if let Some(properties) = self.node_props.get_mut(label) {
            // Check for duplicate property
            if properties.iter().any(|p| p.name == property.name) {
                return Err(CypherGuardError::Schema(
                    CypherGuardSchemaError::DuplicateProperty(format!(
                        "Property '{}' already exists for label '{}'",
                        property.name, label
                    )),
                ));
            }
            properties.push(property.clone());
            Ok(())
        } else {
            Err(CypherGuardError::Schema(
                CypherGuardSchemaError::LabelNotFound(format!("Label '{}' not found", label)),
            ))
        }
    }

    /// Remove a property from a node label
    pub fn remove_node_property(&mut self, label: &str, property_name: &str) -> Result<()> {
        if let Some(properties) = self.node_props.get_mut(label) {
            let initial_len = properties.len();
            properties.retain(|p| p.name != property_name);
            if properties.len() == initial_len {
                return Err(CypherGuardError::Schema(
                    CypherGuardSchemaError::PropertyNotFound(format!(
                        "Property '{}' not found for label '{}'",
                        property_name, label
                    )),
                ));
            }
            Ok(())
        } else {
            Err(CypherGuardError::Schema(
                CypherGuardSchemaError::LabelNotFound(format!("Label '{}' not found", label)),
            ))
        }
    }

    /// Check if a label exists in the schema
    pub fn has_label(&self, label: &str) -> bool {
        self.node_props.contains_key(label)
    }

    /// Check if a specific property exists for a node label
    pub fn has_node_property(&self, label: &str, property_name: &str) -> bool {
        self.node_props
            .get(label)
            .is_some_and(|properties| properties.iter().any(|p| p.name == property_name))
    }

    /// Get all properties for a specific node label
    pub fn get_node_properties(&self, label: &str) -> Option<&Vec<DbSchemaProperty>> {
        self.node_props.get(label)
    }

    /// Get a specific property for a node label
    pub fn get_node_property(&self, label: &str, property_name: &str) -> Option<&DbSchemaProperty> {
        self.node_props
            .get(label)
            .and_then(|properties| properties.iter().find(|p| p.name == property_name))
    }

    /// Check if a property exists in any node
    pub fn has_property_in_nodes(&self, property_name: &str) -> bool {
        self.node_props
            .values()
            .any(|properties| properties.iter().any(|p| p.name == property_name))
    }

    /// Check if a relationship type exists
    pub fn has_relationship_type(&self, rel_type: &str) -> bool {
        self.rel_props.contains_key(rel_type)
            || self.relationships.iter().any(|r| r.rel_type == rel_type)
    }

    /// Check if a specific relationship property exists
    pub fn has_relationship_property(&self, rel_type: &str, property_name: &str) -> bool {
        self.rel_props
            .get(rel_type)
            .is_some_and(|props| props.iter().any(|p| p.name == property_name))
    }

    /// Add a relationship property
    pub fn add_relationship_property(
        &mut self,
        rel_type: &str,
        property: &DbSchemaProperty,
    ) -> Result<()> {
        let properties = self.rel_props.entry(rel_type.to_string()).or_default();

        // Check for duplicates
        if properties.iter().any(|p| p.name == property.name) {
            return Err(CypherGuardError::Schema(
                CypherGuardSchemaError::DuplicateProperty(format!(
                    "Property '{}' already exists for relationship '{}'",
                    property.name, rel_type
                )),
            ));
        }

        properties.push(property.clone());
        Ok(())
    }

    /// Remove a relationship property
    pub fn remove_relationship_property(
        &mut self,
        rel_type: &str,
        property_name: &str,
    ) -> Result<()> {
        if let Some(properties) = self.rel_props.get_mut(rel_type) {
            let initial_len = properties.len();
            properties.retain(|p| p.name != property_name);

            if properties.len() == initial_len {
                return Err(CypherGuardError::Schema(
                    CypherGuardSchemaError::PropertyNotFound(format!(
                        "Property '{}' not found for relationship '{}'",
                        property_name, rel_type
                    )),
                ));
            }

            // Remove empty relationship type
            if properties.is_empty() {
                self.rel_props.remove(rel_type);
            }

            Ok(())
        } else {
            Err(CypherGuardError::Schema(
                CypherGuardSchemaError::relationship_not_found(format!(
                    "Relationship type '{}' not found",
                    rel_type
                )),
            ))
        }
    }

    /// Add a relationship pattern
    pub fn add_relationship_pattern(&mut self, pattern: DbSchemaRelationshipPattern) -> Result<()> {
        // Check for duplicates
        if self.relationships.iter().any(|p| {
            p.start == pattern.start && p.end == pattern.end && p.rel_type == pattern.rel_type
        }) {
            return Err(CypherGuardError::Schema(
                CypherGuardSchemaError::duplicate_relationship(format!(
                    "Relationship pattern '({})--[{}]--->({})' already exists",
                    pattern.start, pattern.rel_type, pattern.end
                )),
            ));
        }

        self.relationships.push(pattern);
        Ok(())
    }

    /// Validate the schema for consistency
    pub fn validate(&self) -> Result<()> {
        // Check that all relationship patterns reference existing node labels
        for pattern in &self.relationships {
            if !self.has_label(&pattern.start) {
                return Err(CypherGuardError::Schema(
                    CypherGuardSchemaError::LabelNotFound(format!(
                        "Start label '{}' in relationship pattern not found",
                        pattern.start
                    )),
                ));
            }
            if !self.has_label(&pattern.end) {
                return Err(CypherGuardError::Schema(
                    CypherGuardSchemaError::LabelNotFound(format!(
                        "End label '{}' in relationship pattern not found",
                        pattern.end
                    )),
                ));
            }
        }

        // Additional validation can be added here
        Ok(())
    }
}

impl fmt::Display for DbSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Labels: {:?}",
            self.node_props.keys().collect::<Vec<_>>()
        )?;
        writeln!(f, "Properties:")?;
        for (label, properties) in &self.node_props {
            writeln!(f, "  {}: {:?}", label, properties)?;
        }
        writeln!(f, "Relationship Properties: {:?}", self.rel_props)?;
        writeln!(f, "Relationships: {:?}", self.relationships)?;
        writeln!(f, "Constraints: {:?}", self.metadata.constraint)?;
        writeln!(f, "Indexes: {:?}", self.metadata.index)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{CypherGuardError, CypherGuardSchemaError};

    fn create_person_name_property() -> DbSchemaProperty {
        DbSchemaProperty::new("name", PropertyType::STRING)
    }

    fn create_person_age_property() -> DbSchemaProperty {
        DbSchemaProperty::new("age", PropertyType::INTEGER)
    }

    fn create_place_name_property() -> DbSchemaProperty {
        DbSchemaProperty::new("name", PropertyType::STRING)
    }

    fn create_knows_since_property() -> DbSchemaProperty {
        DbSchemaProperty::new("since", PropertyType::DATE_TIME)
    }

    fn create_lives_in_rel() -> DbSchemaRelationshipPattern {
        DbSchemaRelationshipPattern::new("Person", "Place", "LIVES_IN")
    }

    fn create_knows_rel() -> DbSchemaRelationshipPattern {
        DbSchemaRelationshipPattern::new("Person", "Person", "KNOWS")
    }

    fn create_test_schema() -> DbSchema {
        let mut schema = DbSchema::new();

        // Add labels
        schema.add_label("Person").unwrap();
        schema.add_label("Place").unwrap();

        // Add properties
        let person_name_property = create_person_name_property();
        let person_age_property = create_person_age_property();
        let place_name_property = create_place_name_property();
        let knows_since_property = create_knows_since_property();

        let lives_in_rel = create_lives_in_rel();
        let knows_rel = create_knows_rel();

        schema
            .add_node_property("Person", &person_name_property)
            .unwrap();
        schema
            .add_node_property("Person", &person_age_property)
            .unwrap();
        schema
            .add_node_property("Place", &place_name_property)
            .unwrap();

        schema
            .add_relationship_property("KNOWS", &knows_since_property)
            .unwrap();

        schema.add_relationship_pattern(lives_in_rel).unwrap();
        schema.add_relationship_pattern(knows_rel).unwrap();

        schema
    }

    #[test]
    fn test_schema_creation() {
        let schema = create_test_schema();
        assert_eq!(schema.node_props.len(), 2);
        assert!(schema.has_label("Person"));
        assert!(schema.has_label("Place"));
        assert!(!schema.has_label("NonExistent"));
    }

    #[test]
    fn test_add_and_remove_labels() {
        let mut schema = DbSchema::new();

        // Add label
        assert!(schema.add_label("Person").is_ok());
        assert!(schema.has_label("Person"));

        // Duplicate label should fail
        assert!(schema.add_label("Person").is_err());

        // Remove label
        assert!(schema.remove_label("Person").is_ok());
        assert!(!schema.has_label("Person"));

        // Remove non-existent label should fail
        assert!(schema.remove_label("Person").is_err());
    }

    #[test]
    fn test_add_and_remove_node_property() {
        let mut schema = DbSchema::new();
        schema.add_label("Person").unwrap();

        // Add property
        assert!(schema
            .add_node_property("Person", &create_person_name_property(),)
            .is_ok());
        assert!(schema.has_node_property("Person", "name"));

        // Duplicate property should fail
        assert!(schema
            .add_node_property("Person", &create_person_name_property())
            .is_err());

        // Remove property
        assert!(schema.remove_node_property("Person", "name").is_ok());
        assert!(!schema.has_node_property("Person", "name"));

        // Remove non-existent property should fail
        assert!(schema.remove_node_property("Person", "name").is_err());
    }

    #[test]
    fn test_add_and_remove_relationship_property() {
        let mut schema = DbSchema::new();
        let knows_since_property = create_knows_since_property();

        // Add relationship property
        assert!(schema
            .add_relationship_property("KNOWS", &knows_since_property)
            .is_ok());
        assert!(schema.rel_props.contains_key("KNOWS"));
        assert_eq!(schema.rel_props["KNOWS"].len(), 1);

        // Duplicate should fail
        assert!(schema
            .add_relationship_property("KNOWS", &knows_since_property)
            .is_err());

        // Remove property
        assert!(schema
            .remove_relationship_property("KNOWS", "since")
            .is_ok());
        assert!(!schema.rel_props.contains_key("KNOWS"));

        // Remove non-existent should fail
        assert!(schema
            .remove_relationship_property("KNOWS", "since")
            .is_err());
    }

    #[test]
    fn test_json_serialization() {
        let schema = create_test_schema();
        let json_str = schema.to_json_string().unwrap();
        let deserialized_schema = DbSchema::from_json_string(&json_str).unwrap();
        assert_eq!(schema, deserialized_schema);
    }

    #[test]
    fn test_schema_validation() {
        let mut schema = DbSchema::new();
        schema.add_label("Person").unwrap();
        schema.add_label("Place").unwrap();

        // Add valid relationship pattern
        let valid_pattern = DbSchemaRelationshipPattern::new("Person", "Place", "LIVES_IN");
        schema.add_relationship_pattern(valid_pattern).unwrap();

        // Schema should be valid
        assert!(schema.validate().is_ok());

        // Add invalid relationship pattern (non-existent label)
        let invalid_pattern = DbSchemaRelationshipPattern::new("Person", "NonExistent", "WORKS_AT");
        schema.add_relationship_pattern(invalid_pattern).unwrap();

        // Schema should now be invalid
        assert!(schema.validate().is_err());
    }

    #[test]
    fn test_property_type_from_string() {
        assert_eq!(
            PropertyType::from_string("STRING").unwrap(),
            PropertyType::STRING
        );
        assert_eq!(
            PropertyType::from_string("str").unwrap(),
            PropertyType::STRING
        );
        assert_eq!(
            PropertyType::from_string("INTEGER").unwrap(),
            PropertyType::INTEGER
        );
        assert_eq!(
            PropertyType::from_string("int").unwrap(),
            PropertyType::INTEGER
        );
        assert_eq!(
            PropertyType::from_string("FLOAT").unwrap(),
            PropertyType::FLOAT
        );
        assert_eq!(
            PropertyType::from_string("BOOLEAN").unwrap(),
            PropertyType::BOOLEAN
        );
        assert_eq!(
            PropertyType::from_string("bool").unwrap(),
            PropertyType::BOOLEAN
        );
        assert_eq!(
            PropertyType::from_string("POINT").unwrap(),
            PropertyType::POINT
        );
        assert_eq!(
            PropertyType::from_string("DATE_TIME").unwrap(),
            PropertyType::DATE_TIME
        );
        assert_eq!(
            PropertyType::from_string("LIST").unwrap(),
            PropertyType::LIST
        );

        // Invalid type should fail
        assert!(PropertyType::from_string("INVALID").is_err());
    }

    #[test]
    fn test_duplicate_property_error() {
        let mut schema = DbSchema::new();
        schema.add_label("Person").unwrap();
        let prop = create_person_name_property();
        schema.add_node_property("Person", &prop).unwrap();
        let result = schema.add_node_property("Person", &prop);
        assert!(matches!(
            result,
            Err(CypherGuardError::Schema(
                CypherGuardSchemaError::DuplicateProperty(_)
            ))
        ));
    }

    #[test]
    fn test_schema_display() {
        let schema = create_test_schema();
        let display_string = format!("{}", schema);
        assert!(display_string.contains("Person"));
        assert!(display_string.contains("Place"));
        assert!(display_string.contains("LIVES_IN"));
        assert!(display_string.contains("KNOWS"));
    }
}
