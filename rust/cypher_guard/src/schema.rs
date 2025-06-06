use crate::errors::CypherGuardError;
#[cfg(feature = "python-bindings")]
use pyo3::prelude::*;
#[cfg(feature = "python-bindings")]
use pyo3::types::{PyDict, PyList, PyType};
#[cfg(feature = "python-bindings")]
use pyo3::PyObject;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub type Result<T> = std::result::Result<T, CypherGuardError>;

/// Enum representing possible property types in the schema.
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
#[cfg_attr(feature = "python-bindings", pyclass)]
pub enum PropertyType {
    /// String property
    #[pyo3(name = "STRING")]
    STRING,
    /// Integer property
    #[pyo3(name = "INTEGER")]
    INTEGER,
    /// Float property
    #[pyo3(name = "FLOAT")]
    FLOAT,
    /// Boolean property
    #[pyo3(name = "BOOLEAN")]
    BOOLEAN,
    /// Point property (for spatial data)
    #[pyo3(name = "POINT")]
    POINT,
    /// DateTime property
    #[pyo3(name = "DATE_TIME")]
    DATE_TIME,
    /// List property
    #[pyo3(name = "LIST")]
    LIST, // / Custom enum type (referenced by name)
          // #[cfg_attr(feature = "python-bindings", pyo3(name = "ENUM"))]
          // ENUM(String),
}

impl fmt::Display for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PropertyType::STRING => write!(f, "STRING"),
            PropertyType::INTEGER => write!(f, "INTEGER"),
            PropertyType::FLOAT => write!(f, "FLOAT"),
            PropertyType::BOOLEAN => write!(f, "BOOLEAN"),
            PropertyType::POINT => write!(f, "POINT"),
            PropertyType::DATE_TIME => write!(f, "DATE_TIME"),
            PropertyType::LIST => write!(f, "LIST"),
        }
    }
}

impl PropertyType {
    pub fn from_string(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "STRING" | "STR" => Ok(PropertyType::STRING),
            "INTEGER" | "INT" => Ok(PropertyType::INTEGER),
            "FLOAT" => Ok(PropertyType::FLOAT),
            "BOOLEAN" | "BOOL" => Ok(PropertyType::BOOLEAN),
            "POINT" => Ok(PropertyType::POINT),
            "DATE_TIME" => Ok(PropertyType::DATE_TIME),
            "LIST" => Ok(PropertyType::LIST),
            _ => Err(CypherGuardError::SchemaError),
        }
    }
}
#[cfg(feature = "python-bindings")]
#[pymethods]
impl PropertyType {
    #[new]
    fn py_new(type_name: &str) -> PyResult<Self> {
        match type_name.to_uppercase().as_str() {
            "STRING" => Ok(PropertyType::STRING),
            "INTEGER" => Ok(PropertyType::INTEGER),
            "FLOAT" => Ok(PropertyType::FLOAT),
            "BOOLEAN" => Ok(PropertyType::BOOLEAN),
            "POINT" => Ok(PropertyType::POINT),
            "DATE_TIME" => Ok(PropertyType::DATE_TIME),
            "LIST" => Ok(PropertyType::LIST),
            _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Invalid property type: {}",
                type_name
            ))),
        }
    }

    #[classmethod]
    fn string(_cls: &Bound<'_, PyType>) -> Self {
        PropertyType::STRING
    }

    #[classmethod]
    fn integer(_cls: &Bound<'_, PyType>) -> Self {
        PropertyType::INTEGER
    }

    #[classmethod]
    fn float(_cls: &Bound<'_, PyType>) -> Self {
        PropertyType::FLOAT
    }

    #[classmethod]
    fn boolean(_cls: &Bound<'_, PyType>) -> Self {
        PropertyType::BOOLEAN
    }

    #[classmethod]
    fn point(_cls: &Bound<'_, PyType>) -> Self {
        PropertyType::POINT
    }

    #[classmethod]
    fn datetime(_cls: &Bound<'_, PyType>) -> Self {
        PropertyType::DATE_TIME
    }

    #[classmethod]
    fn list(_cls: &Bound<'_, PyType>) -> Self {
        PropertyType::LIST
    }

    #[classmethod]
    #[pyo3(name = "from_string", signature = (s))]
    fn py_from_string(_cls: &Bound<'_, PyType>, s: &str) -> PyResult<Self> {
        // Case insensitive matching for simple types
        match s.trim().to_uppercase().as_str() {
            "STRING" | "STR" => Ok(PropertyType::STRING),
            "INTEGER" | "INT"  => Ok(PropertyType::INTEGER),
            "FLOAT" => Ok(PropertyType::FLOAT),
            "BOOLEAN" | "BOOL" => Ok(PropertyType::BOOLEAN),
            "POINT"  => Ok(PropertyType::POINT),
            "DATE_TIME" => Ok(PropertyType::DATE_TIME),
            "LIST" => Ok(PropertyType::LIST),
            _ => Err(pyo3::exceptions::PyValueError::new_err(
                format!("Invalid property type: '{}'. Valid types: STRING, INTEGER, FLOAT, BOOLEAN, POINT, DATE_TIME, LIST", s)
            ))
        }
    }

    fn __repr__(&self) -> String {
        format!("PropertyType.{}", self)
    }

    fn __str__(&self) -> String {
        self.to_string()
    }
}

/// Structure representing a user-defined enum type for properties.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct EnumType {
    /// Name of the enum type
    name: String,
    /// Allowed values for the enum
    values: Vec<String>,
}

/// Structure representing a property in the schema.
/// This may be for either a node or a relationship.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "python-bindings", pyclass)]
pub struct DbSchemaProperty {
    /// Name of the property
    #[pyo3(get, set)]
    pub name: String,
    /// Neo4j type of the property
    #[pyo3(get, set)]
    pub neo4j_type: PropertyType,
    /// Enum values for the property, optional
    #[pyo3(get, set)]
    pub enum_values: Option<Vec<String>>,
    /// Minimum value for the property, optional
    #[pyo3(get, set)]
    pub min_value: Option<f64>,
    /// Maximum value for the property, optional
    #[pyo3(get, set)]
    pub max_value: Option<f64>,
    /// Distinct value count for the property, optional
    #[pyo3(get, set)]
    pub distinct_value_count: Option<i64>,
    /// Example values for the property, optional
    #[pyo3(get, set)]
    pub example_values: Option<Vec<String>>,
}

impl Default for DbSchemaProperty {
    fn default() -> Self {
        Self {
            name: "prop".to_string(),
            neo4j_type: PropertyType::STRING,
            enum_values: None,
            min_value: None,
            max_value: None,
            distinct_value_count: None,
            example_values: None,
        }
    }
}

impl DbSchemaProperty {
    pub fn new(name: &str, neo4j_type: PropertyType) -> Self {
        Self {
            name: name.to_string(),
            neo4j_type,
            ..Default::default()
        }
    }

    pub fn has_enum_values(&self) -> bool {
        self.enum_values.is_some()
    }

    pub fn has_min_value(&self) -> bool {
        self.min_value.is_some()
    }

    pub fn has_max_value(&self) -> bool {
        self.max_value.is_some()
    }

    pub fn has_distinct_value_count(&self) -> bool {
        self.distinct_value_count.is_some()
    }

