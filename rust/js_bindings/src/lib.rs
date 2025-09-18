use cypher_guard::{
    get_cypher_validation_errors, parse_query as parse_query_rust, validate_cypher_with_schema,
    CypherGuardError, CypherGuardParsingError, CypherGuardSchemaError, CypherGuardValidationError,
    DbSchema as CoreDbSchema, DbSchemaConstraint as CoreDbSchemaConstraint,
    DbSchemaIndex as CoreDbSchemaIndex, DbSchemaMetadata as CoreDbSchemaMetadata,
    DbSchemaProperty as CoreDbSchemaProperty,
    DbSchemaRelationshipPattern as CoreDbSchemaRelationshipPattern,
    PropertyType as CorePropertyType,
};
use napi::{Env, JsObject, Result as NapiResult};
use napi_derive::napi;
use std::collections::HashMap;

// === Error Conversion Helpers ===
fn convert_cypher_error(err: CypherGuardError) -> napi::Error {
    match err {
        CypherGuardError::Parsing(e) => convert_parsing_error(e),
        CypherGuardError::Validation(e) => convert_validation_error(e),
        CypherGuardError::Schema(e) => convert_schema_error(e),
        CypherGuardError::InvalidQuery(msg) => napi::Error::from_reason(msg),
    }
}

fn convert_parsing_error(err: CypherGuardParsingError) -> napi::Error {
    napi::Error::from_reason(err.to_string())
}

fn convert_validation_error(err: CypherGuardValidationError) -> napi::Error {
    let msg = match err {
        CypherGuardValidationError::InvalidNodeLabel(label) => {
            format!("Invalid node label: {}", label)
        }
        CypherGuardValidationError::InvalidRelationshipType(rel_type) => {
            format!("Invalid relationship type: {}", rel_type)
        }
        CypherGuardValidationError::InvalidNodeProperty { label, property } => {
            format!("Invalid node property '{}' on label '{}'", property, label)
        }
        CypherGuardValidationError::InvalidRelationshipProperty { rel_type, property } => {
            format!(
                "Invalid relationship property '{}' on type '{}'",
                property, rel_type
            )
        }
        CypherGuardValidationError::InvalidPropertyAccess {
            variable,
            property,
            context,
        } => format!(
            "Invalid property access '{}.{}' in {} clause",
            variable, property, context
        ),
        CypherGuardValidationError::InvalidPropertyName(name) => {
            format!("Invalid property name: {}", name)
        }
        CypherGuardValidationError::TypeMismatch { expected, actual } => {
            format!("Type mismatch: expected {}, got {}", expected, actual)
        }
        CypherGuardValidationError::InvalidRelationship(rel) => {
            format!("Invalid relationship: {}", rel)
        }
        CypherGuardValidationError::InvalidLabel(label) => {
            format!("Invalid label: {}", label)
        }
        CypherGuardValidationError::InvalidPropertyType {
            variable,
            property,
            expected_type,
            actual_value,
        } => format!(
            "Invalid property type for '{}.{}': expected {}, got value '{}'",
            variable, property, expected_type, actual_value
        ),
        CypherGuardValidationError::UndefinedVariable(var) => {
            format!("Undefined variable: {}", var)
        }
    };
    napi::Error::from_reason(msg)
}

fn convert_schema_error(err: CypherGuardSchemaError) -> napi::Error {
    napi::Error::from_reason(err.to_string())
}

