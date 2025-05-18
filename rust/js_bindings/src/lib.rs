use cypher_guard::{get_cypher_validation_errors, validate_cypher_with_schema, DbSchema};
use napi_derive::napi;

#[napi]
pub fn validate_cypher_js(query: String, schema_json: String) -> napi::Result<bool> {
    let schema = DbSchema::from_json_str(&schema_json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;
    validate_cypher_with_schema(&query, &schema)
        .map_err(|e| napi::Error::from_reason(format!("Validation error: {}", e)))
}

#[napi]
pub fn get_validation_errors_js(query: String, schema_json: String) -> napi::Result<Vec<String>> {
    let schema = DbSchema::from_json_str(&schema_json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;
    Ok(get_cypher_validation_errors(&query, &schema))
}
