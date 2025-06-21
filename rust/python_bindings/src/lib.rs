use ::cypher_guard::{
    get_cypher_validation_errors, parse_query, validate_cypher_with_schema, CypherGuardError,
    CypherGuardParsingError, CypherGuardSchemaError, CypherGuardValidationError, DbSchema,
    DbSchemaConstraint, DbSchemaIndex, DbSchemaMetadata, DbSchemaProperty,
    DbSchemaRelationshipPattern, PropertyType,
};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::io::Write;

// === Custom Python Exception Classes ===
#[pyclass(extends=PyException)]
pub struct PyCypherGuardError {
    #[pyo3(get)]
    pub message: String,
}
#[pymethods]
impl PyCypherGuardError {
    #[new]
    fn new(message: String) -> Self {
        Self { message }
    }
}

#[pyclass(extends=PyException)]
pub struct PyCypherGuardParsingError {
    #[pyo3(get)]
    pub message: String,
}
#[pymethods]
impl PyCypherGuardParsingError {
    #[new]
    fn new(message: String) -> Self {
        Self { message }
    }
}

#[pyclass(extends=PyException)]
pub struct PyCypherGuardValidationError {
    #[pyo3(get)]
    pub message: String,
}
#[pymethods]
impl PyCypherGuardValidationError {
    #[new]
    fn new(message: String) -> Self {
        Self { message }
    }
}

#[pyclass(extends=PyException)]
pub struct PyCypherGuardSchemaError {
    #[pyo3(get)]
    pub message: String,
}
#[pymethods]
impl PyCypherGuardSchemaError {
    #[new]
    fn new(message: String) -> Self {
        Self { message }
    }
}

// === Error Conversion Helpers ===
fn convert_cypher_error(py: Python, err: CypherGuardError) -> PyErr {
    match err {
        CypherGuardError::Parsing(e) => convert_parsing_error(py, e),
        CypherGuardError::Validation(e) => convert_validation_error(py, e),
        CypherGuardError::Schema(e) => convert_schema_error(py, e),
        CypherGuardError::InvalidQuery(msg) => {
            PyErr::from_type(py.get_type::<PyCypherGuardError>(), (msg,))
        }
    }
}

fn convert_parsing_error(py: Python, err: CypherGuardParsingError) -> PyErr {
    PyErr::from_type(
        py.get_type::<PyCypherGuardParsingError>(),
        (err.to_string(),),
    )
}
fn convert_validation_error(py: Python, err: CypherGuardValidationError) -> PyErr {
    PyErr::from_type(
        py.get_type::<PyCypherGuardValidationError>(),
        (err.to_string(),),
    )
}
fn convert_schema_error(py: Python, err: CypherGuardSchemaError) -> PyErr {
    PyErr::from_type(
        py.get_type::<PyCypherGuardSchemaError>(),
        (err.to_string(),),
    )
}

// === Python API Functions ===
#[pyfunction]
pub fn validate_cypher_py(py: Python, query: &str, schema_json: &str) -> PyResult<bool> {
    let schema =
        DbSchema::from_json_string(schema_json).map_err(|e| convert_cypher_error(py, e))?;
    validate_cypher_with_schema(query, &schema).map_err(|e| convert_cypher_error(py, e))
}

#[pyfunction]
pub fn get_validation_errors_py(
    py: Python,
    query: &str,
    schema_json: &str,
) -> PyResult<Vec<String>> {
    let schema =
        DbSchema::from_json_string(schema_json).map_err(|e| convert_cypher_error(py, e))?;
    Ok(get_cypher_validation_errors(query, &schema))
}

#[pyfunction]
pub fn parse_query_py(py: Python, query: &str) -> PyResult<PyObject> {
    match parse_query(query) {
        Ok(_ast) => Ok(PyDict::new(py).into()),
        Err(e) => Err(convert_parsing_error(py, e)),
    }
}

#[pymodule]
fn cypher_guard(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("CypherGuardError", py.get_type::<PyCypherGuardError>())?;
    m.add(
        "CypherGuardParsingError",
        py.get_type::<PyCypherGuardParsingError>(),
    )?;
    m.add(
        "CypherGuardValidationError",
        py.get_type::<PyCypherGuardValidationError>(),
    )?;
    m.add(
        "CypherGuardSchemaError",
        py.get_type::<PyCypherGuardSchemaError>(),
    )?;
    m.add_class::<DbSchema>()?;
    m.add_class::<DbSchemaProperty>()?;
    m.add_class::<PropertyType>()?;
    m.add_class::<DbSchemaRelationshipPattern>()?;
    m.add_class::<DbSchemaConstraint>()?;
    m.add_class::<DbSchemaIndex>()?;
    m.add_class::<DbSchemaMetadata>()?;
    m.add_function(wrap_pyfunction!(validate_cypher_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_validation_errors_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_query_py, m)?)?;
    Ok(())
}
