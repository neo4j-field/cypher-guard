use napi_derive::napi;
use cypher_guard::{validate_cypher, get_validation_errors};
use serde_json::Value;

#[napi]
pub fn validate_cypher_js(query: String, schema_json: String) -> napi::Result<bool> {
    let schema: Value = serde_json::from_str(&schema_json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;
    
    validate_cypher(&query, &schema)
        .map_err(|e| napi::Error::from_reason(format!("Validation error: {}", e)))
}

#[napi]
pub fn get_validation_errors_js(query: String, schema_json: String) -> napi::Result<Vec<String>> {
    let schema: Value = serde_json::from_str(&schema_json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse schema JSON: {}", e)))?;
    
    get_validation_errors(&query, &schema)
        .map_err(|e| napi::Error::from_reason(format!("Validation error: {}", e)))
} 