    pub fn has_example_values(&self) -> bool {
        self.example_values.is_some()
    }

    pub fn set_enum_values(&mut self, enum_values: Vec<String>) {
        self.enum_values = Some(enum_values);
    }

    pub fn set_min_value(&mut self, min_value: f64) {
        self.min_value = Some(min_value);
    }

    pub fn set_max_value(&mut self, max_value: f64) {
        self.max_value = Some(max_value);
    }

    pub fn set_distinct_value_count(&mut self, distinct_value_count: i64) {
        self.distinct_value_count = Some(distinct_value_count);
    }

    pub fn set_example_values(&mut self, example_values: Vec<String>) {
        self.example_values = Some(example_values);
    }
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl DbSchemaProperty {
    #[classmethod]
    fn extract_float_value_from_string_or_number(
        _cls: &Bound<'_, PyType>,
        value: Bound<'_, PyAny>,
    ) -> Option<f64> {
        if let Ok(num) = value.extract::<f64>() {
            Some(num)
        } else if let Ok(s) = value.extract::<String>() {
            s.parse::<f64>().ok()
        } else {
            None
        }
    }

    #[new]
    #[pyo3(signature = (name, neo4j_type, enum_values=None, min_value=None, max_value=None, distinct_value_count=None, example_values=None))]
    fn py_new(
        name: String,
        neo4j_type: String,
        enum_values: Option<Vec<String>>,
        min_value: Option<f64>,
        max_value: Option<f64>,
        distinct_value_count: Option<i64>,
        example_values: Option<Vec<String>>,
    ) -> PyResult<Self> {
        // Convert string neo4j_type to PropertyType enum
        let property_type = match PropertyType::from_string(&neo4j_type) {
            Ok(property_type) => property_type,
            Err(e) => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "Invalid property type: {}",
                    e
                )))
            }
        };

        let mut prop = Self::new(&name, property_type);

        if let Some(values) = enum_values {
            prop.set_enum_values(values);
        }

        if let Some(value) = min_value {
            prop.set_min_value(value);
        }

        if let Some(value) = max_value {
            prop.set_max_value(value);
        }

        if let Some(value) = distinct_value_count {
            prop.set_distinct_value_count(value);
        }

        if let Some(values) = example_values {
            prop.set_example_values(values);
        }

        Ok(prop)
    }

    #[classmethod]
    #[pyo3(name = "from_dict", signature = (dict))]
    fn py_from_dict(_cls: &Bound<'_, PyType>, dict: &Bound<'_, PyDict>) -> PyResult<Self> {
        let name = match dict.get_item("name")? {
            Some(value) => value.extract::<String>()?,
            None => match dict.get_item("property")? {
                Some(value) => value.extract::<String>()?,
                None => {
                    return Err(pyo3::exceptions::PyKeyError::new_err(
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
                    return Err(pyo3::exceptions::PyKeyError::new_err(
                        "Missing 'neo4j_type' or 'type' field",
                    ))
                }
            },
        };
        let distinct_value_count = match dict.get_item("distinct_value_count")? {
            Some(value) if !value.is_none() => Some(value.extract::<i64>()?),
            _ => match dict.get_item("distinct_values")? {
                Some(value) if !value.is_none() => Some(value.extract::<i64>()?),
                _ => None,
            },
        };
        let enum_values = match dict.get_item("enum_values")? {
            Some(value) if !value.is_none() => Some(value.extract::<Vec<String>>()?),
            _ => match dict.get_item("values")? {
                Some(value)
                    if !value.is_none()
                        && value.len().is_ok_and(|len| {
                            len == distinct_value_count.unwrap_or(0) as usize
                        }) =>
                {
                    Some(value.extract::<Vec<String>>()?)
                }
                _ => None,
            },
        };

        // Only set min and max values if the property type is INTEGER or FLOAT
        let mut min_value: Option<f64> = None;
        let mut max_value: Option<f64> = None;
        if neo4j_type == "INTEGER" || neo4j_type == "FLOAT" {
            min_value = match dict.get_item("min_value")? {
                Some(value) if !value.is_none() => {
                    Self::extract_float_value_from_string_or_number(_cls, value)
                }
                _ => match dict.get_item("min")? {
                    Some(value) if !value.is_none() => {
                        Self::extract_float_value_from_string_or_number(_cls, value)
                    }
                    _ => None,
                },
            };
        };

        if neo4j_type == "INTEGER" || neo4j_type == "FLOAT" {
            max_value = match dict.get_item("max_value")? {
                Some(value) if !value.is_none() => {
                    Self::extract_float_value_from_string_or_number(_cls, value)
                }
                _ => match dict.get_item("max")? {
                    Some(value) if !value.is_none() => {
                        Self::extract_float_value_from_string_or_number(_cls, value)
                    }
                    _ => None,
                },
            };
        };

        let example_values = match dict.get_item("example_values")? {
            Some(value) if !value.is_none() => Some(value.extract::<Vec<String>>()?),
            _ => match dict.get_item("values")? {
                Some(value) if !value.is_none() => Some(value.extract::<Vec<String>>()?),
                _ => None,
            },
        };
        Self::py_new(
            name,
            neo4j_type.to_string(),
            enum_values,
            min_value,
            max_value,
            distinct_value_count,
            example_values,
        )
    }

    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        dict.set_item("name", &self.name)?;
        dict.set_item("neo4j_type", self.neo4j_type.to_string())?;
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
        format!("DbSchemaProperty(name={}, neo4j_type={}, enum_values={}, min_value={}, max_value={}, distinct_value_count={}, example_values={})", self.name, self.neo4j_type, match &self.enum_values {
            Some(values) => format!("['{}']", values.join("', '")),
            None => "None".to_string(),
        }, self.min_value.map_or("None".to_string(), |v| v.to_string()), self.max_value.map_or("None".to_string(), |v| v.to_string()), self.distinct_value_count.map_or("None".to_string(), |v| v.to_string()), match &self.example_values {
            Some(values) => format!("[{}]", values.join(", ")).to_string(),
            None => "None".to_string(),
        })
    }

    fn __str__(&self) -> String {
        format!("{}: {}", self.name, self.neo4j_type)
    }
}
/// Structure representing a relationship in the schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "python-bindings", pyclass)]
pub struct DbSchemaRelationshipPattern {
    /// Start node label of the relationship
    #[pyo3(get, set)]
    pub start: String,
    /// End node label of the relationship
    #[pyo3(get, set)]
    pub end: String,
    /// Type of the relationship
    #[pyo3(get, set)]
    pub rel_type: String,
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl DbSchemaRelationshipPattern {
    #[new]
    fn py_new(start: &str, end: &str, rel_type: &str) -> PyResult<Self> {
        Ok(Self {
            start: start.to_string(),
            end: end.to_string(),
            rel_type: rel_type.to_string(),
        })
    }

    #[classmethod]
    #[pyo3(name = "from_dict", signature = (dict))]
    fn py_from_dict(_cls: &Bound<'_, PyType>, dict: &Bound<'_, PyDict>) -> PyResult<Self> {
        let start = dict
            .get_item("start")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("Missing 'start' field"))?
            .extract::<String>()?;
        let end = dict
            .get_item("end")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("Missing 'end' field"))?
            .extract::<String>()?;
        let rel_type = match dict.get_item("rel_type")? {
            Some(value) => value.extract::<String>()?,
            None => match dict.get_item("type")? {
                Some(value) => value.extract::<String>()?,
                None => {
                    return Err(pyo3::exceptions::PyKeyError::new_err(
                        "Missing 'rel_type' or 'type' field for Relationship Pattern",
                    ))
                }
            },
        };
        Ok(Self {
            start,
            end,
            rel_type,
        })
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

