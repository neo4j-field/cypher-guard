#![allow(deprecated)]

use ::cypher_guard::{
    get_cypher_validation_errors, parse_query as parse_query_rust, validate_cypher_with_schema,
    CypherGuardError, CypherGuardParsingError, CypherGuardSchemaError, CypherGuardValidationError,
    DbSchema as CoreDbSchema, DbSchemaProperty as CoreDbSchemaProperty,
    DbSchemaRelationshipPattern as CoreDbSchemaRelationshipPattern,
    PropertyType as CorePropertyType,
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

// === Python Wrapper Types ===

/// Python wrapper for PropertyType enum
#[pyclass]
#[derive(Debug, Clone)]
pub struct PropertyType {
    inner: CorePropertyType,
}

#[pymethods]
impl PropertyType {
    #[new]
    fn new(type_str: &str) -> PyResult<Self> {
        let inner = match type_str.to_uppercase().as_str() {
            "STRING" => CorePropertyType::STRING,
            "INTEGER" => CorePropertyType::INTEGER,
            "FLOAT" => CorePropertyType::FLOAT,
            "BOOLEAN" => CorePropertyType::BOOLEAN,
            "POINT" => CorePropertyType::POINT,
            "DATE_TIME" => CorePropertyType::DATE_TIME,
            "LIST" => CorePropertyType::LIST,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid property type: {}",
                    type_str
                )))
            }
        };
        Ok(Self { inner })
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("PropertyType({})", self.inner)
    }
}

/// Python wrapper for DbSchemaProperty
#[pyclass]
#[derive(Debug, Clone)]
pub struct DbSchemaProperty {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub neo4j_type: String,
    #[pyo3(get)]
    pub enum_values: Option<Vec<String>>,
    #[pyo3(get)]
    pub min_value: Option<f64>,
    #[pyo3(get)]
    pub max_value: Option<f64>,
    #[pyo3(get)]
    pub distinct_value_count: Option<i64>,
    #[pyo3(get)]
    pub example_values: Option<Vec<String>>,
    inner: CoreDbSchemaProperty,
}

#[pymethods]
impl DbSchemaProperty {
    #[new]
    fn new(name: String, neo4j_type: &str) -> PyResult<Self> {
        let property_type = match neo4j_type.to_uppercase().as_str() {
            "STRING" => CorePropertyType::STRING,
            "INTEGER" => CorePropertyType::INTEGER,
            "FLOAT" => CorePropertyType::FLOAT,
            "BOOLEAN" => CorePropertyType::BOOLEAN,
            "POINT" => CorePropertyType::POINT,
            "DATE_TIME" => CorePropertyType::DATE_TIME,
            "LIST" => CorePropertyType::LIST,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid property type: {}",
                    neo4j_type
                )))
            }
        };

        let inner = CoreDbSchemaProperty {
            name,
            neo4j_type: property_type,
            enum_values: None,
            min_value: None,
            max_value: None,
            distinct_value_count: None,
            example_values: None,
        };

        Ok(Self {
            name: inner.name.clone(),
            neo4j_type: inner.neo4j_type.to_string(),
            enum_values: inner.enum_values.clone(),
            min_value: inner.min_value,
            max_value: inner.max_value,
            distinct_value_count: inner.distinct_value_count,
            example_values: inner.example_values.clone(),
            inner,
        })
    }

    #[classmethod]
    #[pyo3(name = "from_dict")]
    fn py_from_dict(
        _cls: &Bound<'_, pyo3::types::PyType>,
        dict: &Bound<'_, pyo3::types::PyDict>,
    ) -> PyResult<Self> {
        let name = match dict.get_item("name")? {
            Some(value) => value.extract::<String>()?,
            None => match dict.get_item("property")? {
                Some(value) => value.extract::<String>()?,
                None => {
                    return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                        "Missing 'name' or 'property' field",
                    ))
                }
            },
        };

        let neo4j_type = match dict.get_item("neo4j_type")? {
            Some(value) => value.extract::<String>()?,
            None => match dict.get_item("type")? {
                Some(value) => value.extract::<String>()?,
                None => {
                    return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                        "Missing 'neo4j_type' or 'type' field",
                    ))
                }
            },
        };

        Self::new(name, &neo4j_type)
    }

    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        dict.set_item("name", &self.name)?;
        dict.set_item("neo4j_type", &self.neo4j_type)?;
        if let Some(enum_values) = &self.enum_values {
            dict.set_item("enum_values", enum_values)?;
        }
        if let Some(min_value) = self.min_value {
            dict.set_item("min_value", min_value)?;
        }
        if let Some(max_value) = self.max_value {
            dict.set_item("max_value", max_value)?;
        }
        if let Some(distinct_value_count) = self.distinct_value_count {
            dict.set_item("distinct_value_count", distinct_value_count)?;
        }
        if let Some(example_values) = &self.example_values {
            dict.set_item("example_values", example_values)?;
        }
        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "DbSchemaProperty(name={}, neo4j_type={})",
            self.name, self.neo4j_type
        )
    }
}

