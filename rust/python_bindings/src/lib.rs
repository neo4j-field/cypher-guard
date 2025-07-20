#![allow(deprecated)]

use ::cypher_guard::{
    get_cypher_validation_errors, parse_query as parse_query_rust, validate_cypher_with_schema,
    CypherGuardError, CypherGuardParsingError, CypherGuardSchemaError, CypherGuardValidationError,
    DbSchema, DbSchemaConstraint, DbSchemaIndex, DbSchemaMetadata, DbSchemaProperty,
    DbSchemaRelationshipPattern, PropertyType,
};
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyDict;

// Base exception for all validation errors
create_exception!(cypher_guard, CypherValidationError, PyException);
create_exception!(cypher_guard, InvalidNodeLabel, CypherValidationError);
create_exception!(cypher_guard, InvalidRelationshipType, CypherValidationError);
create_exception!(cypher_guard, InvalidNodeProperty, CypherValidationError);
create_exception!(
    cypher_guard,
    InvalidRelationshipProperty,
    CypherValidationError
);
create_exception!(cypher_guard, InvalidPropertyAccess, CypherValidationError);
create_exception!(cypher_guard, InvalidPropertyName, CypherValidationError);
create_exception!(cypher_guard, UndefinedVariable, CypherValidationError);
create_exception!(cypher_guard, TypeMismatch, CypherValidationError);
create_exception!(cypher_guard, InvalidRelationship, CypherValidationError);
create_exception!(cypher_guard, InvalidLabel, CypherValidationError);
create_exception!(cypher_guard, InvalidPropertyType, CypherValidationError);

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
    match err {
        CypherGuardValidationError::InvalidNodeLabel(label) => {
            InvalidNodeLabel::new_err(format!("Invalid node label: {}", label))
        }
        CypherGuardValidationError::InvalidRelationshipType(rel_type) => {
            InvalidRelationshipType::new_err(format!("Invalid relationship type: {}", rel_type))
        }
        CypherGuardValidationError::InvalidNodeProperty { label, property } => {
            InvalidNodeProperty::new_err(format!(
                "Invalid node property '{}' on label '{}'",
                property, label
            ))
        }
        CypherGuardValidationError::InvalidRelationshipProperty { rel_type, property } => {
            InvalidRelationshipProperty::new_err(format!(
                "Invalid relationship property '{}' on type '{}'",
                property, rel_type
            ))
        }
        CypherGuardValidationError::InvalidPropertyAccess {
            variable,
            property,
            context,
        } => InvalidPropertyAccess::new_err(format!(
            "Invalid property access '{}.{}' in {} clause",
            variable, property, context
        )),
        CypherGuardValidationError::InvalidPropertyName(name) => {
            InvalidPropertyName::new_err(format!("Invalid property name: {}", name))
        }
        CypherGuardValidationError::UndefinedVariable(var) => {
            UndefinedVariable::new_err(format!("Undefined variable: {}", var))
        }
        CypherGuardValidationError::TypeMismatch { expected, actual } => TypeMismatch::new_err(
            format!("Type mismatch: expected {}, got {}", expected, actual),
        ),
        CypherGuardValidationError::InvalidRelationship(rel) => {
            InvalidRelationship::new_err(format!("Invalid relationship: {}", rel))
        }
        CypherGuardValidationError::InvalidLabel(label) => {
            InvalidLabel::new_err(format!("Invalid label: {}", label))
        }
        CypherGuardValidationError::InvalidPropertyType {
            variable,
            property,
            expected_type,
            actual_value,
        } => InvalidPropertyType::new_err(format!(
            "Invalid property type for '{}.{}': expected {}, got value '{}'",
            variable, property, expected_type, actual_value
        )),
    }
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
fn cypher_guard(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
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

    // Expose error classes using the simpler approach from PyO3 docs
    m.add(
        "CypherValidationError",
        py.get_type::<CypherValidationError>(),
    )?;
    m.add("InvalidNodeLabel", py.get_type::<InvalidNodeLabel>())?;
    m.add(
        "InvalidRelationshipType",
        py.get_type::<InvalidRelationshipType>(),
    )?;
    m.add("InvalidNodeProperty", py.get_type::<InvalidNodeProperty>())?;
    m.add(
        "InvalidRelationshipProperty",
        py.get_type::<InvalidRelationshipProperty>(),
    )?;
    m.add(
        "InvalidPropertyAccess",
        py.get_type::<InvalidPropertyAccess>(),
    )?;
    m.add("InvalidPropertyName", py.get_type::<InvalidPropertyName>())?;
    m.add("UndefinedVariable", py.get_type::<UndefinedVariable>())?;
    m.add("TypeMismatch", py.get_type::<TypeMismatch>())?;
    m.add("InvalidRelationship", py.get_type::<InvalidRelationship>())?;
    m.add("InvalidLabel", py.get_type::<InvalidLabel>())?;
    m.add("InvalidPropertyType", py.get_type::<InvalidPropertyType>())?;

    Ok(())
}
