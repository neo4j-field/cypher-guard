use ::cypher_guard::{get_cypher_validation_errors, validate_cypher_with_schema, DbSchema, DbSchemaProperty, PropertyType};
use pyo3::prelude::*;
use std::io::Write;



#[pyfunction]
pub fn validate_cypher_py(query: &str, schema_json: &str) -> PyResult<bool> {
    println!("[PYBIND] Validating query: {}", query);
    println!("[PYBIND] Schema JSON: {}", schema_json);
    std::io::stdout().flush().unwrap();
    let schema = DbSchema::from_json_string(schema_json).map_err(|e| {
        println!("[PYBIND] Schema error: {:?}", e);
        std::io::stdout().flush().unwrap();
        PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid schema")
    })?;
    println!("[PYBIND] Schema loaded successfully");
    std::io::stdout().flush().unwrap();
    validate_cypher_with_schema(query, &schema).map_err(|e| {
        println!("[PYBIND] Validation error: {:?}", e);
        std::io::stdout().flush().unwrap();
        PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid query")
    })
}

#[pyfunction]
pub fn get_validation_errors_py(query: &str, schema_json: &str) -> PyResult<Vec<String>> {
    println!("[PYBIND] Getting validation errors for query: {}", query);
    println!("[PYBIND] Schema JSON: {}", schema_json);
    std::io::stdout().flush().unwrap();
    let schema = DbSchema::from_json_string(schema_json).map_err(|e| {
        println!("[PYBIND] Schema error: {:?}", e);
        std::io::stdout().flush().unwrap();
        PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid schema")
    })?;
    println!("[PYBIND] Schema loaded successfully");
    std::io::stdout().flush().unwrap();
    Ok(get_cypher_validation_errors(query, &schema))
}

#[pymodule]
fn cypher_guard(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DbSchema>()?;
    m.add_class::<DbSchemaProperty>()?;
    m.add_class::<PropertyType>()?;
    // m.add_class::<DbSchemaRelationshipPattern>()?;
    // m.add_class::<DbSchemaConstraint>()?;
    // m.add_class::<DbSchemaIndex>()?;
    m.add_function(wrap_pyfunction!(validate_cypher_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_validation_errors_py, m)?)?;
    Ok(())
}