/// Python wrapper for DbSchemaRelationshipPattern
#[pyclass]
#[derive(Debug, Clone)]
pub struct DbSchemaRelationshipPattern {
    #[pyo3(get)]
    pub start: String,
    #[pyo3(get)]
    pub end: String,
    #[pyo3(get)]
    pub rel_type: String,
    inner: CoreDbSchemaRelationshipPattern,
}

#[pymethods]
impl DbSchemaRelationshipPattern {
    #[new]
    fn new(start: String, end: String, rel_type: String) -> Self {
        let inner = CoreDbSchemaRelationshipPattern {
            start: start.clone(),
            end: end.clone(),
            rel_type: rel_type.clone(),
        };
        Self {
            start,
            end,
            rel_type,
            inner,
        }
    }

    #[classmethod]
    #[pyo3(name = "from_dict")]
    fn py_from_dict(
        _cls: &Bound<'_, pyo3::types::PyType>,
        dict: &Bound<'_, pyo3::types::PyDict>,
    ) -> PyResult<Self> {
        let start = dict
            .get_item("start")?
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("Missing 'start' field"))?
            .extract::<String>()?;
        let end = dict
            .get_item("end")?
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("Missing 'end' field"))?
            .extract::<String>()?;
        let rel_type = match dict.get_item("rel_type")? {
            Some(value) => value.extract::<String>()?,
            None => match dict.get_item("type")? {
                Some(value) => value.extract::<String>()?,
                None => {
                    return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                        "Missing 'rel_type' or 'type' field for Relationship Pattern",
                    ))
                }
            },
        };
        Ok(Self::new(start, end, rel_type))
    }

    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        dict.set_item("start", &self.start)?;
        dict.set_item("end", &self.end)?;
        dict.set_item("rel_type", &self.rel_type)?;
        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "DbSchemaRelationshipPattern(start={}, end={}, rel_type={})",
            self.start, self.end, self.rel_type
        )
    }
}

/// Python wrapper for DbSchema
#[pyclass]
#[derive(Debug, Clone)]
pub struct DbSchema {
    #[pyo3(get)]
    pub node_props: std::collections::HashMap<String, Vec<DbSchemaProperty>>,
    #[pyo3(get)]
    pub relationships: Vec<DbSchemaRelationshipPattern>,
    inner: CoreDbSchema,
}

#[pymethods]
impl DbSchema {
    #[new]
    fn new() -> Self {
        let inner = CoreDbSchema::new();
        Self {
            node_props: std::collections::HashMap::new(),
            relationships: Vec::new(),
            inner,
        }
    }

    #[classmethod]
    #[pyo3(name = "from_json_string")]
    fn py_from_json_string(
        _cls: &Bound<'_, pyo3::types::PyType>,
        json_str: &str,
    ) -> PyResult<Self> {
        let inner = CoreDbSchema::from_json_string(json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

        // Convert core node_props to Python wrapper node_props
        let node_props = inner
            .node_props
            .iter()
            .map(|(label, core_properties)| {
                let properties = core_properties
                    .iter()
                    .map(|core_prop| DbSchemaProperty {
                        name: core_prop.name.clone(),
                        neo4j_type: core_prop.neo4j_type.to_string(),
                        enum_values: core_prop.enum_values.clone(),
                        min_value: core_prop.min_value,
                        max_value: core_prop.max_value,
                        distinct_value_count: core_prop.distinct_value_count,
                        example_values: core_prop.example_values.clone(),
                        inner: core_prop.clone(),
                    })
                    .collect();

                (label.clone(), properties)
            })
            .collect();

        // Convert core relationships to Python wrapper relationships
        let relationships = inner
            .relationships
            .iter()
            .map(|core_rel| DbSchemaRelationshipPattern {
                start: core_rel.start.clone(),
                end: core_rel.end.clone(),
                rel_type: core_rel.rel_type.clone(),
                inner: core_rel.clone(),
            })
            .collect();

        Ok(Self {
            node_props,
            relationships,
            inner,
        })
    }

    fn has_label(&self, label: &str) -> bool {
        self.inner.has_label(label)
    }

    fn has_node_property(&self, label: &str, name: &str) -> bool {
        self.inner.has_node_property(label, name)
    }

    #[classmethod]
    #[pyo3(name = "from_dict")]
    fn py_from_dict(
        _cls: &Bound<'_, pyo3::types::PyType>,
        dict: &Bound<'_, pyo3::types::PyDict>,
    ) -> PyResult<Self> {
        let mut core_schema = CoreDbSchema::new();

        // Parse node_props (Neo4j GraphRAG standard format)
        if let Some(node_props_item) = dict.get_item("node_props")? {
            let node_props_dict = node_props_item.downcast::<pyo3::types::PyDict>()?;
            for (label, props_item) in node_props_dict.iter() {
                let label = label.extract::<String>()?;

                core_schema
                    .add_label(&label)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

                let props_list = props_item.downcast::<pyo3::types::PyList>()?;
                for prop_item in props_list.iter() {
                    let prop_dict = prop_item.downcast::<pyo3::types::PyDict>()?;
                    let prop = DbSchemaProperty::py_from_dict(_cls, prop_dict)?;
                    core_schema
                        .add_node_property(&label, &prop.inner)
                        .map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
                        })?;
                }
            }
        }