    fn __str__(&self) -> String {
        format!("(:{})-[:{}]->(:{})", self.start, self.rel_type, self.end)
    }
}
/// Structure representing a constraint in the schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "python-bindings", pyclass)]
pub struct DbSchemaConstraint {
    /// ID of the constraint
    #[pyo3(get, set)]
    pub id: i64,
    /// Name of the constraint
    #[pyo3(get, set)]
    pub name: String,
    /// Type of the constraint
    #[pyo3(get, set)]
    pub constraint_type: String,
    /// Entity type of the constraint
    #[pyo3(get, set)]
    pub entity_type: String,
    /// Labels or types of the constraint
    #[pyo3(get, set)]
    pub labels_or_types: Vec<String>,
    /// Properties of the constraint
    #[pyo3(get, set)]
    pub properties: Vec<String>,
    /// Owned index of the constraint
    #[pyo3(get, set)]
    pub owned_index: String,
    /// Property type of the constraint
    #[pyo3(get, set)]
    pub property_type: Option<String>,
}

impl DbSchemaConstraint {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i64,
        name: String,
        constraint_type: String,
        entity_type: String,
        labels_or_types: Vec<String>,
        properties: Vec<String>,
        owned_index: String,
        property_type: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            constraint_type,
            entity_type,
            labels_or_types,
            properties,
            owned_index,
            property_type,
        }
    }
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl DbSchemaConstraint {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (id, name, constraint_type, entity_type, labels_or_types, properties, owned_index, property_type))]
    fn py_new(
        id: i64,
        name: String,
        constraint_type: String,
        entity_type: String,
        labels_or_types: Vec<String>,
        properties: Vec<String>,
        owned_index: String,
        property_type: Option<String>,
    ) -> PyResult<Self> {
        Ok(Self {
            id,
            name,
            constraint_type,
            entity_type,
            labels_or_types,
            properties,
            owned_index,
            property_type,
        })
    }

    #[classmethod]
    #[pyo3(name = "from_dict", signature = (dict))]
    fn py_from_dict(_cls: &Bound<'_, PyType>, dict: &Bound<'_, PyDict>) -> PyResult<Self> {
        let id = dict
            .get_item("id")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("Missing 'id' field"))?
            .extract::<i64>()?;
        let name = dict
            .get_item("name")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("Missing 'name' field"))?
            .extract::<String>()?;
        let constraint_type = match dict.get_item("constraint_type")? {
            Some(value) => value.extract::<String>()?,
            None => match dict.get_item("type")? {
                Some(value) => value.extract::<String>()?,
                None => {
                    return Err(pyo3::exceptions::PyKeyError::new_err(
                        "Missing 'constraint_type' or 'type' field",
                    ))
                }
            },
        };
        let entity_type = match dict.get_item("entity_type")? {
            Some(value) => value.extract::<String>()?,
            None => match dict.get_item("entityType")? {
                Some(value) => value.extract::<String>()?,
                None => {
                    return Err(pyo3::exceptions::PyKeyError::new_err(
                        "Missing 'entity_type' or 'entityType' field",
                    ))
                }
            },
        };
        let labels_or_types = match dict.get_item("labels_or_types")? {
            Some(value) => value.extract::<Vec<String>>()?,
            None => match dict.get_item("labelsOrTypes")? {
                Some(value) => value.extract::<Vec<String>>()?,
                None => {
                    return Err(pyo3::exceptions::PyKeyError::new_err(
                        "Missing 'labels_or_types' or 'labelsOrTypes' field",
                    ))
                }
            },
        };

        let properties = dict
            .get_item("properties")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("Missing 'properties' field"))?
            .extract::<Vec<String>>()?;
        let owned_index = match dict.get_item("owned_index")? {
            Some(value) => value.extract::<String>()?,
            None => match dict.get_item("ownedIndex")? {
                Some(value) => value.extract::<String>()?,
                None => {
                    return Err(pyo3::exceptions::PyKeyError::new_err(
                        "Missing 'owned_index' or 'ownedIndex' field",
                    ))
                }
            },
        };
        let property_type = match dict.get_item("property_type")? {
            Some(value) if !value.is_none() => Some(value.extract::<String>()?),
            _ => match dict.get_item("propertyType")? {
                Some(value) if !value.is_none() => Some(value.extract::<String>()?),
                _ => None,
            },
        };
        Ok(Self {
            id,
            name,
            constraint_type,
            entity_type,
            labels_or_types,
            properties,
            owned_index,
            property_type,
        })
    }

    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        dict.set_item("id", self.id)?;
        dict.set_item("name", &self.name)?;
        dict.set_item("constraint_type", &self.constraint_type)?;
        dict.set_item("entity_type", &self.entity_type)?;
        dict.set_item("labels_or_types", &self.labels_or_types)?;
        dict.set_item("properties", &self.properties)?;
        dict.set_item("owned_index", &self.owned_index)?;
        dict.set_item("property_type", &self.property_type)?;
        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!("DbSchemaConstraint(id={}, name={}, constraint_type={}, entity_type={}, labels_or_types=[{}], properties=[{}], owned_index={}, property_type={})", 
            self.id,
            self.name,
            self.constraint_type,
            self.entity_type,
            self.labels_or_types.join(", "), 
            self.properties.join(", "), 
            self.owned_index,
            self.property_type.as_ref().map_or("None".to_string(), |pt| pt.clone())
        )
    }

    fn __str__(&self) -> String {
        format!(
            "{} CONSTRAINT {} ON {} ({}).{{{}}}",
            self.constraint_type,
            self.name,
            self.entity_type,
            self.labels_or_types.join(", "),
            self.properties.join(", "),
        )
    }
}

