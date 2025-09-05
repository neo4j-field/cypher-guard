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
use pyo3::types::{PyAny, PyDict};

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

/// Validate a Cypher query against a schema.
/// 
/// Args:
///     query (str): The Cypher query string to validate
///     schema (str | DbSchema): Either a JSON schema string or a DbSchema object
/// 
/// Returns:
///     bool: True if the query is valid according to the schema, False otherwise
/// 
/// Raises:
///     CypherValidationError: If validation fails due to schema violations
///     TypeError: If schema is neither a string nor DbSchema object
/// 
/// Examples:
///     >>> schema_json = '{"node_props": {"Person": [{"name": "name", "neo4j_type": "STRING"}]}, "rel_props": {}, "relationships": [], "metadata": {"index": [], "constraint": []}}'
///     >>> validate_cypher("MATCH (p:Person) RETURN p.name", schema_json)
///     True
///     
///     >>> schema = DbSchema.from_dict({"node_props": {"Person": [{"name": "name", "neo4j_type": "STRING"}]}, "rel_props": {}, "relationships": [], "metadata": {"index": [], "constraint": []}})
///     >>> validate_cypher("MATCH (p:Person) RETURN p.name", schema)
///     True
#[pyfunction]
#[pyo3(text_signature = "(query, schema, /)")]
pub fn validate_cypher(py: Python, query: &str, schema: &Bound<'_, PyAny>) -> PyResult<bool> {
    let db_schema = if let Ok(schema_str) = schema.extract::<&str>() {
        // Schema provided as JSON string
        DbSchema::from_json_string(schema_str).map_err(|e| convert_cypher_error(py, e))?
    } else if let Ok(schema_obj) = schema.extract::<DbSchema>() {
        // Schema provided as DbSchema object
        schema_obj
    } else {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "schema must be either a JSON string or DbSchema object"
        ));
    };
    
    validate_cypher_with_schema(query, &db_schema).map_err(|e| convert_cypher_error(py, e))
}

#[pyfunction]
#[pyo3(text_signature = "(query, schema, /)")]
/// Get all validation errors for a Cypher query against a schema.
/// 
/// Args:
///     query (str): The Cypher query string to validate
///     schema (str | DbSchema): Either a JSON schema string or a DbSchema object
/// 
/// Returns:
///     List[str]: List of validation error messages. Empty list if query is valid.
/// 
/// Raises:
///     TypeError: If schema is neither a string nor DbSchema object
/// 
/// Examples:
///     >>> schema_json = '{"node_props": {"Person": [{"name": "name", "neo4j_type": "STRING"}]}, "rel_props": {}, "relationships": [], "metadata": {"index": [], "constraint": []}}'
///     >>> get_validation_errors("MATCH (p:InvalidLabel) RETURN p.name", schema_json)
///     ['Invalid node label: InvalidLabel']
///     
///     >>> schema = DbSchema.from_dict({"node_props": {"Person": [{"name": "name", "neo4j_type": "STRING"}]}, "rel_props": {}, "relationships": [], "metadata": {"index": [], "constraint": []}})
///     >>> get_validation_errors("MATCH (p:Person) RETURN p.name", schema)
///     []
pub fn get_validation_errors(py: Python, query: &str, schema: &Bound<'_, PyAny>) -> PyResult<Vec<String>> {
    let db_schema = if let Ok(schema_str) = schema.extract::<&str>() {
        // Schema provided as JSON string
        DbSchema::from_json_string(schema_str).map_err(|e| convert_cypher_error(py, e))?
    } else if let Ok(schema_obj) = schema.extract::<DbSchema>() {
        // Schema provided as DbSchema object
        schema_obj
    } else {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "schema must be either a JSON string or DbSchema object"
        ));
    };
    
    Ok(get_cypher_validation_errors(query, &db_schema))
}

#[pyfunction]
#[pyo3(text_signature = "(query, /)")]
/// Parse a Cypher query into an Abstract Syntax Tree (AST).
/// 
/// Args:
///     query (str): The Cypher query string to parse
/// 
/// Returns:
///     dict: The parsed AST as a Python dictionary (currently returns empty dict)
/// 
/// Raises:
///     ValueError: If the query has syntax errors and cannot be parsed
/// 
/// Examples:
///     >>> parse_query("MATCH (n) RETURN n")
///     {}
/// 
/// Note:
///     This function currently returns an empty dictionary. Full AST serialization
///     to Python dictionaries is planned for future versions.
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
    // `parse_query` is not implemented yet
    // m.add_function(wrap_pyfunction!(parse_query, m)?)?;

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
