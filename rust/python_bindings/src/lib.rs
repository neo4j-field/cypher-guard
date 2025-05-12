use ::cypher_guard::{get_cypher_validation_errors, validate_cypher_with_schema, DbSchema};
use pyo3::prelude::*;

#[pyfunction]
fn validate_cypher_py(query: &str, schema_json: &str) -> PyResult<bool> {
    let schema = match DbSchema::from_json_str(schema_json) {
        Ok(s) => s,
        Err(_) => return Ok(false),
    };
    match validate_cypher_with_schema(query, &schema) {
        Ok(valid) => Ok(valid),
        Err(_) => Ok(false),
    }
}

#[pyfunction]
fn get_validation_errors_py(query: &str, schema_json: &str) -> PyResult<Vec<String>> {
    let schema = match DbSchema::from_json_str(schema_json) {
        Ok(s) => s,
        Err(_) => return Ok(vec!["Invalid schema JSON".to_string()]),
    };
    Ok(get_cypher_validation_errors(query, &schema))
}

#[pymodule]
fn cypher_guard(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate_cypher_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_validation_errors_py, m)?)?;
    Ok(())
}