/// Structure representing an index in the schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "python-bindings", pyclass)]
pub struct DbSchemaIndex {
    /// Label of the index
    #[pyo3(get, set)]
    pub label: String,
    /// Properties of the index
    #[pyo3(get, set)]
    pub properties: Vec<String>,
    /// Size of the index
    #[pyo3(get, set)]
    pub size: i64,
    /// Type of the index
    #[pyo3(get, set)]
    pub index_type: String,
    /// Values selectivity of the index
    #[pyo3(get, set)]
    pub values_selectivity: f64,
    /// Distinct values of the index
    #[pyo3(get, set)]
    pub distinct_values: f64,
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl DbSchemaIndex {
    /// Create an index
    #[new]
    fn py_new(
        label: String,
        properties: Vec<String>,
        size: i64,
        index_type: String,
        values_selectivity: f64,
        distinct_values: f64,
    ) -> PyResult<Self> {
        Ok(Self {
            label,
            properties,
            size,
            index_type,
            values_selectivity,
            distinct_values,
        })
    }

    /// Create an index from a Python dictionary
    #[classmethod]
    #[pyo3(name = "from_dict", signature = (dict))]
    fn py_from_dict(_cls: &Bound<'_, PyType>, dict: &Bound<'_, PyDict>) -> PyResult<Self> {
        let label = dict
            .get_item("label")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("Missing 'label' field"))?
            .extract::<String>()?;
        let properties = dict
            .get_item("properties")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("Missing 'properties' field"))?
            .extract::<Vec<String>>()?;
        let size = dict
            .get_item("size")?
            .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("Missing 'size' field"))?
            .extract::<i64>()?;
        let index_type = match dict.get_item("index_type")? {
            Some(value) => value.extract::<String>()?,
            None => dict
                .get_item("type")?
                .ok_or_else(|| {
                    pyo3::exceptions::PyKeyError::new_err("Missing 'index_type' or 'type' field")
                })?
                .extract::<String>()?,
        };
        let values_selectivity = match dict.get_item("values_selectivity")? {
            Some(value) => value.extract::<f64>()?,
            None => dict
                .get_item("valuesSelectivity")?
                .ok_or_else(|| {
                    pyo3::exceptions::PyKeyError::new_err(
                        "Missing 'values_selectivity' or 'valuesSelectivity' field",
                    )
                })?
                .extract::<f64>()?,
        };
        let distinct_values = match dict.get_item("distinct_values")? {
            Some(value) => value.extract::<f64>()?,
            None => dict
                .get_item("distinctValues")?
                .ok_or_else(|| {
                    pyo3::exceptions::PyKeyError::new_err(
                        "Missing 'distinct_values' or 'distinctValues' field",
                    )
                })?
                .extract::<f64>()?,
        };
        Ok(Self {
            label,
            properties,
            size,
            index_type,
            values_selectivity,
            distinct_values,
        })
    }

    /// Convert the index to a Python dictionary
    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        dict.set_item("label", &self.label)?;
        dict.set_item("properties", &self.properties)?;
        dict.set_item("size", self.size)?;
        dict.set_item("index_type", &self.index_type)?;
        dict.set_item("values_selectivity", self.values_selectivity)?;
        dict.set_item("distinct_values", self.distinct_values)?;
        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!("DbSchemaIndex(label={}, properties=[{}], size={}, index_type={}, values_selectivity={}, distinct_values={})", self.label, self.properties.join(", "), self.size, self.index_type, self.values_selectivity, self.distinct_values)
    }

    fn __str__(&self) -> String {
        format!(
            "INDEX {} ON {} ({})",
            self.index_type,
            self.label,
            self.properties.join(", ")
        )
    }
}

/// Structure representing metadata for the schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "python-bindings", pyclass)]
pub struct DbSchemaMetadata {
    /// Constraints in the schema
    #[pyo3(get, set)]
    pub constraint: Vec<DbSchemaConstraint>,
    /// Indexes in the schema
    #[pyo3(get, set)]
    pub index: Vec<DbSchemaIndex>,
}

