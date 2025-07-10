#![allow(deprecated)]

use ::cypher_guard::{
    get_cypher_validation_errors, parse_query as parse_query_rust, validate_cypher_with_schema,
    CypherGuardError, CypherGuardParsingError, CypherGuardSchemaError, CypherGuardValidationError,
    DbSchema, DbSchemaConstraint, DbSchemaIndex, DbSchemaMetadata, DbSchemaProperty,
    DbSchemaRelationshipPattern, PropertyType,
};
use pyo3::prelude::*;
use pyo3::types::PyDict;

// === Error Conversion Helpers ===
fn convert_cypher_error(py: Python, err: CypherGuardError) -> PyErr {
    match err {
        CypherGuardError::Parsing(e) => convert_parsing_error(py, e),
        CypherGuardError::Validation(e) => convert_validation_error(py, e),
        CypherGuardError::Schema(e) => convert_schema_error(py, e),
        CypherGuardError::InvalidQuery(msg) => PyErr::new::<pyo3::exceptions::PyValueError, _>(msg),
    }
}

fn convert_parsing_error(_py: Python, err: CypherGuardParsingError) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(err.to_string())
}

fn convert_validation_error(_py: Python, err: CypherGuardValidationError) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(err.to_string())
}

fn convert_schema_error(_py: Python, err: CypherGuardSchemaError) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyValueError, _>(err.to_string())
}

// === Python API Functions ===
#[pyfunction]
pub fn validate_cypher(py: Python, query: &str, schema_json: &str) -> PyResult<bool> {
    let schema =
        DbSchema::from_json_string(schema_json).map_err(|e| convert_cypher_error(py, e))?;
    validate_cypher_with_schema(query, &schema).map_err(|e| convert_cypher_error(py, e))
}

#[pyfunction]
pub fn get_validation_errors(py: Python, query: &str, schema_json: &str) -> PyResult<Vec<String>> {
    let schema =
        DbSchema::from_json_string(schema_json).map_err(|e| convert_cypher_error(py, e))?;
    Ok(get_cypher_validation_errors(query, &schema))
}

#[pyfunction]
pub fn parse_query(py: Python, query: &str) -> PyResult<PyObject> {
    match parse_query_rust(query) {
        Ok(_ast) => Ok(PyDict::new_bound(py).into()),
        Err(e) => Err(convert_parsing_error(py, e)),
    }
}

#[pymodule]
fn cypher_guard(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<DbSchema>()?;
    m.add_class::<DbSchemaProperty>()?;
    m.add_class::<PropertyType>()?;
    m.add_class::<DbSchemaRelationshipPattern>()?;
    m.add_class::<DbSchemaConstraint>()?;
    m.add_class::<DbSchemaIndex>()?;
    m.add_class::<DbSchemaMetadata>()?;
    m.add_function(wrap_pyfunction!(validate_cypher, m)?)?;
    m.add_function(wrap_pyfunction!(get_validation_errors, m)?)?;
    m.add_function(wrap_pyfunction!(parse_query, m)?)?;
    Ok(())
}