/// Validate a Cypher query against a schema.
///
/// @param query - The Cypher query string to validate
/// @param schema_json - JSON schema string representing the Neo4j database schema
/// @returns True if the query is valid according to the schema, False otherwise
/// @throws Error if validation fails due to schema violations or parsing errors
///
/// @example
/// ```javascript
/// const schemaJson = JSON.stringify({
///   "node_props": {"Person": [{"name": "name", "neo4j_type": "STRING"}]},
///   "rel_props": {},
///   "relationships": [],
///   "metadata": {"index": [], "constraint": []}
/// });
/// 
/// const isValid = validateCypher("MATCH (p:Person) RETURN p.name", schemaJson);
/// console.log(isValid); // true
/// ```
#[napi]
pub fn validate_cypher(query: String, schema_json: String) -> napi::Result<bool> {
    let schema = CoreDbSchema::from_json_string(&schema_json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;
    validate_cypher_with_schema(&query, &schema).map_err(convert_cypher_error)
}

/// Get all validation errors for a Cypher query against a schema.
///
/// @param query - The Cypher query string to validate
/// @param schema_json - JSON schema string representing the Neo4j database schema
/// @returns Array of validation error messages. Empty array if query is valid.
///
/// @example
/// ```javascript
/// const schemaJson = JSON.stringify({
///   "node_props": {"Person": [{"name": "name", "neo4j_type": "STRING"}]},
///   "rel_props": {},
///   "relationships": [],
///   "metadata": {"index": [], "constraint": []}
/// });
/// 
/// const errors = getValidationErrors("MATCH (p:InvalidLabel) RETURN p.name", schemaJson);
/// console.log(errors); // ['Invalid node label: InvalidLabel']
/// 
/// const validErrors = getValidationErrors("MATCH (p:Person) RETURN p.name", schemaJson);
/// console.log(validErrors); // []
/// ```
#[napi]
pub fn get_validation_errors(query: String, schema_json: String) -> napi::Result<Vec<String>> {
    let schema = CoreDbSchema::from_json_string(&schema_json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;
    Ok(get_cypher_validation_errors(&query, &schema))
}

/// Parse a Cypher query into an Abstract Syntax Tree (AST).
///
/// @param query - The Cypher query string to parse
/// @returns The parsed AST as a JavaScript object (currently returns empty object)
/// @throws Error if the query has syntax errors and cannot be parsed
#[napi]
pub fn parse_query(env: Env, query: String) -> NapiResult<JsObject> {
    match parse_query_rust(&query) {
        Ok(_ast) => env.create_object(), // Return an empty JS object for now
        Err(e) => Err(convert_parsing_error(e)),
    }
}

/// Fast validation check - returns true if query is valid, false if it has any errors.
/// Optimized for LLM validation loops where you only need to know if the query is valid.
///
/// @param query - The Cypher query string to validate
/// @param schema_json - JSON schema string representing the Neo4j database schema
/// @returns True if query is completely valid, false if it has any validation or parsing errors
///
/// @example
/// ```javascript
/// const schemaJson = JSON.stringify({
///   "node_props": {"Person": [{"name": "name", "neo4j_type": "STRING"}]},
///   "rel_props": {},
///   "relationships": [],
///   "metadata": {"index": [], "constraint": []}
/// });
/// 
/// console.log(hasValidCypher("MATCH (p:Person) RETURN p.name", schemaJson)); // true
/// console.log(hasValidCypher("MATCH (p:InvalidLabel) RETURN p.name", schemaJson)); // false
/// ```
#[napi]
pub fn has_valid_cypher(query: String, schema_json: String) -> napi::Result<bool> {
    let schema = CoreDbSchema::from_json_string(&schema_json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;
    // Fast path - just check if there are any validation errors
    let errors = get_cypher_validation_errors(&query, &schema);
    Ok(errors.is_empty())
}

/// JavaScript wrapper for PropertyType enum
#[napi(object)]
pub struct PropertyType {
    pub type_name: String,
}

/// JavaScript wrapper for DbSchemaProperty
#[napi(object)]
pub struct DbSchemaProperty {
    pub name: String,
    pub neo4j_type: String,
    pub enum_values: Option<Vec<String>>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub distinct_value_count: Option<i64>,
    pub example_values: Option<Vec<String>>,
}

/// JavaScript wrapper for DbSchemaRelationshipPattern
#[napi(object)]
pub struct DbSchemaRelationshipPattern {
    pub start: String,
    pub end: String,
    pub rel_type: String,
}

/// JavaScript wrapper for DbSchemaConstraint
#[napi(object)]
pub struct DbSchemaConstraint {
    pub id: i64,
    pub name: String,
    pub constraint_type: String,
    pub entity_type: String,
    pub labels: Vec<String>,
    pub properties: Vec<String>,
}

/// JavaScript wrapper for DbSchemaIndex
#[napi(object)]
pub struct DbSchemaIndex {
    pub label: String,
    pub properties: Vec<String>,
    pub size: i64,
    pub index_type: String,
}

/// JavaScript wrapper for DbSchemaMetadata
#[napi(object)]
pub struct DbSchemaMetadata {
    pub constraint: Vec<DbSchemaConstraint>,
    pub index: Vec<DbSchemaIndex>,
}

/// JavaScript wrapper for DbSchema
#[napi(object)]
pub struct DbSchema {
    pub node_props: HashMap<String, Vec<DbSchemaProperty>>,
    pub rel_props: HashMap<String, Vec<DbSchemaProperty>>,
    pub relationships: Vec<DbSchemaRelationshipPattern>,
    pub metadata: DbSchemaMetadata,
}

// === Implementation methods for type conversions ===

impl PropertyType {
    fn from_core(core_type: &CorePropertyType) -> Self {
        Self {
            type_name: core_type.to_string(),
        }
    }

    fn to_core(&self) -> Result<CorePropertyType, napi::Error> {
        match self.type_name.to_uppercase().as_str() {
            "STRING" => Ok(CorePropertyType::STRING),
            "INTEGER" => Ok(CorePropertyType::INTEGER),
            "FLOAT" => Ok(CorePropertyType::FLOAT),
            "BOOLEAN" => Ok(CorePropertyType::BOOLEAN),
            "POINT" => Ok(CorePropertyType::POINT),
            "DATE_TIME" => Ok(CorePropertyType::DATE_TIME),
            "LIST" => Ok(CorePropertyType::LIST),
            _ => Err(napi::Error::from_reason(format!(
                "Invalid property type: {}",
                self.type_name
            ))),
        }
    }
}

impl DbSchemaProperty {
    fn from_core(core_prop: &CoreDbSchemaProperty) -> Self {
        Self {
            name: core_prop.name.clone(),
            neo4j_type: core_prop.neo4j_type.to_string(),
            enum_values: core_prop.enum_values.clone(),
            min_value: core_prop.min_value,
            max_value: core_prop.max_value,
            distinct_value_count: core_prop.distinct_value_count,
            example_values: core_prop.example_values.clone(),
        }
    }

    fn to_core(&self) -> Result<CoreDbSchemaProperty, napi::Error> {
        let property_type = match self.neo4j_type.to_uppercase().as_str() {
            "STRING" => CorePropertyType::STRING,
            "INTEGER" => CorePropertyType::INTEGER,
            "FLOAT" => CorePropertyType::FLOAT,
            "BOOLEAN" => CorePropertyType::BOOLEAN,
            "POINT" => CorePropertyType::POINT,
            "DATE_TIME" => CorePropertyType::DATE_TIME,
            "LIST" => CorePropertyType::LIST,
            _ => {
                return Err(napi::Error::from_reason(format!(
                    "Invalid property type: {}",
                    self.neo4j_type
                )))
            }
        };

        Ok(CoreDbSchemaProperty {
            name: self.name.clone(),
            neo4j_type: property_type,
            enum_values: self.enum_values.clone(),
            min_value: self.min_value,
            max_value: self.max_value,
            distinct_value_count: self.distinct_value_count,
            example_values: self.example_values.clone(),
        })
    }
}

impl DbSchemaRelationshipPattern {
    fn from_core(core_rel: &CoreDbSchemaRelationshipPattern) -> Self {
        Self {
            start: core_rel.start.clone(),
            end: core_rel.end.clone(),
            rel_type: core_rel.rel_type.clone(),
        }
    }

    fn to_core(&self) -> CoreDbSchemaRelationshipPattern {
        CoreDbSchemaRelationshipPattern {
            start: self.start.clone(),
            end: self.end.clone(),
            rel_type: self.rel_type.clone(),
        }
    }
}

impl DbSchemaConstraint {
    fn from_core(core_constraint: &CoreDbSchemaConstraint) -> Self {
        Self {
            id: core_constraint.id,
            name: core_constraint.name.clone(),
            constraint_type: core_constraint.constraint_type.clone(),
            entity_type: core_constraint.entity_type.clone(),
            labels: core_constraint.labels.clone(),
            properties: core_constraint.properties.clone(),
        }
    }

    fn to_core(&self) -> CoreDbSchemaConstraint {
        CoreDbSchemaConstraint {
            id: self.id,
            name: self.name.clone(),
            constraint_type: self.constraint_type.clone(),
            entity_type: self.entity_type.clone(),
            labels: self.labels.clone(),
            properties: self.properties.clone(),
        }
    }
}

impl DbSchemaIndex {
    fn from_core(core_index: &CoreDbSchemaIndex) -> Self {
        Self {
            label: core_index.label.clone(),
            properties: core_index.properties.clone(),
            size: core_index.size,
            index_type: core_index.index_type.clone(),
        }
    }

    fn to_core(&self) -> CoreDbSchemaIndex {
        CoreDbSchemaIndex {
            label: self.label.clone(),
            properties: self.properties.clone(),
            size: self.size,
            index_type: self.index_type.clone(),
        }
    }
}

impl DbSchemaMetadata {
    fn from_core(core_metadata: &CoreDbSchemaMetadata) -> Self {
        let constraints = core_metadata
            .constraint
            .iter()
            .map(DbSchemaConstraint::from_core)
            .collect();

        let indexes = core_metadata
            .index
            .iter()
            .map(DbSchemaIndex::from_core)
            .collect();

        Self {
            constraint: constraints,
            index: indexes,
        }
    }

    fn to_core(&self) -> CoreDbSchemaMetadata {
        let core_constraints = self
            .constraint
            .iter()
            .map(|c| c.to_core())
            .collect();

        let core_indexes = self
            .index
            .iter()
            .map(|i| i.to_core())
            .collect();

        CoreDbSchemaMetadata {
            constraint: core_constraints,
            index: core_indexes,
        }
    }

    fn new() -> Self {
        Self {
            constraint: Vec::new(),
            index: Vec::new(),
        }
    }
}

impl DbSchema {
    fn from_core(core_schema: &CoreDbSchema) -> Self {
        let node_props = core_schema
            .node_props
            .iter()
            .map(|(label, core_properties)| {
                let properties = core_properties
                    .iter()
                    .map(DbSchemaProperty::from_core)
                    .collect();
                (label.clone(), properties)
            })
            .collect();

        let rel_props = core_schema
            .rel_props
            .iter()
            .map(|(rel_type, core_properties)| {
                let properties = core_properties
                    .iter()
                    .map(DbSchemaProperty::from_core)
                    .collect();
                (rel_type.clone(), properties)
            })
            .collect();

        let relationships = core_schema
            .relationships
            .iter()
            .map(DbSchemaRelationshipPattern::from_core)
            .collect();

        let metadata = DbSchemaMetadata::from_core(&core_schema.metadata);

        Self {
            node_props,
            rel_props,
            relationships,
            metadata,
        }
    }

    fn to_core(&self) -> Result<CoreDbSchema, napi::Error> {
        let mut core_schema = CoreDbSchema::new();

        // Add node properties
        for (label, properties) in &self.node_props {
            core_schema
                .add_label(label)
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;

            for prop in properties {
                let core_prop = prop.to_core()?;
                core_schema
                    .add_node_property(label, &core_prop)
                    .map_err(|e| napi::Error::from_reason(e.to_string()))?;
            }
        }

        // Add relationship properties
        for (rel_type, properties) in &self.rel_props {
            for prop in properties {
                let core_prop = prop.to_core()?;
                core_schema
                    .add_relationship_property(rel_type, &core_prop)
                    .map_err(|e| napi::Error::from_reason(e.to_string()))?;
            }
        }

        // Add relationships
        for rel in &self.relationships {
            let core_rel = rel.to_core();
            core_schema
                .add_relationship_pattern(core_rel)
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        }

        Ok(core_schema)
    }
}

// === JavaScript API Functions ===

/// Create a new DbSchema from a JSON string
///
/// @param json_str - JSON string representing the schema
/// @returns DbSchema object
#[napi]
pub fn db_schema_from_json_string(json_str: String) -> napi::Result<DbSchema> {
    let core_schema = CoreDbSchema::from_json_string(&json_str)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;
    Ok(DbSchema::from_core(&core_schema))
}

/// Create a new empty DbSchema
///
/// @returns Empty DbSchema object
#[napi]
pub fn db_schema_new() -> DbSchema {
    let core_schema = CoreDbSchema::new();
    DbSchema::from_core(&core_schema)
}

/// Create a new empty DbSchemaMetadata
///
/// @returns Empty DbSchemaMetadata object
#[napi]
pub fn db_schema_metadata_new() -> DbSchemaMetadata {
    DbSchemaMetadata::new()
}

/// Create a new DbSchemaConstraint
///
/// @param id - Constraint ID
/// @param name - Constraint name
/// @param constraint_type - Type of constraint (e.g., "UNIQUE", "EXISTS")
/// @param entity_type - Entity type (node or relationship)
/// @param labels - Labels or relationship types affected
/// @param properties - Properties affected by the constraint
/// @returns DbSchemaConstraint object
#[napi]
pub fn db_schema_constraint_new(
    id: i64,
    name: String,
    constraint_type: String,
    entity_type: String,
    labels: Vec<String>,
    properties: Vec<String>,
) -> DbSchemaConstraint {
    DbSchemaConstraint {
        id,
        name,
        constraint_type,
        entity_type,
        labels,
        properties,
    }
}

/// Create a new DbSchemaIndex
///
/// @param label - Index label (node type)
/// @param properties - Properties included in the index
/// @param size - Size of the index
/// @param index_type - Type of index (e.g., "BTREE", "TEXT")
/// @returns DbSchemaIndex object
#[napi]
pub fn db_schema_index_new(
    label: String,
    properties: Vec<String>,
    size: i64,
    index_type: String,
) -> DbSchemaIndex {
    DbSchemaIndex {
        label,
        properties,
        size,
        index_type,
    }
}

/// Check if a schema has a specific label
///
/// @param schema - The DbSchema object
/// @param label - Label to check for
/// @returns True if the label exists in the schema
#[napi]
pub fn db_schema_has_label(schema: DbSchema, label: String) -> napi::Result<bool> {
    let core_schema = schema.to_core()?;
    Ok(core_schema.has_label(&label))
}

/// Check if a schema has a specific node property
///
/// @param schema - The DbSchema object
/// @param label - Node label
/// @param property - Property name to check for
/// @returns True if the property exists on the label
#[napi]
pub fn db_schema_has_node_property(
    schema: DbSchema,
    label: String,
    property: String,
) -> napi::Result<bool> {
    let core_schema = schema.to_core()?;
    Ok(core_schema.has_node_property(&label, &property))
}

/// Create a new DbSchemaProperty
///
/// @param name - Property name
/// @param neo4j_type - Property type (STRING, INTEGER, etc.)
/// @returns DbSchemaProperty object
#[napi]
pub fn db_schema_property_new(name: String, neo4j_type: String) -> napi::Result<DbSchemaProperty> {
    let property_type = match neo4j_type.to_uppercase().as_str() {
        "STRING" => CorePropertyType::STRING,
        "INTEGER" => CorePropertyType::INTEGER,
        "FLOAT" => CorePropertyType::FLOAT,
        "BOOLEAN" => CorePropertyType::BOOLEAN,
        "POINT" => CorePropertyType::POINT,
        "DATE_TIME" => CorePropertyType::DATE_TIME,
        "LIST" => CorePropertyType::LIST,
        _ => {
            return Err(napi::Error::from_reason(format!(
                "Invalid property type: {}",
                neo4j_type
            )))
        }
    };

    let core_prop = CoreDbSchemaProperty {
        name: name.clone(),
        neo4j_type: property_type,
        enum_values: None,
        min_value: None,
        max_value: None,
        distinct_value_count: None,
        example_values: None,
    };

    Ok(DbSchemaProperty::from_core(&core_prop))
}

/// Create a new DbSchemaRelationshipPattern
///
/// @param start - Start node label
/// @param end - End node label
/// @param rel_type - Relationship type
/// @returns DbSchemaRelationshipPattern object
#[napi]
pub fn db_schema_relationship_pattern_new(
    start: String,
    end: String,
    rel_type: String,
) -> DbSchemaRelationshipPattern {
    DbSchemaRelationshipPattern {
        start,
        end,
        rel_type,
    }
}

// === Structured Error Types ===

#[napi(object)]
pub struct StructuredErrorCategories {
    pub schema_errors: Vec<String>,
    pub property_errors: Vec<String>,
    pub syntax_errors: Vec<String>,
    pub type_errors: Vec<String>,
    pub parsing_errors: Vec<String>,
}

#[napi(object)]
pub struct StructuredErrors {
    pub has_errors: bool,
    pub error_count: u32,
    pub categories: StructuredErrorCategories,
    pub query: String,
    pub suggestions: Vec<String>,
}

/// Internal function that separates parsing errors from validation errors properly
fn get_structured_cypher_errors(
    query: &str,
    schema: &CoreDbSchema,
) -> (Vec<String>, Vec<CypherGuardParsingError>) {
    // First, check for parsing errors
    let parsing_errors = match parse_query_rust(query) {
        Ok(_) => Vec::new(), // No parsing errors, proceed to validation
        Err(parsing_error) => vec![parsing_error], // Parsing failed, return parsing error
    };

    // Only run validation if parsing succeeded
    let validation_error_strings = if parsing_errors.is_empty() {
        get_cypher_validation_errors(query, schema)
    } else {
        Vec::new() // Skip validation if parsing failed
    };

    (validation_error_strings, parsing_errors)
}

/// Get structured validation errors optimized for LLM feedback.
/// Returns categorized error information to help LLMs generate better corrections.
///
/// @param query - The Cypher query string to validate
/// @param schema_json - JSON schema string representing the Neo4j database schema
/// @returns Structured error information with categories and suggestions
///
/// @example
/// ```javascript
/// const schemaJson = JSON.stringify({
///   "node_props": {"Person": [{"name": "name", "neo4j_type": "STRING"}]},
///   "rel_props": {},
///   "relationships": [],
///   "metadata": {"index": [], "constraint": []}
/// });
/// 
/// const structuredErrors = getStructuredErrors("MATCH (p:InvalidLabel) RETURN p.invalid", schemaJson);
/// console.log(structuredErrors);
/// // {
/// //   has_errors: true,
/// //   error_count: 2,
/// //   categories: {
/// //     schema_errors: ['Invalid node label: InvalidLabel'],
/// //     property_errors: ['Invalid property: p.invalid'],
/// //     syntax_errors: [],
/// //     type_errors: [],
/// //     parsing_errors: []
/// //   },
/// //   query: 'MATCH (p:InvalidLabel) RETURN p.invalid',
/// //   suggestions: ['Check available node labels in schema', 'Verify property names']
/// // }
/// ```
#[napi]
pub fn get_structured_errors(
    query: String,
    schema_json: String,
) -> napi::Result<StructuredErrors> {
    let schema = CoreDbSchema::from_json_string(&schema_json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;

    // Get structured errors (validation as strings, parsing as structured types)
    let (validation_error_strings, parsing_errors) =
        get_structured_cypher_errors(&query, &schema);

    // Categorize errors using enhanced string matching with better patterns
    let mut schema_errors = Vec::new();
    let mut property_errors = Vec::new();
    let mut syntax_errors = Vec::new();
    let mut type_errors = Vec::new();
    let mut parsing_error_list = Vec::new();
    let mut suggestions = Vec::new();

    // Handle validation errors with improved categorization
    for error_str in &validation_error_strings {
        // Skip generic parsing errors if we have specific parsing errors to process
        if error_str == "Invalid Cypher syntax" && !parsing_errors.is_empty() {
            continue; // Skip generic messages when we have specific parsing errors
        }

        if error_str.contains("Invalid node label") {
            schema_errors.push(error_str.clone());
            suggestions.push("Check available node labels in your schema".to_string());
        } else if error_str.contains("Invalid relationship type") {
            schema_errors.push(error_str.clone());
            suggestions.push("Check available relationship types in your schema".to_string());
        } else if error_str.contains("Invalid label") {
            schema_errors.push(error_str.clone());
            suggestions.push("Verify the label exists in your schema definition".to_string());
        } else if error_str.contains("Invalid relationship") {
            schema_errors.push(error_str.clone());
            suggestions.push("Verify the relationship type exists in your schema".to_string());
        } else if error_str.contains("Invalid property access")
            || error_str.contains("property access")
        {
            property_errors.push(error_str.clone());
            suggestions.push(
                "Ensure the variable is defined and the property exists on the bound type"
                    .to_string(),
            );
        } else if error_str.contains("Invalid node property") {
            property_errors.push(error_str.clone());
            suggestions
                .push("Check that the property exists on the specified node label".to_string());
        } else if error_str.contains("Invalid relationship property") {
            property_errors.push(error_str.clone());
            suggestions.push(
                "Check that the property exists on the specified relationship type".to_string(),
            );
        } else if error_str.contains("Invalid property name") {
            property_errors.push(error_str.clone());
            suggestions
                .push("Verify the property name follows Neo4j naming conventions".to_string());
        } else if error_str.contains("Invalid property type") || error_str.contains("property type")
        {
            type_errors.push(error_str.clone());
            suggestions.push("Ensure the value type matches the property's expected type (STRING, INTEGER, etc.)".to_string());
        } else if error_str.contains("Type mismatch") || error_str.contains("expected") {
            type_errors.push(error_str.clone());
            suggestions.push(
                "Check that the data types are compatible in the comparison or assignment"
                    .to_string(),
            );
        } else if !parsing_errors.is_empty() {
            // If we have specific parsing errors, don't add generic suggestions for unknown validation errors
            syntax_errors.push(error_str.clone());
        } else {
            // Only add generic suggestions when there are no specific parsing errors
            syntax_errors.push(error_str.clone());
            suggestions.push("Review query structure and validation requirements".to_string());
        }
    }

    // Handle parsing errors with precise type matching for much better LLM guidance
    for error in &parsing_errors {
        let error_str = error.to_string();
        parsing_error_list.push(error_str.clone());

        match error {
            // Specific clause order errors (17 variants) with targeted suggestions
            CypherGuardParsingError::ReturnBeforeOtherClauses { .. } => {
                syntax_errors.push(error_str);
                suggestions.push("Move RETURN clause to the end of the query, after MATCH, WHERE, and WITH clauses".to_string());
            }
            CypherGuardParsingError::MatchAfterReturn { .. } => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "MATCH clauses must come before RETURN - reorganize your query structure"
                        .to_string(),
                );
            }
            CypherGuardParsingError::CreateAfterReturn { .. } => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "CREATE clauses must come before RETURN - move CREATE earlier in the query"
                        .to_string(),
                );
            }
            CypherGuardParsingError::MergeAfterReturn { .. } => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "MERGE clauses must come before RETURN - move MERGE earlier in the query"
                        .to_string(),
                );
            }
            CypherGuardParsingError::DeleteAfterReturn { .. } => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "DELETE clauses must come before RETURN - move DELETE earlier in the query"
                        .to_string(),
                );
            }
            CypherGuardParsingError::SetAfterReturn { .. } => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "SET clauses must come before RETURN - move SET earlier in the query"
                        .to_string(),
                );
            }
            CypherGuardParsingError::WhereAfterReturn { .. } => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "WHERE clauses must come before RETURN - move WHERE after MATCH".to_string(),
                );
            }
            CypherGuardParsingError::WithAfterReturn { .. } => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "WITH clauses must come before RETURN - move WITH earlier in the query"
                        .to_string(),
                );
            }
            CypherGuardParsingError::UnwindAfterReturn { .. } => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "UNWIND clauses must come before RETURN - move UNWIND earlier in the query"
                        .to_string(),
                );
            }
            CypherGuardParsingError::WhereBeforeMatch { .. } => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "WHERE must come after MATCH, UNWIND, or WITH - add a MATCH clause first"
                        .to_string(),
                );
            }
            CypherGuardParsingError::ReturnAfterReturn { .. } => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "Only one RETURN clause is allowed per query - combine into a single RETURN"
                        .to_string(),
                );
            }
            CypherGuardParsingError::OrderByBeforeReturn => {
                syntax_errors.push(error_str);
                suggestions.push("ORDER BY must come after RETURN or WITH clause".to_string());
            }
            CypherGuardParsingError::SkipBeforeReturn => {
                syntax_errors.push(error_str);
                suggestions
                    .push("SKIP must come after RETURN, WITH, or ORDER BY clause".to_string());
            }
            CypherGuardParsingError::LimitBeforeReturn => {
                syntax_errors.push(error_str);
                suggestions.push(
                    "LIMIT must come after RETURN, WITH, ORDER BY, or SKIP clause".to_string(),
                );
            }

            // Structure and clause errors
            CypherGuardParsingError::MissingRequiredClause { clause } => {
                syntax_errors.push(error_str);
                suggestions.push(format!(
                    "Add the required {} clause to complete your query",
                    clause
                ));
            }
            CypherGuardParsingError::InvalidClauseOrder { context, details } => {
                syntax_errors.push(error_str);
                suggestions.push(format!("Fix clause order in {}: {}", context, details));
            }

            // Variable errors
            CypherGuardParsingError::UndefinedVariable(var_name) => {
                syntax_errors.push(error_str);
                suggestions.push(format!(
                    "Define variable '{}' in a MATCH, WITH, or other clause before using it",
                    var_name
                ));
            }

            // Basic syntax errors
            CypherGuardParsingError::ExpectedToken { expected, found } => {
                syntax_errors.push(error_str);
                suggestions.push(format!(
                    "Expected '{}' but found '{}' - check syntax around this area",
                    expected, found
                ));
            }
            CypherGuardParsingError::InvalidSyntax(msg) => {
                syntax_errors.push(error_str);
                suggestions.push(format!(
                    "Invalid syntax: {} - review Cypher syntax rules",
                    msg
                ));
            }
            CypherGuardParsingError::UnexpectedEnd => {
                syntax_errors.push(error_str);
                suggestions.push("Query appears incomplete - check for missing closing parentheses, brackets, or clauses".to_string());
            }

            // Pattern and expression errors
            CypherGuardParsingError::InvalidPattern { context, details } => {
                syntax_errors.push(error_str);
                suggestions.push(format!(
                    "Invalid pattern in {}: {} - check node/relationship syntax",
                    context, details
                ));
            }
            CypherGuardParsingError::InvalidWhereCondition { context, details } => {
                syntax_errors.push(error_str);
                suggestions.push(format!("Invalid WHERE condition in {}: {} - check comparison operators and expressions", context, details));
            }
            CypherGuardParsingError::InvalidExpression { context, details } => {
                syntax_errors.push(error_str);
                suggestions.push(format!(
                    "Invalid expression in {}: {} - check function calls and syntax",
                    context, details
                ));
            }

            // Low-level parsing errors
            CypherGuardParsingError::Nom(_) => {
                syntax_errors.push(error_str);
                suggestions.push("Fundamental parsing error - check basic query structure, parentheses, and syntax".to_string());
            }
        }
    }

    // Remove duplicate suggestions
    suggestions.sort();
    suggestions.dedup();

    // Calculate total error count
    let total_errors = validation_error_strings.len() + parsing_errors.len();

    Ok(StructuredErrors {
        has_errors: total_errors > 0,
        error_count: total_errors as u32,
        categories: StructuredErrorCategories {
            schema_errors,
            property_errors,
            syntax_errors,
            type_errors,
            parsing_errors: parsing_error_list,
        },
        query,
        suggestions,
    })
}