impl Default for DbSchemaMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl DbSchemaMetadata {
    pub fn new() -> Self {
        Self {
            constraint: Vec::new(),
            index: Vec::new(),
        }
    }
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl DbSchemaMetadata {
    #[new]
    fn py_new(constraint: Vec<DbSchemaConstraint>, index: Vec<DbSchemaIndex>) -> PyResult<Self> {
        Ok(Self { constraint, index })
    }

    /// Create metadata from a Python dictionary
    #[classmethod]
    #[pyo3(name = "from_dict", signature = (dict))]
    fn py_from_dict(_cls: &Bound<'_, PyType>, dict: &Bound<'_, PyDict>) -> PyResult<Self> {
        let constraint = match dict.get_item("constraint")? {
            Some(items) => {
                let iter = items.iter()?;
                iter.map(|item| {
                    let item = item?;
                    if let Ok(item) = item.downcast::<PyDict>() {
                        DbSchemaConstraint::py_from_dict(_cls, item)
                    } else {
                        Err(pyo3::exceptions::PyTypeError::new_err(
                            "Invalid constraint type",
                        ))
                    }
                })
                .collect::<PyResult<Vec<_>>>()?
            }
            None => vec![],
        };
        let index = match dict.get_item("index")? {
            Some(items) => {
                let iter = items.iter()?;
                iter.map(|item| {
                    let item = item?;
                    if let Ok(item) = item.downcast::<PyDict>() {
                        DbSchemaIndex::py_from_dict(_cls, item)
                    } else {
                        Err(pyo3::exceptions::PyTypeError::new_err("Invalid index type"))
                    }
                })
                .collect::<PyResult<Vec<_>>>()?
            }
            None => vec![],
        };
        Ok(Self { constraint, index })
    }
    /// Convert the metadata to a Python dictionary
    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new_bound(py);
        let constraint_dict: PyResult<Vec<PyObject>> =
            self.constraint.iter().map(|c| c.py_to_dict(py)).collect();
        dict.set_item("constraint", &constraint_dict?)?;
        let index_dict: PyResult<Vec<PyObject>> =
            self.index.iter().map(|i| i.py_to_dict(py)).collect();
        dict.set_item("index", &index_dict?)?;
        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "DbSchemaMetadata(constraint=[{}], index=[{}])",
            self.constraint
                .iter()
                .map(|c| c.__repr__())
                .collect::<Vec<String>>()
                .join(", "),
            self.index
                .iter()
                .map(|i| i.__repr__())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    fn __str__(&self) -> String {
        format!(
            "DbSchemaMetadata(constraint=[{}], index=[{}])",
            self.constraint
                .iter()
                .map(|c| c.__str__())
                .collect::<Vec<String>>()
                .join(", "),
            self.index
                .iter()
                .map(|i| i.__str__())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

/// Main schema struct for the graph database.
/// This is a collection of node labels, relationship types, properties, enums, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
#[cfg_attr(feature = "python-bindings", pyclass)]
pub struct DbSchema {
    /// Node keys and vector of properties for each node label
    #[pyo3(get, set)]
    pub node_props: HashMap<String, Vec<DbSchemaProperty>>,
    /// Relationship keys and vector of properties for each relationship type
    #[pyo3(get, set)]
    pub rel_props: HashMap<String, Vec<DbSchemaProperty>>,
    /// Vector of relationships
    #[pyo3(get, set)]
    pub relationships: Vec<DbSchemaRelationshipPattern>,
    /// Metadata about the schema containing constraint and index
    #[pyo3(get, set)]
    pub metadata: DbSchemaMetadata,
}

impl Default for DbSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl DbSchema {
    /// Create a new, empty schema
    pub fn new() -> Self {
        Self {
            node_props: HashMap::new(),
            rel_props: HashMap::new(),
            relationships: Vec::new(),
            metadata: DbSchemaMetadata::new(),
        }
    }

    /// Add a label to the schema
    pub fn add_label(&mut self, label: &str) -> Result<()> {
        if self.node_props.contains_key(label) {
            return Err(CypherGuardError::SchemaError);
        }
        self.node_props.insert(label.to_string(), Vec::new());
        Ok(())
    }

    /// Remove a label from the schema
    pub fn remove_label(&mut self, label: &str) -> Result<()> {
        if self.node_props.remove(label).is_some() {
            Ok(())
        } else {
            Err(CypherGuardError::SchemaError)
        }
    }

    /// Add a relationship type to the schema
    pub fn add_relationship(&mut self, relationship: &DbSchemaRelationshipPattern) -> Result<()> {
        if self.relationships.contains(relationship) {
            return Err(CypherGuardError::SchemaError);
        }
        self.relationships.push(relationship.clone());
        Ok(())
    }

    /// Remove a relationship type from the schema
    pub fn remove_relationship(
        &mut self,
        relationship: &DbSchemaRelationshipPattern,
    ) -> Result<()> {
        if let Some(index) = self.relationships.iter().position(|r| r == relationship) {
            self.relationships.swap_remove(index);
            self.rel_props.remove(&relationship.rel_type);
            Ok(())
        } else {
            Err(CypherGuardError::SchemaError)
        }
    }

    /// Add a node property to the schema
    pub fn add_node_property(&mut self, label: &str, property: &DbSchemaProperty) -> Result<()> {
        match self.node_props.get_mut(label) {
            // check for duplicate property by name
            Some(properties) => {
                if properties.iter().any(|p| p.name == property.name) {
                    return Err(CypherGuardError::SchemaError);
                }
            }
            // insert key and empty vector if key doesn't exist
            None => {
                self.add_label(label)?;
            }
        }

        self.node_props
            .entry(label.to_string())
            .or_default()
            .push(property.clone());
        Ok(())
    }

    /// Remove a property from the schema
    pub fn remove_node_property(&mut self, label: &str, name: &str) -> Result<()> {
        if let Some(properties) = self.node_props.get_mut(label) {
            if let Some(index) = properties.iter().position(|p| p.name == name) {
                properties.remove(index);
                Ok(())
            } else {
                Err(CypherGuardError::SchemaError)
            }
        } else {
            Err(CypherGuardError::SchemaError)
        }
    }

    /// Add a relationship property to the schema
    pub fn add_relationship_property(
        &mut self,
        rel_type: &str,
        property: &DbSchemaProperty,
    ) -> Result<()> {
        match self.rel_props.get_mut(rel_type) {
            // check for duplicate property by name
            Some(properties) => {
                if properties.iter().any(|p| p.name == property.name) {
                    return Err(CypherGuardError::SchemaError);
                }
            }
            // insert key and empty vector if key doesn't exist
            None => {
                self.rel_props.insert(rel_type.to_string(), Vec::new());
            }
        }

        self.rel_props
            .entry(rel_type.to_string())
            .or_default()
            .push(property.clone());
        Ok(())
    }

    /// Remove a property from the schema
    pub fn remove_relationship_property(&mut self, rel_type: &str, name: &str) -> Result<()> {
        if let Some(properties) = self.rel_props.get_mut(rel_type) {
            if let Some(index) = properties.iter().position(|p| p.name == name) {
                properties.remove(index);
                Ok(())
            } else {
                Err(CypherGuardError::SchemaError)
            }
        } else {
            Err(CypherGuardError::SchemaError)
        }
    }

    /// Check if a label exists
    pub fn has_label(&self, label: &str) -> bool {
        self.node_props.contains_key(label)
    }

    /// Check if a relationship type exists with any start and end node labels
    pub fn has_relationship_type(&self, rel_type: &str) -> bool {
        self.relationships.iter().any(|r| r.rel_type == rel_type)
    }

    /// check if a relationship with start and end node labels exists
    pub fn has_relationship(&self, relationship: &DbSchemaRelationshipPattern) -> bool {
        self.relationships.iter().any(|r| {
            r.start == relationship.start
                && r.end == relationship.end
                && r.rel_type == relationship.rel_type
        })
    }

    /// Check if a property exists
    pub fn has_node_property(&self, label: &str, name: &str) -> bool {
        println!("Checking node property: label={}, name={}", label, name); // Debug
        if !self.has_label(label) {
            println!("Label '{}' not found in schema", label); // Debug
            return false;
        }
        let result = self
            .node_props
            .get(label)
            .map(|props| props.iter().any(|p| p.name == name))
            .unwrap_or(false);
        println!("Node property check result: {}", result); // Debug
        result
    }

    /// Check if a relationship property exists
    pub fn has_relationship_property(&self, rel_type: &str, name: &str) -> bool {
        println!(
            "Checking relationship property: type={}, name={}",
            rel_type, name
        ); // Debug
        if !self.has_relationship_type(rel_type) {
            println!("Relationship type '{}' not found in schema", rel_type); // Debug
            return false;
        }
        let result = self
            .rel_props
            .get(rel_type)
            .map(|props| props.iter().any(|p| p.name == name))
            .unwrap_or(false);
        println!("Relationship property check result: {}", result); // Debug
        result
    }

    /// Get all node properties for a label
    pub fn get_node_properties(&self, label: &str) -> Option<&Vec<DbSchemaProperty>> {
        self.node_props.get(label)
    }

    /// Get all relationship properties for a relationship type
    pub fn get_relationship_properties(&self, rel_type: &str) -> Option<&Vec<DbSchemaProperty>> {
        self.rel_props.get(rel_type)
    }

    /// Get a node property by label and property name
    pub fn get_node_property(&self, label: &str, name: &str) -> Option<&DbSchemaProperty> {
        self.node_props
            .get(label)
            .and_then(|props| props.iter().find(|p| p.name == name))
    }

    /// Check if a property exists in any node
    pub fn has_property_in_nodes(&self, name: &str) -> bool {
        self.node_props
            .iter()
            .any(|(_, props)| props.iter().any(|p| p.name == name))
    }

    /// Check if a property exists in any relationship
    pub fn has_property_in_relationships(&self, name: &str) -> bool {
        self.rel_props
            .iter()
            .any(|(_, props)| props.iter().any(|p| p.name == name))
    }

    /// Get a relationship property by relationship type and property name
    pub fn get_relationship_property(
        &self,
        rel_type: &str,
        name: &str,
    ) -> Option<&DbSchemaProperty> {
        self.rel_props
            .get(rel_type)
            .and_then(|props| props.iter().find(|p| p.name == name))
    }
    /// Validate the schema for consistency and integrity.
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        // Check for duplicate names across labels, rel_types, enums
        let mut all_names = self.node_props.keys().cloned().collect::<Vec<_>>();
        all_names.extend(self.rel_props.keys().cloned());
        all_names.sort();
        for w in all_names.windows(2) {
            if w[0] == w[1] {
                errors.push(format!(
                    "Duplicate name found across schema elements: {}",
                    w[0]
                ));
            }
        }

        // Check property names for snake_case (simple check)
        for name in self
            .node_props
            .values()
            .flat_map(|props| props.iter().map(|p| p.name.clone()))
        {
            if !name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '_' || c.is_ascii_digit())
            {
                errors.push(format!(
                    "Property name '{}' should be snake_case (lowercase, digits, underscores)",
                    name
                ));
            }
        }

        // // Check that enum properties reference valid enums
        // for (label, properties) in &self.node_props {
        //     for prop in properties {
        //         if let PropertyType::ENUM(enum_name) = &prop.neo4j_type {
        //             if prop.enum_values.is_none() {
        //                 errors.push(format!(
        //                     "Property '{}' in node label '{}' references undefined enum type '{}'.",
        //                     prop.name, label, enum_name
        //                 ));
        //             }
        //         }
        //     }
        // }

        // for (rel_type, properties) in &self.rel_props {
        //     for prop in properties {
        //         if let PropertyType::ENUM(enum_name) = &prop.neo4j_type {
        //             if prop.enum_values.is_none() {
        //                 errors.push(format!(
        //                     "Property '{}' in relationship type '{}' references undefined enum type '{}'.",
        //                     prop.name, rel_type, enum_name
        //                 ));
        //             }
        //         }
        //     }
        // }

        errors
    }

    /// Load a DbSchema from a JSON string
    pub fn from_json_string(s: &str) -> Result<Self> {
        println!("Loading schema from JSON: {}", s); // Debug: Print input JSON
        let schema = serde_json::from_str::<DbSchema>(s).map_err(|e| {
            eprintln!("JSON parse error: {}", e);
            CypherGuardError::SchemaError
        })?;
        println!("Loaded schema: {:?}", schema); // Debug: Print loaded schema
        println!("Node properties: {:?}", schema.node_props); // Debug: Print node properties
        println!("Relationship properties: {:?}", schema.rel_props); // Debug: Print relationship properties
        println!("Relationships: {:?}", schema.relationships); // Debug: Print relationships
        Ok(schema)
    }

    /// Load a DbSchema from a JSON file
    pub fn from_json_file(path: &str) -> Result<Self> {
        let file = File::open(path).map_err(|_| CypherGuardError::SchemaError)?;
        let reader = BufReader::new(file);
        let schema = serde_json::from_reader(reader).map_err(|_| CypherGuardError::SchemaError)?;
        Ok(schema)
    }

    /// Serialize a DbSchema to a JSON string
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|_| CypherGuardError::SchemaError)
    }