        // Parse rel_props (if present)
        if let Some(rel_props_item) = dict.get_item("rel_props")? {
            let rel_props_dict = rel_props_item.downcast::<pyo3::types::PyDict>()?;
            for (rel_type, properties) in rel_props_dict.iter() {
                let rel_type_str = rel_type.extract::<String>()?;
                let properties_list = properties.downcast::<pyo3::types::PyList>()?;
                for prop_item in properties_list.iter() {
                    let prop_dict = prop_item.downcast::<pyo3::types::PyDict>()?;
                    let prop = DbSchemaProperty::py_from_dict(_cls, prop_dict)?;
                    core_schema
                        .add_relationship_property(&rel_type_str, &prop.inner)
                        .map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())
                        })?;
                }
            }
        }

        // Parse relationships (if present)
        if let Some(relationships_item) = dict.get_item("relationships")? {
            let relationships_list = relationships_item.downcast::<pyo3::types::PyList>()?;
            for rel_item in relationships_list.iter() {
                let rel_dict = rel_item.downcast::<pyo3::types::PyDict>()?;
                let rel = DbSchemaRelationshipPattern::py_from_dict(_cls, rel_dict)?;
                core_schema
                    .add_relationship_pattern(rel.inner)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
            }
        }

        // Convert core schema to wrapper
        let node_props = core_schema
            .node_props
            .iter()
            .map(|(label, core_properties)| {
                let properties = core_properties
                    .iter()
                    .map(|core_prop| DbSchemaProperty {
                        name: core_prop.name.clone(),
                        neo4j_type: core_prop.neo4j_type.to_string(),
                        enum_values: core_prop.enum_values.clone(),
                        min_value: core_prop.min_value,
                        max_value: core_prop.max_value,
                        distinct_value_count: core_prop.distinct_value_count,
                        example_values: core_prop.example_values.clone(),
                        inner: core_prop.clone(),
                    })
                    .collect();
                (label.clone(), properties)
            })
            .collect();

        let relationships = core_schema
            .relationships
            .iter()
            .map(|core_rel| DbSchemaRelationshipPattern {
                start: core_rel.start.clone(),
                end: core_rel.end.clone(),
                rel_type: core_rel.rel_type.clone(),
                inner: core_rel.clone(),
            })
            .collect();

        Ok(Self {
            node_props,
            relationships,
            inner: core_schema,
        })
    }

    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);

        // Convert node_props to dict
        let node_props_dict = pyo3::types::PyDict::new_bound(py);
        for (label, properties) in &self.node_props {
            let props_list = pyo3::types::PyList::empty_bound(py);
            for prop in properties {
                props_list.append(prop.py_to_dict(py)?)?;
            }
            node_props_dict.set_item(label, props_list)?;
        }
        dict.set_item("node_props", node_props_dict)?;

        // Convert relationships to dict
        let rels_list = pyo3::types::PyList::empty_bound(py);
        for rel in &self.relationships {
            rels_list.append(rel.py_to_dict(py)?)?;
        }
        dict.set_item("relationships", rels_list)?;

        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "DbSchema(node_props={} labels, relationships={} types)",
            self.node_props.len(),
            self.relationships.len()
        )
    }
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
        DbSchema::py_from_json_string(&py.get_type_bound::<DbSchema>(), schema_str)?
    } else if let Ok(schema_obj) = schema.extract::<DbSchema>() {
        // Schema provided as DbSchema object
        schema_obj
    } else {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "schema must be either a JSON string or DbSchema object",
        ));
    };

    validate_cypher_with_schema(query, &db_schema.inner).map_err(|e| convert_cypher_error(py, e))
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
pub fn get_validation_errors(
    py: Python,
    query: &str,
    schema: &Bound<'_, PyAny>,
) -> PyResult<Vec<String>> {
    let db_schema = if let Ok(schema_str) = schema.extract::<&str>() {
        // Schema provided as JSON string
        DbSchema::py_from_json_string(&py.get_type_bound::<DbSchema>(), schema_str)?
    } else if let Ok(schema_obj) = schema.extract::<DbSchema>() {
        // Schema provided as DbSchema object
        schema_obj
    } else {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "schema must be either a JSON string or DbSchema object",
        ));
    };

    Ok(get_cypher_validation_errors(query, &db_schema.inner))
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
    m.add_class::<DbSchemaRelationshipPattern>()?;
    m.add_class::<PropertyType>()?;
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
