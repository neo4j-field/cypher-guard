use cypher_guard::{
    get_cypher_validation_errors, parse_query as parse_query_rust, validate_cypher_with_schema, DbSchema,
};
use napi_derive::napi;
use napi::{JsObject, Env, Result as NapiResult};

#[napi]
pub fn validate_cypher(query: String, schema_json: String) -> napi::Result<bool> {
    let schema = DbSchema::from_json_string(&schema_json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;
    validate_cypher_with_schema(&query, &schema)
        .map_err(|e| napi::Error::from_reason(format!("Validation error: {}", e)))
}

#[napi]
pub fn get_validation_errors(query: String, schema_json: String) -> napi::Result<Vec<String>> {
    let schema = DbSchema::from_json_string(&schema_json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;
    Ok(get_cypher_validation_errors(&query, &schema))
}

#[napi]
pub fn parse_query(env: Env, query: String) -> NapiResult<JsObject> {
    match parse_query_rust(&query) {
        Ok(_ast) => env.create_object(), // Return an empty JS object for now
        Err(e) => Err(napi::Error::from_reason(format!("Parse error: {}", e))),
    }
}

#[napi(object)]
pub struct JsPropertyType {
    pub type_name: String,
}

#[napi(object)]
pub struct JsDbSchemaProperty {
    pub name: String,
    pub neo4j_type: JsPropertyType,
    pub enum_values: Option<Vec<String>>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub distinct_value_count: Option<i64>,
    pub example_values: Option<Vec<String>>,
}

#[napi(object)]
pub struct JsDbSchemaRelationshipPattern {
    pub start: String,
    pub end: String,
    pub rel_type: String,
}

#[napi(object)]
pub struct JsDbSchemaConstraint {
    pub id: i64,
    pub name: String,
    pub constraint_type: String,
    pub entity_type: String,
    pub labels_or_types: Vec<String>,
    pub properties: Vec<String>,
    pub owned_index: String,
    pub property_type: Option<String>,
}

#[napi(object)]
pub struct JsDbSchemaIndex {
    pub label: String,
    pub properties: Vec<String>,
    pub size: i64,
    pub index_type: String,
    pub values_selectivity: f64,
    pub distinct_values: f64,
}

#[napi(object)]
pub struct JsDbSchemaMetadata {
    pub constraint: Vec<JsDbSchemaConstraint>,
    pub index: Vec<JsDbSchemaIndex>,
}

#[napi(object)]
pub struct JsDbSchema {
    pub node_props: std::collections::HashMap<String, Vec<JsDbSchemaProperty>>,
    pub rel_props: std::collections::HashMap<String, Vec<JsDbSchemaProperty>>,
    pub relationships: Vec<JsDbSchemaRelationshipPattern>,
    pub metadata: JsDbSchemaMetadata,
}