    /// Serialize a DbSchema to a JSON file
    pub fn to_json_file(&self, path: &str) -> Result<()> {
        let file = File::create(path).map_err(|_| CypherGuardError::SchemaError)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self).map_err(|_| CypherGuardError::SchemaError)
    }
}

impl fmt::Display for DbSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Labels: {:?}", self.node_props.keys())?;
        writeln!(f, "Properties:")?;
        for (label, properties) in &self.node_props {
            writeln!(f, "  {}: {:?}", label, properties)?;
        }
        writeln!(f, "Relationship Types: {:?}", self.relationships)?;
        writeln!(f, "Relationship Properties:")?;
        for (rel_type, properties) in &self.rel_props {
            writeln!(f, "  {}: {:?}", rel_type, properties)?;
        }
        writeln!(f, "Constraints: {:?}", self.metadata.constraint)?;
        writeln!(f, "Indexes: {:?}", self.metadata.index)?;
        Ok(())
    }
}

#[cfg(feature = "python-bindings")]
#[pymethods]
impl DbSchema {
    #[new]
    pub fn py_new(
        node_props: HashMap<String, Vec<DbSchemaProperty>>,
        rel_props: HashMap<String, Vec<DbSchemaProperty>>,
        relationships: Vec<DbSchemaRelationshipPattern>,
        metadata: DbSchemaMetadata,
    ) -> PyResult<Self> {
        Ok(Self {
            node_props,
            rel_props,
            relationships,
            metadata,
        })
    }

    #[classmethod]
    #[pyo3(name = "from_dict", signature = (dict))]
    fn py_from_dict(_cls: &Bound<'_, PyType>, dict: &Bound<'_, PyDict>) -> PyResult<Self> {
        let node_props = match dict.get_item("node_props")? {
            Some(items) => {
                let items = items.downcast::<PyDict>()?;
                let mut node_props = HashMap::new();
                for (label, properties) in items.iter() {
                    let properties_iter = properties.iter()?;
                    let mut props = Vec::new();
                    for property_item in properties_iter {
                        let property = property_item?;
                        if let Ok(property_dict) = property.downcast::<PyDict>() {
                            props.push(DbSchemaProperty::py_from_dict(_cls, property_dict)?);
                        } else {
                            return Err(pyo3::exceptions::PyTypeError::new_err(
                                "property is not a dictionary",
                            ));
                        }
                    }
                    node_props.insert(label.extract::<String>()?, props);
                }
                node_props
            }
            None => {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "node_props is required",
                ))
            }
        };

        let rel_props = match dict.get_item("rel_props")? {
            Some(items) => {
                let items = items.downcast::<PyDict>()?;
                let mut rel_props = HashMap::new();
                for (rel_type, properties) in items.iter() {
                    let properties_iter = properties.iter()?;
                    let mut props = Vec::new();
                    for property_item in properties_iter {
                        let property = property_item?;
                        if let Ok(property_dict) = property.downcast::<PyDict>() {
                            props.push(DbSchemaProperty::py_from_dict(_cls, property_dict)?);
                        } else {
                            return Err(pyo3::exceptions::PyTypeError::new_err(
                                "property is not a dictionary",
                            ));
                        }
                    }
                    rel_props.insert(rel_type.extract::<String>()?, props);
                }
                rel_props
            }
            None => HashMap::new(),
        };

        let relationships = match dict.get_item("relationships")? {
            Some(items) => {
                let iter = items.iter()?;
                let mut relationships = Vec::new();
                for item in iter {
                    let rel = item?;
                    if let Ok(rel_dict) = rel.downcast::<PyDict>() {
                        relationships
                            .push(DbSchemaRelationshipPattern::py_from_dict(_cls, rel_dict)?);
                    } else {
                        return Err(pyo3::exceptions::PyTypeError::new_err(
                            "relationship is not a dictionary",
                        ));
                    }
                }
                relationships
            }
            None => Vec::new(),
        };

        let metadata = match dict.get_item("metadata")? {
            Some(metadata) => {
                if let Ok(metadata_dict) = metadata.downcast::<PyDict>() {
                    DbSchemaMetadata::py_from_dict(_cls, metadata_dict)?
                } else {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "metadata is not a dictionary",
                    ));
                }
            }
            None => DbSchemaMetadata::new(),
        };

        Ok(Self {
            node_props,
            rel_props,
            relationships,
            metadata,
        })
    }

    fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new_bound(py);

        // Handle node properties
        let node_props_dict = PyDict::new_bound(py);
        for (label, properties) in &self.node_props {
            let prop_list = PyList::empty_bound(py);
            for prop in properties {
                prop_list.append(prop.py_to_dict(py)?)?;
            }
            node_props_dict.set_item(label, prop_list)?;
        }
        dict.set_item("node_props", node_props_dict)?;

        // Handle relationship properties
        let rel_props_dict = PyDict::new_bound(py);
        for (rel_type, properties) in &self.rel_props {
            let prop_list = PyList::empty_bound(py);
            for prop in properties {
                prop_list.append(prop.py_to_dict(py)?)?;
            }
            rel_props_dict.set_item(rel_type, prop_list)?;
        }
        dict.set_item("rel_props", rel_props_dict)?;

        // Handle relationships
        let rel_list = PyList::empty_bound(py);
        for rel in &self.relationships {
            rel_list.append(rel.py_to_dict(py)?)?;
        }
        dict.set_item("relationships", rel_list)?;

        // Handle metadata
        dict.set_item("metadata", self.metadata.py_to_dict(py)?)?;

        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "DbSchema(node_props={}, rel_props={}, relationships={}, metadata={})",
            {
                let node_props = self
                    .node_props
                    .iter()
                    .map(|(k, props)| {
                        format!(
                            "'{}': {}",
                            k,
                            props
                                .iter()
                                .map(|p| p.__repr__())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{{{}}}", node_props)
            },
            {
                let rel_props = self
                    .rel_props
                    .iter()
                    .map(|(k, props)| {
                        format!(
                            "{}: {}",
                            k,
                            props
                                .iter()
                                .map(|p| p.__repr__())
                                .collect::<Vec<String>>()
                                .join(", ")
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{{{}}}", rel_props)
            },
            {
                let relationships = self
                    .relationships
                    .iter()
                    .map(|c| c.__repr__())
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("[{}]", relationships)
            },
            self.metadata.__repr__()
        )
    }

    fn __str__(&self) -> String {
        format!(
            "Nodes:\n{}\nRelationship Properties:\n{}\nRelationships:\n{}\nConstraints:\n{}\nIndexes:\n{}",
            self.node_props.iter()
                .map(|(k, props)| k.to_string() + ":\n" + props.iter().map(|p| p.__str__()).collect::<Vec<String>>().join("\n").as_str())
                .collect::<Vec<String>>()
                .join("\n"),
            self.rel_props.iter()
                .map(|(k, props)| k.to_string() + ":\n" + props.iter().map(|p| p.__str__()).collect::<Vec<String>>().join("\n").as_str())
                .collect::<Vec<String>>()
                .join("\n"),
            self.relationships.iter()
                .map(|c| c.__str__())
                .collect::<Vec<String>>()
                .join("\n"),
            self.metadata.constraint.iter()
                .map(|c| c.__str__())
                .collect::<Vec<String>>()
                .join("\n"),
            self.metadata.index.iter()
                .map(|c| c.__str__())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_file;

    fn create_person_name_property() -> DbSchemaProperty {
        DbSchemaProperty::new("name", PropertyType::STRING)
    }

    fn create_person_age_property() -> DbSchemaProperty {
        DbSchemaProperty::new("age", PropertyType::INTEGER)
    }

    fn create_place_name_property() -> DbSchemaProperty {
        DbSchemaProperty::new("name", PropertyType::STRING)
    }

    fn create_knows_since_property() -> DbSchemaProperty {
        DbSchemaProperty::new("since", PropertyType::DATE_TIME)
    }

    // fn create_favorite_color_property() -> DbSchemaProperty {
    //     DbSchemaProperty::new(
    //         "favorite_color",
    //         PropertyType::ENUM("color_enum".to_string()),
    //     )
    // }

    fn create_lives_in_rel() -> DbSchemaRelationshipPattern {
        DbSchemaRelationshipPattern {
            start: "Person".to_string(),
            end: "Place".to_string(),
            rel_type: "LIVES_IN".to_string(),
        }
    }

    fn create_knows_rel() -> DbSchemaRelationshipPattern {
        DbSchemaRelationshipPattern {
            start: "Person".to_string(),
            end: "Person".to_string(),
            rel_type: "KNOWS".to_string(),
        }
    }

    fn create_test_schema_valid() -> DbSchema {
        let mut schema = DbSchema::new();
        let person_name_property = create_person_name_property();
        let person_age_property = create_person_age_property();
        let place_name_property = create_place_name_property();
        let knows_since_property = create_knows_since_property();

        let lives_in_rel = create_lives_in_rel();
        let knows_rel = create_knows_rel();

        schema
            .add_node_property("Person", &person_name_property)
            .unwrap();
        schema
            .add_node_property("Person", &person_age_property)
            .unwrap();
        schema
            .add_node_property("Place", &place_name_property)
            .unwrap();
        schema.add_relationship(&lives_in_rel).unwrap();
        schema.add_relationship(&knows_rel).unwrap();
        schema
            .add_relationship_property("KNOWS", &knows_since_property)
            .unwrap();
        schema
    }

    #[test]
    fn test_property_type_serialization() {
        let types = vec![
            PropertyType::STRING,
            PropertyType::INTEGER,
            PropertyType::FLOAT,
            PropertyType::BOOLEAN,
            PropertyType::DATE_TIME,
            // PropertyType::ENUM("ColorEnum".to_string()),
        ];
        for t in types {
            let serialized = serde_json::to_string(&t).unwrap();
            let deserialized: PropertyType = serde_json::from_str(&serialized).unwrap();
            assert_eq!(t, deserialized);
        }
    }

    #[test]
    fn test_enum_type_serialization() {
        let enum_type = EnumType {
            name: "ColorEnum".to_string(),
            values: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        };
        let serialized = serde_json::to_string(&enum_type).unwrap();
        let deserialized: EnumType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(enum_type, deserialized);
    }

    #[test]
    fn test_db_schema_creation_and_serialization() {
        let mut schema = DbSchema::new();
        schema.add_label("Person").unwrap();
        schema.add_relationship(&create_knows_rel()).unwrap();
        schema
            .add_node_property("Person", &create_person_age_property())
            .unwrap();

        let serialized = serde_json::to_string(&schema).unwrap();
        let deserialized: DbSchema = serde_json::from_str(&serialized).unwrap();
        assert_eq!(schema, deserialized);
        assert_eq!(
            deserialized.node_props.keys().collect::<Vec<_>>(),
            vec!["Person"]
        );
        assert_eq!(
            deserialized.relationships,
            vec![DbSchemaRelationshipPattern {
                start: "Person".to_string(),
                end: "Person".to_string(),
                rel_type: "KNOWS".to_string(),
            }]
        );
        assert_eq!(
            deserialized.node_props["Person"][0].neo4j_type,
            PropertyType::INTEGER
        );
    }

    #[test]
    fn test_add_and_remove_label() {
        let mut schema = DbSchema::new();
        assert!(schema.add_label("Person").is_ok());
        assert!(schema.has_label("Person"));
        // Duplicate label
        assert!(schema.add_label("Person").is_err());
        // Remove label
        assert!(schema.remove_label("Person").is_ok());
        assert!(!schema.has_label("Person"));
        // Remove non-existent label
        assert!(schema.remove_label("Person").is_err());
    }

    #[test]
    fn test_add_and_remove_relationship_type() {
        let mut schema = DbSchema::new();
        assert!(schema.add_relationship(&create_knows_rel()).is_ok());
        assert!(schema.has_relationship_type("KNOWS"));
        // Duplicate
        assert!(schema.add_relationship(&create_knows_rel()).is_err());
        // Remove
        assert!(schema.remove_relationship(&create_knows_rel()).is_ok());
        assert!(!schema.has_relationship_type("KNOWS"));
        // Remove non-existent
        assert!(schema.remove_relationship(&create_knows_rel()).is_err());
    }

    #[test]
    fn test_add_and_remove_node_property() {
        let mut schema = DbSchema::new();
        assert!(schema
            .add_node_property("Person", &create_person_name_property(),)
            .is_ok());
        assert!(schema.has_node_property("Person", "name"));
        // Duplicate
        assert!(schema
            .add_node_property("Person", &create_person_name_property())
            .is_err());
        // Remove
        assert!(schema.remove_node_property("Person", "name").is_ok());
        assert!(!schema.has_node_property("Person", "name"));
        // Remove non-existent
        assert!(schema.remove_node_property("Person", "name").is_err());
    }

    #[test]
    fn test_add_and_remove_relationship_property() {
        let mut schema = DbSchema::new();
        schema.add_relationship(&create_knows_rel()).unwrap();
        assert!(schema
            .add_relationship_property("KNOWS", &create_knows_since_property())
            .is_ok());
        assert!(schema.has_relationship_property("KNOWS", "since"));
        assert!(schema
            .add_relationship_property("KNOWS", &create_knows_since_property())
            .is_err());
        assert!(schema.has_relationship_property("KNOWS", "since"));
        assert!(schema
            .remove_relationship_property("KNOWS", "since")
            .is_ok());
        assert!(!schema.has_relationship_property("KNOWS", "since"));
        assert!(schema
            .remove_relationship_property("KNOWS", "since")
            .is_err());
    }

    #[test]
    fn test_schema_validation_no_errors() {
        let mut schema = DbSchema::new();
        schema.add_label("Person").unwrap();
        schema.add_relationship(&create_knows_rel()).unwrap();
        schema
            .add_node_property("Person", &create_person_age_property())
            .unwrap();

        // schema
        //     .add_node_property("Person", &create_favorite_color_property())
        //     .unwrap();
        let errors = schema.validate();
        assert!(
            errors.is_empty(),
            "Expected no validation errors, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_schema_validation_duplicate_names_add() {
        let mut schema = DbSchema::new();
        schema.add_label("Person").unwrap();
        assert!(schema.add_label("Person").is_err());
    }

    #[test]
    fn test_schema_validation_bad_property_name() {
        let mut schema = DbSchema::new();
        let bad_name_property = DbSchemaProperty::new("BadName", PropertyType::STRING);
        schema
            .add_node_property("Person", &bad_name_property)
            .unwrap();
        let errors = schema.validate();
        assert!(
            errors.iter().any(|e| e.contains("snake_case")),
            "Expected snake_case property name error"
        );
    }

    // #[test]
    // fn test_schema_validation_missing_enum_reference() {
    //     let mut schema = DbSchema::new();
    //     let missing_enum_property = DbSchemaProperty::new(
    //         "favorite_color",
    //         PropertyType::ENUM("missing_enum".to_string()),
    //     );
    //     schema
    //         .add_node_property("Person", &missing_enum_property)
    //         .unwrap();
    //     let errors = schema.validate();
    //     assert!(
    //         errors.iter().any(|e| e.contains("undefined enum type")),
    //         "Expected undefined enum type error"
    //     );
    // }

    #[test]
    fn test_from_json_str_valid_all_keys_present() {
        let json = r#"{
            "node_props": {"Person": [{"name": "age", "neo4j_type": {"type": "INTEGER"}}]},
            "rel_props": {},
            "relationships": [{"start": "Person", "end": "Person", "rel_type": "KNOWS"}],
            "metadata": {"index":[], "constraint":[]}
        }"#;
        let schema = DbSchema::from_json_string(json);
        assert!(schema.is_ok());
        let schema = schema.unwrap();
        assert!(schema.has_label("Person"));
        assert!(schema.has_relationship_type("KNOWS"));
        assert!(schema.has_node_property("Person", "age"));
    }

    #[test]
    fn test_from_json_str_valid_missing_keys() {
        let json = r#"{
            "node_props": {"Person": [{"name": "age", "neo4j_type": {"type": "INTEGER"}}]},
            "relationships": [{"start": "Person", "end": "Person", "rel_type": "KNOWS"}]
        }"#;
        let schema = DbSchema::from_json_string(json);
        assert!(schema.is_ok());
        let schema = schema.unwrap();
        assert!(schema.has_label("Person"));
        assert!(schema.has_relationship_type("KNOWS"));
        assert!(schema.has_node_property("Person", "age"));
    }

    #[test]
    fn test_from_json_str_invalid() {
        let json = "not valid json";
        let schema = DbSchema::from_json_string(json);
        assert!(schema.is_err());
    }

    #[test]
    fn test_to_json_string() {
        let mut schema = DbSchema::new();
        schema.add_label("Person").unwrap();
        let json = schema.to_json_string();
        assert!(json.is_ok());
        let json = json.unwrap();
        assert!(json.contains("Person"));
    }

    #[test]
    fn test_schema_has_label() {
        let schema = create_test_schema_valid();
        assert!(schema.has_label("Person"));
        assert!(schema.has_label("Place"));
        assert!(!schema.has_label("Company"));
    }

    #[test]
    fn test_schema_has_relationship_type() {
        let schema = create_test_schema_valid();
        assert!(schema.has_relationship_type("KNOWS"));
        assert!(schema.has_relationship_type("LIVES_IN"));
        assert!(!schema.has_relationship_type("WORKS_FOR"));
    }

    #[test]
    fn test_schema_has_relationship() {
        let schema = create_test_schema_valid();
        assert!(schema.has_relationship(&create_knows_rel()));
        assert!(schema.has_relationship(&create_lives_in_rel()));
    }

    #[test]
    fn test_schema_has_node_property() {
        let schema = create_test_schema_valid();
        assert!(schema.has_node_property("Person", "name"));
        assert!(schema.has_node_property("Place", "name"));
        assert!(!schema.has_node_property("Person", "born_at"));
    }

    #[test]
    fn test_schema_has_relationship_property() {
        let schema = create_test_schema_valid();
        assert!(schema.has_relationship_property("KNOWS", "since"));
        assert!(!schema.has_relationship_property("KNOWS", "age"));
    }

    #[test]
    fn test_schema_has_property_in_nodes() {
        let schema = create_test_schema_valid();
        assert!(schema.has_property_in_nodes("name"));
        assert!(schema.has_property_in_nodes("age"));
        assert!(!schema.has_property_in_nodes("born_at"));
    }

    #[test]
    fn test_schema_has_property_in_relationships() {
        let schema = create_test_schema_valid();
        assert!(schema.has_property_in_relationships("since"));
        assert!(!schema.has_property_in_relationships("age"));
        assert!(!schema.has_property_in_relationships("born_at"));
    }

    #[test]
    fn test_schema_get_node_properties() {
        let schema = create_test_schema_valid();
        assert_eq!(schema.get_node_properties("Person").unwrap().len(), 2);
        assert_eq!(schema.get_node_properties("Place").unwrap().len(), 1);
    }

    #[test]
    fn test_schema_get_relationship_properties() {
        let schema = create_test_schema_valid();
        assert_eq!(
            schema.get_relationship_properties("KNOWS").unwrap().len(),
            1
        );
    }

    #[test]
    fn test_schema_get_node_property() {
        let schema = create_test_schema_valid();
        assert_eq!(
            schema.get_node_property("Person", "name").unwrap().name,
            "name"
        );
    }

    #[test]
    fn test_schema_get_relationship_property() {
        let schema = create_test_schema_valid();
        assert_eq!(
            schema
                .get_relationship_property("KNOWS", "since")
                .unwrap()
                .name,
            "since"
        );
    }

    #[test]
    fn test_round_trip_serialization() {
        let schema = create_test_schema_valid();
        let json = schema.to_json_file("test_schema.json");
        assert!(json.is_ok());
        let schema = DbSchema::from_json_file("test_schema.json");
        assert!(schema.is_ok());
        let schema = schema.unwrap();
        assert_eq!(schema, create_test_schema_valid());
        remove_file("test_schema.json").unwrap();
    }
}
