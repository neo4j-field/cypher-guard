#![allow(deprecated)]

use ::cypher_guard::{
    get_cypher_validation_errors, parse_query as parse_query_rust, CypherGuardError,
    CypherGuardParsingError, CypherGuardSchemaError, CypherGuardValidationError,
    DbSchema as CoreDbSchema, DbSchemaMetadata as CoreDbSchemaMetadata,
    DbSchemaProperty as CoreDbSchemaProperty,
    DbSchemaRelationshipPattern as CoreDbSchemaRelationshipPattern,
    DbSchemaConstraint as CoreDbSchemaConstraint, DbSchemaIndex as CoreDbSchemaIndex,
    PropertyType as CorePropertyType,
};
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyAny;

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

// Parsing-specific exceptions
create_exception!(cypher_guard, CypherParsingError, PyException);
create_exception!(cypher_guard, NomParsingError, CypherParsingError);
create_exception!(cypher_guard, UnexpectedEndOfInput, CypherParsingError);
create_exception!(cypher_guard, ExpectedToken, CypherParsingError);
create_exception!(cypher_guard, InvalidSyntax, CypherParsingError);
create_exception!(cypher_guard, ParsingUndefinedVariable, CypherParsingError);
create_exception!(cypher_guard, MissingRequiredClause, CypherParsingError);
create_exception!(cypher_guard, InvalidClauseOrder, CypherParsingError);
create_exception!(cypher_guard, ReturnBeforeOtherClauses, CypherParsingError);
create_exception!(cypher_guard, MatchAfterReturn, CypherParsingError);
create_exception!(cypher_guard, CreateAfterReturn, CypherParsingError);
create_exception!(cypher_guard, MergeAfterReturn, CypherParsingError);
create_exception!(cypher_guard, DeleteAfterReturn, CypherParsingError);
create_exception!(cypher_guard, SetAfterReturn, CypherParsingError);
create_exception!(cypher_guard, WhereAfterReturn, CypherParsingError);
create_exception!(cypher_guard, WithAfterReturn, CypherParsingError);
create_exception!(cypher_guard, UnwindAfterReturn, CypherParsingError);
create_exception!(cypher_guard, WhereBeforeMatch, CypherParsingError);
create_exception!(cypher_guard, ReturnAfterReturn, CypherParsingError);
create_exception!(cypher_guard, OrderByBeforeReturn, CypherParsingError);
create_exception!(cypher_guard, SkipBeforeReturn, CypherParsingError);
create_exception!(cypher_guard, LimitBeforeReturn, CypherParsingError);
create_exception!(cypher_guard, InvalidPattern, CypherParsingError);
create_exception!(cypher_guard, InvalidWhereCondition, CypherParsingError);
create_exception!(cypher_guard, InvalidExpression, CypherParsingError);

// Schema-specific exceptions
create_exception!(cypher_guard, CypherSchemaError, PyException);
create_exception!(cypher_guard, InvalidSchemaFormat, CypherSchemaError);
create_exception!(cypher_guard, MissingSchemaField, CypherSchemaError);
create_exception!(cypher_guard, InvalidSchemaPropertyType, CypherSchemaError);
create_exception!(cypher_guard, DuplicateSchemaDefinition, CypherSchemaError);
create_exception!(cypher_guard, InvalidSchemaPropertyName, CypherSchemaError);
create_exception!(
    cypher_guard,
    InvalidSchemaRelationshipPattern,
    CypherSchemaError
);
create_exception!(cypher_guard, InvalidSchemaConstraint, CypherSchemaError);
create_exception!(cypher_guard, InvalidSchemaIndex, CypherSchemaError);
create_exception!(cypher_guard, InvalidSchemaMetadata, CypherSchemaError);
create_exception!(cypher_guard, InvalidSchemaEnumValues, CypherSchemaError);
create_exception!(cypher_guard, InvalidSchemaValueRange, CypherSchemaError);
create_exception!(
    cypher_guard,
    InvalidSchemaDistinctValueCount,
    CypherSchemaError
);
create_exception!(cypher_guard, InvalidSchemaExampleValues, CypherSchemaError);
create_exception!(cypher_guard, InvalidSchemaJson, CypherSchemaError);
create_exception!(cypher_guard, SchemaIoError, CypherSchemaError);
create_exception!(cypher_guard, SchemaLabelNotFound, CypherSchemaError);
create_exception!(cypher_guard, DuplicateSchemaLabel, CypherSchemaError);
create_exception!(cypher_guard, SchemaRelationshipNotFound, CypherSchemaError);
create_exception!(cypher_guard, DuplicateSchemaRelationship, CypherSchemaError);
create_exception!(cypher_guard, SchemaPropertyNotFound, CypherSchemaError);
create_exception!(cypher_guard, DuplicateSchemaProperty, CypherSchemaError);
create_exception!(cypher_guard, SchemaFileOpenError, CypherSchemaError);
create_exception!(cypher_guard, SchemaFileCreateError, CypherSchemaError);
create_exception!(cypher_guard, SchemaJsonReadError, CypherSchemaError);
create_exception!(cypher_guard, SchemaSerializationError, CypherSchemaError);

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
    match err {
        CypherGuardParsingError::Nom(nom_err) => {
            NomParsingError::new_err(format!("Nom parsing error: {}", nom_err))
        }
        CypherGuardParsingError::UnexpectedEnd => {
            UnexpectedEndOfInput::new_err("Unexpected end of input")
        }
        CypherGuardParsingError::ExpectedToken { expected, found } => {
            ExpectedToken::new_err(format!("Expected {}, found {}", expected, found))
        }
        CypherGuardParsingError::InvalidSyntax(msg) => {
            InvalidSyntax::new_err(format!("Invalid syntax: {}", msg))
        }
        CypherGuardParsingError::UndefinedVariable(var) => {
            ParsingUndefinedVariable::new_err(format!("Undefined variable: {}", var))
        }
        CypherGuardParsingError::MissingRequiredClause { clause } => {
            MissingRequiredClause::new_err(format!("Missing required clause: {}", clause))
        }
        CypherGuardParsingError::InvalidClauseOrder { context, details } => {
            InvalidClauseOrder::new_err(format!("Invalid clause order: {} - {}", context, details))
        }
        CypherGuardParsingError::ReturnBeforeOtherClauses { line, column } => {
            ReturnBeforeOtherClauses::new_err(format!(
                "RETURN clause must come after all other clauses except ORDER BY, SKIP, LIMIT, and writing clauses (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::MatchAfterReturn { line, column } => {
            MatchAfterReturn::new_err(format!(
                "MATCH clause cannot come after RETURN clause (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::CreateAfterReturn { line, column } => {
            CreateAfterReturn::new_err(format!(
                "CREATE clause cannot come after RETURN clause (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::MergeAfterReturn { line, column } => {
            MergeAfterReturn::new_err(format!(
                "MERGE clause cannot come after RETURN clause (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::DeleteAfterReturn { line, column } => {
            DeleteAfterReturn::new_err(format!(
                "DELETE clause cannot come after RETURN clause (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::SetAfterReturn { line, column } => {
            SetAfterReturn::new_err(format!(
                "SET clause cannot come after RETURN clause (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::WhereAfterReturn { line, column } => {
            WhereAfterReturn::new_err(format!(
                "WHERE clause cannot come after RETURN clause (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::WithAfterReturn { line, column } => {
            WithAfterReturn::new_err(format!(
                "WITH clause cannot come after RETURN clause (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::UnwindAfterReturn { line, column } => {
            UnwindAfterReturn::new_err(format!(
                "UNWIND clause cannot come after RETURN clause (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::WhereBeforeMatch { line, column } => {
            WhereBeforeMatch::new_err(format!(
                "WHERE clause must come after MATCH, UNWIND, or WITH clause (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::ReturnAfterReturn { line, column } => {
            ReturnAfterReturn::new_err(format!(
                "RETURN clause cannot appear after another RETURN clause (found at line {}, column {})",
                line, column
            ))
        }
        CypherGuardParsingError::OrderByBeforeReturn => {
            OrderByBeforeReturn::new_err("ORDER BY clause must come after RETURN or WITH clause")
        }
        CypherGuardParsingError::SkipBeforeReturn => {
            SkipBeforeReturn::new_err("SKIP clause must come after RETURN, WITH, or ORDER BY clause")
        }
        CypherGuardParsingError::LimitBeforeReturn => {
            LimitBeforeReturn::new_err("LIMIT clause must come after RETURN, WITH, ORDER BY, or SKIP clause")
        }
        CypherGuardParsingError::InvalidPattern { context, details } => {
            InvalidPattern::new_err(format!("Invalid pattern: {} - {}", context, details))
        }
        CypherGuardParsingError::InvalidWhereCondition { context, details } => {
            InvalidWhereCondition::new_err(format!("Invalid WHERE condition: {} - {}", context, details))
        }
        CypherGuardParsingError::InvalidExpression { context, details } => {
            InvalidExpression::new_err(format!("Invalid expression: {} - {}", context, details))
        }
    }
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
        CypherGuardValidationError::UndefinedVariable(var) => {
            UndefinedVariable::new_err(format!("Undefined variable: {}", var))
        }
    }
}

fn convert_schema_error(_py: Python, err: CypherGuardSchemaError) -> PyErr {
    match err {
        CypherGuardSchemaError::InvalidFormat(msg) => {
            InvalidSchemaFormat::new_err(format!("Invalid schema format: {}", msg))
        }
        CypherGuardSchemaError::MissingField(field) => {
            MissingSchemaField::new_err(format!("Missing required field: {}", field))
        }
        CypherGuardSchemaError::InvalidPropertyType(prop_type) => {
            InvalidSchemaPropertyType::new_err(format!("Invalid property type: {}", prop_type))
        }
        CypherGuardSchemaError::DuplicateDefinition(def) => {
            DuplicateSchemaDefinition::new_err(format!("Duplicate definition: {}", def))
        }
        CypherGuardSchemaError::InvalidPropertyName(name) => {
            InvalidSchemaPropertyName::new_err(format!("Invalid property name: {}", name))
        }
        CypherGuardSchemaError::InvalidRelationshipPattern(pattern) => {
            InvalidSchemaRelationshipPattern::new_err(format!(
                "Invalid relationship pattern: {}",
                pattern
            ))
        }
        CypherGuardSchemaError::InvalidConstraint(constraint) => {
            InvalidSchemaConstraint::new_err(format!("Invalid constraint: {}", constraint))
        }
        CypherGuardSchemaError::InvalidIndex(index) => {
            InvalidSchemaIndex::new_err(format!("Invalid index: {}", index))
        }
        CypherGuardSchemaError::InvalidMetadata(metadata) => {
            InvalidSchemaMetadata::new_err(format!("Invalid metadata: {}", metadata))
        }
        CypherGuardSchemaError::InvalidEnumValues(enum_vals) => {
            InvalidSchemaEnumValues::new_err(format!("Invalid enum values: {}", enum_vals))
        }
        CypherGuardSchemaError::InvalidValueRange { min, max } => {
            InvalidSchemaValueRange::new_err(format!(
                "Invalid value range: min {} is greater than max {}",
                min, max
            ))
        }
        CypherGuardSchemaError::InvalidDistinctValueCount(count) => {
            InvalidSchemaDistinctValueCount::new_err(format!(
                "Invalid distinct value count: {}",
                count
            ))
        }
        CypherGuardSchemaError::InvalidExampleValues(examples) => {
            InvalidSchemaExampleValues::new_err(format!("Invalid example values: {}", examples))
        }
        CypherGuardSchemaError::InvalidJson(json_err) => {
            InvalidSchemaJson::new_err(format!("Invalid JSON format: {}", json_err))
        }
        CypherGuardSchemaError::IoError(io_err) => {
            SchemaIoError::new_err(format!("IO error: {}", io_err))
        }
        CypherGuardSchemaError::LabelNotFound(label) => {
            SchemaLabelNotFound::new_err(format!("Label not found: {}", label))
        }
        CypherGuardSchemaError::DuplicateLabel(label) => {
            DuplicateSchemaLabel::new_err(format!("Duplicate label: {}", label))
        }
        CypherGuardSchemaError::RelationshipNotFound(rel) => {
            SchemaRelationshipNotFound::new_err(format!("Relationship not found: {}", rel))
        }
        CypherGuardSchemaError::DuplicateRelationship(rel) => {
            DuplicateSchemaRelationship::new_err(format!("Duplicate relationship: {}", rel))
        }
        CypherGuardSchemaError::PropertyNotFound(prop) => {
            SchemaPropertyNotFound::new_err(format!("Property not found: {}", prop))
        }
        CypherGuardSchemaError::DuplicateProperty(prop) => {
            DuplicateSchemaProperty::new_err(format!("Duplicate property: {}", prop))
        }
        CypherGuardSchemaError::FileOpenError(file_err) => {
            SchemaFileOpenError::new_err(format!("File open error: {}", file_err))
        }
        CypherGuardSchemaError::FileCreateError(file_err) => {
            SchemaFileCreateError::new_err(format!("File create error: {}", file_err))
        }
        CypherGuardSchemaError::JsonReadError(json_err) => {
            SchemaJsonReadError::new_err(format!("JSON read error: {}", json_err))
        }
        CypherGuardSchemaError::SerializationError(ser_err) => {
            SchemaSerializationError::new_err(format!("Serialization error: {}", ser_err))
        }
    }
}

// === Python Wrapper Types ===

/// Internal PropertyType enum (not exposed to Python)
/// Valid values: "STRING", "INTEGER", "FLOAT", "BOOLEAN", "POINT", "DATE_TIME", "LIST"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum PropertyType {
    STRING,
    INTEGER,
    FLOAT,
    BOOLEAN,
    POINT,
    DATE_TIME,
    LIST,
}


impl PropertyType {
    pub fn to_core(&self) -> CorePropertyType {
        match self {
            PropertyType::STRING => CorePropertyType::STRING,
            PropertyType::INTEGER => CorePropertyType::INTEGER,
            PropertyType::FLOAT => CorePropertyType::FLOAT,
            PropertyType::BOOLEAN => CorePropertyType::BOOLEAN,
            PropertyType::POINT => CorePropertyType::POINT,
            PropertyType::DATE_TIME => CorePropertyType::DATE_TIME,
            PropertyType::LIST => CorePropertyType::LIST,
        }
    }

    pub fn from_core(core_type: &CorePropertyType) -> Self {
        match core_type {
            CorePropertyType::STRING => PropertyType::STRING,
            CorePropertyType::INTEGER => PropertyType::INTEGER,
            CorePropertyType::FLOAT => PropertyType::FLOAT,
            CorePropertyType::BOOLEAN => PropertyType::BOOLEAN,
            CorePropertyType::POINT => PropertyType::POINT,
            CorePropertyType::DATE_TIME => PropertyType::DATE_TIME,
            CorePropertyType::LIST => PropertyType::LIST,
        }
    }

    pub fn from_string(s: &str) -> PyResult<Self> {
        match s.trim().to_uppercase().as_str() {
            "STRING" | "STR" => Ok(PropertyType::STRING),
            "INTEGER" | "INT" => Ok(PropertyType::INTEGER),
            "FLOAT" => Ok(PropertyType::FLOAT),
            "BOOLEAN" | "BOOL" => Ok(PropertyType::BOOLEAN),
            "POINT" => Ok(PropertyType::POINT),
            "DATE_TIME" => Ok(PropertyType::DATE_TIME),
            "LIST" => Ok(PropertyType::LIST),
            _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Invalid property type: '{}'. Valid types: STRING, INTEGER, FLOAT, BOOLEAN, POINT, DATE_TIME, LIST",
                s
            ))),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            PropertyType::STRING => "STRING".to_string(),
            PropertyType::INTEGER => "INTEGER".to_string(),
            PropertyType::FLOAT => "FLOAT".to_string(),
            PropertyType::BOOLEAN => "BOOLEAN".to_string(),
            PropertyType::POINT => "POINT".to_string(),
            PropertyType::DATE_TIME => "DATE_TIME".to_string(),
            PropertyType::LIST => "LIST".to_string(),
        }
    }

    pub fn py_from_string(s: &str) -> PyResult<Self> {
        Self::from_string(s)
    }

    pub fn __str__(&self) -> String {
        self.to_string()
    }
}


/// Python wrapper for DbSchemaProperty
#[pyclass]
#[derive(Debug, Clone)]
pub struct DbSchemaProperty {

    inner: CoreDbSchemaProperty,
}

#[pymethods]
impl DbSchemaProperty {
    /// Create a new DbSchemaProperty.
    ///
    /// Args:
    ///     name (str): The property name
    ///     neo4j_type (str): The property type. Must be one of: "STRING", "INTEGER", "FLOAT", "BOOLEAN", "POINT", "DATE_TIME", "LIST"
    ///     enum_values (Optional[List[str]]): List of allowed enum values
    ///     min_value (Optional[float]): Minimum value for numeric types
    ///     max_value (Optional[float]): Maximum value for numeric types
    ///     distinct_value_count (Optional[int]): Number of distinct values
    ///     example_values (Optional[List[str]]): Example values
    #[new]
    #[pyo3(signature = (name, neo4j_type, enum_values=None, min_value=None, max_value=None, distinct_value_count=None, example_values=None))]
    fn new(
        name: String,
        neo4j_type: String,
        enum_values: Option<Vec<String>>,
        min_value: Option<f64>,
        max_value: Option<f64>,
        distinct_value_count: Option<i64>,
        example_values: Option<Vec<String>>,
    ) -> PyResult<Self> {
        // Validate the neo4j_type string and convert to internal enum
        let property_type_enum = PropertyType::from_string(&neo4j_type)?;

        let inner = CoreDbSchemaProperty {
            name: name.clone(),
            neo4j_type: property_type_enum.to_core(),
            enum_values: enum_values.clone(),
            min_value,
            max_value,
            distinct_value_count,
            example_values: example_values.clone(),
        };

        Ok(Self {
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

        let property_type_enum = PropertyType::from_string(&neo4j_type)?;

        // Extract optional fields with alternative field names support
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
                        && value
                            .len()
                            .is_ok_and(|len| len == distinct_value_count.unwrap_or(0) as usize) =>
                {
                    Some(value.extract::<Vec<String>>()?)
                }
                _ => None,
            },
        };

        // Helper function to extract float from string or number
        let extract_float_value = |value: &Bound<'_, pyo3::types::PyAny>| -> Option<f64> {
            if let Ok(num) = value.extract::<f64>() {
                Some(num)
            } else if let Ok(s) = value.extract::<String>() {
                s.parse::<f64>().ok()
            } else {
                None
            }
        };

        // Only set min and max values if the property type is INTEGER or FLOAT
        let mut min_value: Option<f64> = None;
        let mut max_value: Option<f64> = None;
        if neo4j_type == "INTEGER" || neo4j_type == "FLOAT" {
            min_value = match dict.get_item("min_value")? {
                Some(value) if !value.is_none() => extract_float_value(&value),
                _ => match dict.get_item("min")? {
                    Some(value) if !value.is_none() => extract_float_value(&value),
                    _ => None,
                },
            };

            max_value = match dict.get_item("max_value")? {
                Some(value) if !value.is_none() => extract_float_value(&value),
                _ => match dict.get_item("max")? {
                    Some(value) if !value.is_none() => extract_float_value(&value),
                    _ => None,
                },
            };
        }

        let example_values = match dict.get_item("example_values")? {
            Some(value) if !value.is_none() => Some(value.extract::<Vec<String>>()?),
            _ => match dict.get_item("values")? {
                Some(value) if !value.is_none() => Some(value.extract::<Vec<String>>()?),
                _ => None,
            },
        };

        let inner = CoreDbSchemaProperty {
            name: name.clone(),
            neo4j_type: property_type_enum.to_core(),
            enum_values: enum_values.clone(),
            min_value,
            max_value,
            distinct_value_count,
            example_values: example_values.clone(),
        };

        Ok(Self {
            inner,
        })
    }

    // Getters that reference inner values
    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[getter]
    fn neo4j_type(&self) -> String {
        PropertyType::from_core(&self.inner.neo4j_type).to_string()
    }

    #[getter]
    fn enum_values(&self) -> Option<Vec<String>> {
        self.inner.enum_values.clone()
    }

    #[getter]
    fn min_value(&self) -> Option<f64> {
        self.inner.min_value
    }

    #[getter]
    fn max_value(&self) -> Option<f64> {
        self.inner.max_value
    }

    #[getter]
    fn distinct_value_count(&self) -> Option<i64> {
        self.inner.distinct_value_count
    }

    #[getter]
    fn example_values(&self) -> Option<Vec<String>> {
        self.inner.example_values.clone()
    }

    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("name", &self.inner.name)?;
        dict.set_item("neo4j_type", &PropertyType::from_core(&self.inner.neo4j_type).to_string())?;
        if let Some(ref enum_values) = self.inner.enum_values {
            dict.set_item("enum_values", enum_values)?;
        }
        if let Some(min_value) = self.inner.min_value {
            dict.set_item("min_value", min_value)?;
        }
        if let Some(max_value) = self.inner.max_value {
            dict.set_item("max_value", max_value)?;
        }
        if let Some(distinct_value_count) = self.inner.distinct_value_count {
            dict.set_item("distinct_value_count", distinct_value_count)?;
        }
        if let Some(ref example_values) = self.inner.example_values {
            dict.set_item("example_values", example_values)?;
        }
        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        let enum_values_str = match &self.inner.enum_values {
            Some(values) => format!("[{}]", values.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ")),
            None => "None".to_string(),
        };

        let min_value_str = match self.inner.min_value {
            Some(val) => val.to_string(),
            None => "None".to_string(),
        };

        let max_value_str = match self.inner.max_value {
            Some(val) => val.to_string(),
            None => "None".to_string(),
        };

        let distinct_value_count_str = match self.inner.distinct_value_count {
            Some(val) => val.to_string(),
            None => "None".to_string(),
        };

        let example_values_str = match &self.inner.example_values {
            Some(values) => format!("[{}]", values.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ")),
            None => "None".to_string(),
        };

        format!(
            "DbSchemaProperty(name={}, neo4j_type={}, enum_values={}, min_value={}, max_value={}, distinct_value_count={}, example_values={})",
            self.inner.name,
            PropertyType::from_core(&self.inner.neo4j_type).to_string(),
            enum_values_str,
            min_value_str,
            max_value_str,
            distinct_value_count_str,
            example_values_str
        )
    }

    fn __str__(&self) -> String {
        format!("{}: {}", self.inner.name, PropertyType::from_core(&self.inner.neo4j_type).to_string())
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
        let dict = pyo3::types::PyDict::new(py);
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

/// Python wrapper for DbSchemaConstraint
#[pyclass]
#[derive(Debug, Clone)]
pub struct DbSchemaConstraint {
    #[pyo3(get)]
    pub id: i64,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub constraint_type: String,
    #[pyo3(get)]
    pub entity_type: String,
    #[pyo3(get)]
    pub labels_or_types: Vec<String>,
    #[pyo3(get)]
    pub properties: Vec<String>,
    #[pyo3(get)]
    pub owned_index: String,
    #[pyo3(get)]
    pub property_type: Option<String>,
    inner: CoreDbSchemaConstraint,
}

#[pymethods]
impl DbSchemaConstraint {
    #[new]
    #[pyo3(signature = (id, name, constraint_type, entity_type, labels_or_types, properties, owned_index=None, property_type=None))]
    fn new(
        id: i64,
        name: String,
        constraint_type: String,
        entity_type: String,
        labels_or_types: Vec<String>,
        properties: Vec<String>,
        owned_index: Option<String>,
        property_type: Option<String>,
    ) -> Self {
        let inner = CoreDbSchemaConstraint::new(
            id,
            name.clone(),
            constraint_type.clone(),
            entity_type.clone(),
            labels_or_types.clone(),
            properties.clone(),
        );

        Self {
            id,
            name,
            constraint_type,
            entity_type,
            labels_or_types,
            properties,
            owned_index: owned_index.unwrap_or_default(),
            property_type,
            inner,
        }
    }

    #[classmethod]
    #[pyo3(name = "from_dict")]
    fn py_from_dict(_cls: &Bound<'_, pyo3::types::PyType>, dict: &Bound<'_, pyo3::types::PyDict>) -> PyResult<Self> {
        let id = dict
            .get_item("id")?
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("Missing 'id' field"))?
            .extract::<i64>()?;
        let name = dict
            .get_item("name")?
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("Missing 'name' field"))?
            .extract::<String>()?;
        let constraint_type = match dict.get_item("constraint_type")? {
            Some(value) => value.extract::<String>()?,
            None => match dict.get_item("type")? {
                Some(value) => value.extract::<String>()?,
                None => {
                    return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
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
                    return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                        "Missing 'entity_type' or 'entityType' field",
                    ))
                }
            },
        };
        let labels_or_types = match dict.get_item("labels_or_types")? {
            Some(value) => value.extract::<Vec<String>>()?,
            None => match dict.get_item("labelsOrTypes")? {
                Some(value) => value.extract::<Vec<String>>()?,
                None => match dict.get_item("labels")? {
                    Some(value) => value.extract::<Vec<String>>()?,
                    None => {
                        return Err(PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                            "Missing 'labels_or_types', 'labelsOrTypes', or 'labels' field",
                        ))
                    }
                },
            },
        };

        let properties = dict
            .get_item("properties")?
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("Missing 'properties' field"))?
            .extract::<Vec<String>>()?;
        let owned_index = match dict.get_item("owned_index")? {
            Some(value) => Some(value.extract::<String>()?),
            None => match dict.get_item("ownedIndex")? {
                Some(value) => Some(value.extract::<String>()?),
                None => None,
            },
        };
        let property_type = match dict.get_item("property_type")? {
            Some(value) if !value.is_none() => Some(value.extract::<String>()?),
            _ => match dict.get_item("propertyType")? {
                Some(value) if !value.is_none() => Some(value.extract::<String>()?),
                _ => None,
            },
        };

        Ok(Self::new(
            id,
            name,
            constraint_type,
            entity_type,
            labels_or_types,
            properties,
            owned_index,
            property_type,
        ))
    }

    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("id", self.id)?;
        dict.set_item("name", &self.name)?;
        dict.set_item("constraint_type", &self.constraint_type)?;
        dict.set_item("entity_type", &self.entity_type)?;
        dict.set_item("labels_or_types", &self.labels_or_types)?;
        dict.set_item("properties", &self.properties)?;
        dict.set_item("owned_index", &self.owned_index)?;
        if let Some(property_type) = &self.property_type {
            dict.set_item("property_type", property_type)?;
        }
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

/// Python wrapper for DbSchemaIndex
#[pyclass]
#[derive(Debug, Clone)]
pub struct DbSchemaIndex {
    #[pyo3(get)]
    pub label: String,
    #[pyo3(get)]
    pub properties: Vec<String>,
    #[pyo3(get)]
    pub size: i64,
    #[pyo3(get)]
    pub index_type: String,
    #[pyo3(get)]
    pub values_selectivity: f64,
    #[pyo3(get)]
    pub distinct_values: f64,
    inner: CoreDbSchemaIndex,
}

#[pymethods]
impl DbSchemaIndex {
    #[new]
    #[pyo3(signature = (label, properties, size, index_type, values_selectivity=0.0, distinct_values=0.0))]
    fn new(
        label: String,
        properties: Vec<String>,
        size: i64,
        index_type: String,
        values_selectivity: f64,
        distinct_values: f64,
    ) -> Self {
        let inner = CoreDbSchemaIndex::new(
            label.clone(),
            properties.clone(),
            size,
            index_type.clone(),
        );

        Self {
            label,
            properties,
            size,
            index_type,
            values_selectivity,
            distinct_values,
            inner,
        }
    }

    #[classmethod]
    #[pyo3(name = "from_dict")]
    fn py_from_dict(_cls: &Bound<'_, pyo3::types::PyType>, dict: &Bound<'_, pyo3::types::PyDict>) -> PyResult<Self> {
        let label = dict
            .get_item("label")?
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("Missing 'label' field"))?
            .extract::<String>()?;
        let properties = dict
            .get_item("properties")?
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("Missing 'properties' field"))?
            .extract::<Vec<String>>()?;
        let size = dict
            .get_item("size")?
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyKeyError, _>("Missing 'size' field"))?
            .extract::<i64>()?;
        let index_type = match dict.get_item("index_type")? {
            Some(value) => value.extract::<String>()?,
            None => dict
                .get_item("type")?
                .ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyKeyError, _>("Missing 'index_type' or 'type' field")
                })?
                .extract::<String>()?,
        };
        let values_selectivity = match dict.get_item("values_selectivity")? {
            Some(value) => value.extract::<f64>()?,
            None => match dict.get_item("valuesSelectivity")? {
                Some(value) => value.extract::<f64>()?,
                None => 0.0,
            },
        };
        let distinct_values = match dict.get_item("distinct_values")? {
            Some(value) => value.extract::<f64>()?,
            None => match dict.get_item("distinctValues")? {
                Some(value) => value.extract::<f64>()?,
                None => 0.0,
            },
        };

        Ok(Self::new(
            label,
            properties,
            size,
            index_type,
            values_selectivity,
            distinct_values,
        ))
    }

    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("label", &self.label)?;
        dict.set_item("properties", &self.properties)?;
        dict.set_item("size", self.size)?;
        dict.set_item("index_type", &self.index_type)?;
        dict.set_item("values_selectivity", self.values_selectivity)?;
        dict.set_item("distinct_values", self.distinct_values)?;
        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!("DbSchemaIndex(label={}, properties=[{}], size={}, index_type={}, values_selectivity={}, distinct_values={})",
            self.label,
            self.properties.join(", "),
            self.size,
            self.index_type,
            self.values_selectivity,
            self.distinct_values
        )
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

/// Python wrapper for DbSchemaMetadata
#[pyclass]
#[derive(Debug, Clone)]
pub struct DbSchemaMetadata {
    #[pyo3(get)]
    pub constraint: Vec<DbSchemaConstraint>,
    #[pyo3(get)]
    pub index: Vec<DbSchemaIndex>,
    inner: CoreDbSchemaMetadata,
}

#[pymethods]
impl DbSchemaMetadata {
    #[new]
    fn new(constraint: Option<Vec<DbSchemaConstraint>>, index: Option<Vec<DbSchemaIndex>>) -> Self {
        let constraint = constraint.unwrap_or_default();
        let index = index.unwrap_or_default();

        let inner = CoreDbSchemaMetadata::new();
        Self {
            constraint,
            index,
            inner,
        }
    }

    #[classmethod]
    #[pyo3(name = "from_dict")]
    fn py_from_dict(_cls: &Bound<'_, pyo3::types::PyType>, dict: &Bound<'_, pyo3::types::PyDict>) -> PyResult<Self> {
        let constraint = match dict.get_item("constraint")? {
            Some(items) => {
                let iter = items.try_iter()?;
                let mut constraints = Vec::new();
                for item in iter {
                    let constraint_item = item?;
                    if let Ok(constraint_dict) = constraint_item.downcast::<pyo3::types::PyDict>() {
                        constraints.push(DbSchemaConstraint::py_from_dict(_cls, constraint_dict)?);
                    } else {
                        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                            "constraint item is not a dictionary",
                        ));
                    }
                }
                constraints
            }
            None => Vec::new(),
        };

        let index = match dict.get_item("index")? {
            Some(items) => {
                let iter = items.try_iter()?;
                let mut indexes = Vec::new();
                for item in iter {
                    let index_item = item?;
                    if let Ok(index_dict) = index_item.downcast::<pyo3::types::PyDict>() {
                        indexes.push(DbSchemaIndex::py_from_dict(_cls, index_dict)?);
                    } else {
                        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                            "index item is not a dictionary",
                        ));
                    }
                }
                indexes
            }
            None => Vec::new(),
        };

        Ok(Self::new(Some(constraint), Some(index)))
    }

    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);

        let constraint_list = pyo3::types::PyList::empty(py);
        for constraint in &self.constraint {
            constraint_list.append(constraint.py_to_dict(py)?)?;
        }
        dict.set_item("constraint", constraint_list)?;

        let index_list = pyo3::types::PyList::empty(py);
        for index in &self.index {
            index_list.append(index.py_to_dict(py)?)?;
        }
        dict.set_item("index", index_list)?;

        Ok(dict.into())
    }

    fn __repr__(&self) -> String {
        format!(
            "DbSchemaMetadata(constraint=[{}], index=[{}])",
            self.constraint.iter().map(|c| c.__repr__()).collect::<Vec<String>>().join(", "),
            self.index.iter().map(|i| i.__repr__()).collect::<Vec<String>>().join(", ")
        )
    }

    fn __str__(&self) -> String {
        format!(
            "Constraints:\n{}\nIndexes:\n{}",
            self.constraint.iter().map(|c| c.__str__()).collect::<Vec<String>>().join("\n"),
            self.index.iter().map(|i| i.__str__()).collect::<Vec<String>>().join("\n")
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
    pub rel_props: std::collections::HashMap<String, Vec<DbSchemaProperty>>,
    #[pyo3(get)]
    pub relationships: Vec<DbSchemaRelationshipPattern>,
    #[pyo3(get)]
    pub metadata: DbSchemaMetadata,
    inner: CoreDbSchema,
}

#[pymethods]
impl DbSchema {
    #[new]
    fn new() -> Self {
        let inner = CoreDbSchema::new();
        Self {
            node_props: std::collections::HashMap::new(),
            rel_props: std::collections::HashMap::new(),
            relationships: Vec::new(),
            metadata: DbSchemaMetadata::new(None, None),
            inner,
        }
    }

    #[classmethod]
    #[pyo3(name = "from_json_string")]
    fn py_from_json_string(
        _cls: &Bound<'_, pyo3::types::PyType>,
        py: Python,
        json_str: &str,
    ) -> PyResult<Self> {
        let inner = CoreDbSchema::from_json_string(json_str).map_err(|e| match e {
            CypherGuardError::Schema(schema_err) => convert_schema_error(py, schema_err),
            other => convert_cypher_error(py, other),
        })?;

        // Convert core node_props to Python wrapper node_props
        let node_props = inner
            .node_props
            .iter()
            .map(|(label, core_properties)| {
                let properties = core_properties
                    .iter()
                    .map(|core_prop| DbSchemaProperty {
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

        // Convert core rel_props to wrapper rel_props
        let rel_props = inner
            .rel_props
            .iter()
            .map(|(rel_type, core_properties)| {
                let properties = core_properties
                    .iter()
                    .map(|core_prop| DbSchemaProperty {
                        inner: core_prop.clone(),
                    })
                    .collect();
                (rel_type.clone(), properties)
            })
            .collect();

        // Convert core metadata to wrapper metadata (currently empty but properly typed)
        let metadata = DbSchemaMetadata::new(None, None);

        Ok(Self {
            node_props,
            rel_props,
            relationships,
            metadata,
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

        // Parse rel_props
        let rel_props = core_schema
            .rel_props
            .iter()
            .map(|(rel_type, core_properties)| {
                let properties = core_properties
                    .iter()
                    .map(|core_prop| DbSchemaProperty {
                        inner: core_prop.clone(),
                    })
                    .collect();
                (rel_type.clone(), properties)
            })
            .collect();

        // Parse metadata from the input dictionary
        let metadata = if let Some(metadata_item) = dict.get_item("metadata")? {
            let metadata_dict = metadata_item.downcast::<pyo3::types::PyDict>()?;
            DbSchemaMetadata::py_from_dict(_cls, metadata_dict)?
        } else {
            DbSchemaMetadata::new(None, None)
        };

        Ok(Self {
            node_props,
            rel_props,
            relationships,
            metadata,
            inner: core_schema,
        })
    }

    #[pyo3(name = "to_dict")]
    fn py_to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = pyo3::types::PyDict::new(py);

        // Convert node_props to dict
        let node_props_dict = pyo3::types::PyDict::new(py);
        for (label, properties) in &self.node_props {
            let props_list = pyo3::types::PyList::empty(py);
            for prop in properties {
                props_list.append(prop.py_to_dict(py)?)?;
            }
            node_props_dict.set_item(label, props_list)?;
        }
        dict.set_item("node_props", node_props_dict)?;

        // Convert relationships to dict
        let rels_list = pyo3::types::PyList::empty(py);
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

// === CORE PYTHON API FUNCTIONS ===

#[pyfunction]
#[pyo3(text_signature = "(query, schema, /)")]
/// Fast validation check - returns True if query is valid, False if it has any errors.
/// Optimized for LLM validation loops where you only need to know if the query is valid.
///
/// Args:
///     query (str): The Cypher query string to validate
///     schema (str | DbSchema): Either a JSON schema string or a DbSchema object
///
/// Returns:
///     bool: True if query is completely valid, False if it has any validation or parsing errors
///
/// Examples:
///     >>> has_valid_cypher("MATCH (p:Person) RETURN p.name", schema_json)
///     True
///     >>> has_valid_cypher("MATCH (p:InvalidLabel) RETURN p.name", schema_json)  
///     False
pub fn has_valid_cypher(py: Python, query: &str, schema: &Bound<'_, PyAny>) -> PyResult<bool> {
    let db_schema = if let Ok(schema_str) = schema.extract::<&str>() {
        DbSchema::py_from_json_string(&py.get_type::<DbSchema>(), py, schema_str)?
    } else if let Ok(schema_obj) = schema.extract::<DbSchema>() {
        schema_obj
    } else {
        return Err(convert_schema_error(
            py,
            CypherGuardSchemaError::invalid_format(
                "schema must be either a JSON string or DbSchema object",
            ),
        ));
    };

    // Fast path - just check if there are any validation errors
    let errors = get_cypher_validation_errors(query, &db_schema.inner);
    Ok(errors.is_empty())
}

/// Check if a Cypher query has valid syntax.
///
/// **Note**: The parser fails fast on the first syntax error encountered.
/// If there are multiple syntax errors, you'll need to fix them iteratively:
/// 1. Call check_syntax() to find the first error
/// 2. Fix that error
/// 3. Call check_syntax() again to find the next error
/// 4. Repeat until no more errors
///
/// Args:
///     query (str): The Cypher query string to check
///
/// Returns:
///     bool: True if the query has valid syntax, False otherwise
///
/// Raises:
///     Various parsing errors: If there's a syntax error (e.g., NomParsingError, UnexpectedEndOfInput, etc.)
///
/// Examples:
///     >>> check_syntax("MATCH (n) RETURN n")
///     True
///     >>> check_syntax("INVALID SYNTAX")
///     False
#[pyfunction]
#[pyo3(text_signature = "(query, /)")]
pub fn check_syntax(py: Python, query: &str) -> PyResult<bool> {
    // Check if the query can be parsed (syntax check)
    // Schema is not needed for syntax checking - only for validation
    match parse_query_rust(query) {
        Ok(_) => {
            // If parsing succeeds, syntax is valid
            Ok(true)
        }
        Err(e) => {
            // If parsing fails, syntax is invalid
            // Convert parsing error to specific error type
            Err(convert_parsing_error(py, e))
        }
    }
}

/// Validate a Cypher query against a schema and return validation errors.
///
/// Args:
///     query (str): The Cypher query string to validate
///     schema (str | DbSchema): Either a JSON schema string or a DbSchema object
///
/// Returns:
///     List[str]: List of validation error messages. Empty list if query is valid.
///
/// Raises:
///     ValueError: If there's a parsing error (syntax error)
///
/// Examples:
///     >>> validate_cypher("MATCH (n:InvalidLabel) RETURN n", schema_json)
///     ['Invalid node label: InvalidLabel']
///     >>> validate_cypher("MATCH (n:Person) RETURN n", schema_json)
///     []
#[pyfunction]
#[pyo3(text_signature = "(query, schema, /)")]
pub fn validate_cypher(
    py: Python,
    query: &str,
    schema: &Bound<'_, PyAny>,
) -> PyResult<Vec<String>> {
    let db_schema = if let Ok(schema_str) = schema.extract::<&str>() {
        // Schema provided as JSON string
        DbSchema::py_from_json_string(&py.get_type::<DbSchema>(), py, schema_str)?
    } else if let Ok(schema_obj) = schema.extract::<DbSchema>() {
        // Schema provided as DbSchema object
        schema_obj
    } else {
        return Err(convert_schema_error(
            py,
            CypherGuardSchemaError::invalid_format(
                "schema must be either a JSON string or DbSchema object",
            ),
        ));
    };

    // First check if the query can be parsed (syntax check)
    match parse_query_rust(query) {
        Ok(_) => {
            // If parsing succeeds, get validation errors
            Ok(get_cypher_validation_errors(query, &db_schema.inner))
        }
        Err(e) => {
            // If parsing fails, raise syntax error
            Err(convert_parsing_error(py, e))
        }
    }
}

/// Check if a Cypher query contains write operations (CREATE, MERGE, DELETE, SET, REMOVE).
///
/// Args:
///     query (str): The Cypher query string to check
///
/// Returns:
///     bool: True if the query contains write operations, False otherwise
///
/// Raises:
///     ValueError: If there's a parsing error (syntax error)
///
/// Examples:
///     >>> is_write("MATCH (n) RETURN n")
///     False
///     >>> is_write("CREATE (n:Person {name: 'Alice'})")
///     True
///     >>> is_write("MATCH (n) SET n.name = 'Bob'")
///     True
#[pyfunction]
#[pyo3(text_signature = "(query, /)")]
pub fn is_write(py: Python, query: &str) -> PyResult<bool> {
    // First check if the query can be parsed (syntax check)
    match parse_query_rust(query) {
        Ok(ast) => {
            // Check AST for write operations
            let has_ast_write_ops = !ast.create_clauses.is_empty()
                || !ast.merge_clauses.is_empty()
                || !ast.call_clauses.is_empty(); // CALL can contain write operations

            // Check for SET operations in MERGE clauses
            let has_set_ops = ast.merge_clauses.iter().any(|merge| {
                merge
                    .on_create
                    .as_ref()
                    .is_some_and(|on_create| !on_create.set_clauses.is_empty())
                    || merge
                        .on_match
                        .as_ref()
                        .is_some_and(|on_match| !on_match.set_clauses.is_empty())
            });

            // For now, we need to fall back to string matching for DELETE/REMOVE
            // since they're not implemented as separate clauses yet
            let query_upper = query.to_uppercase();
            let has_string_write_ops = query_upper.contains("DELETE")
                || query_upper.contains("DETACH DELETE")
                || query_upper.contains("REMOVE");

            Ok(has_ast_write_ops || has_set_ops || has_string_write_ops)
        }
        Err(e) => {
            // If parsing fails, raise syntax error
            Err(convert_parsing_error(py, e))
        }
    }
}

/// Check if a Cypher query is a read-only query (no write operations).
///
/// Args:
///     query (str): The Cypher query string to check
///
/// Returns:
///     bool: True if the query is read-only, False if it contains write operations
///
/// Raises:
///     ValueError: If there's a parsing error (syntax error)
///
/// Examples:
///     >>> is_read("MATCH (n) RETURN n")
///     True
///     >>> is_read("CREATE (n:Person {name: 'Alice'})")
///     False
///     >>> is_read("MATCH (n) SET n.name = 'Bob'")
///     False
#[pyfunction]
#[pyo3(text_signature = "(query, /)")]
pub fn is_read(py: Python, query: &str) -> PyResult<bool> {
    // First check if the query can be parsed (syntax check)
    match parse_query_rust(query) {
        Ok(ast) => {
            // Check AST for write operations
            let has_ast_write_ops = !ast.create_clauses.is_empty()
                || !ast.merge_clauses.is_empty()
                || !ast.call_clauses.is_empty(); // CALL can contain write operations

            // Check for SET operations in MERGE clauses
            let has_set_ops = ast.merge_clauses.iter().any(|merge| {
                merge
                    .on_create
                    .as_ref()
                    .is_some_and(|on_create| !on_create.set_clauses.is_empty())
                    || merge
                        .on_match
                        .as_ref()
                        .is_some_and(|on_match| !on_match.set_clauses.is_empty())
            });

            // For now, we need to fall back to string matching for DELETE/REMOVE
            // since they're not implemented as separate clauses yet
            let query_upper = query.to_uppercase();
            let has_string_write_ops = query_upper.contains("DELETE")
                || query_upper.contains("DETACH DELETE")
                || query_upper.contains("REMOVE");

            // Return True if it's read-only (no write operations)
            Ok(!(has_ast_write_ops || has_set_ops || has_string_write_ops))
        }
        Err(e) => {
            // If parsing fails, raise syntax error
            Err(convert_parsing_error(py, e))
        }
    }
}

/// Check if a Cypher query has any parsing errors (syntax errors).
///
/// Args:
///     query (str): The Cypher query string to check
///
/// Returns:
///     bool: True if the query has parsing errors, False if it parses successfully
///
/// Examples:
///     >>> has_parser_errors("MATCH (n) RETURN n")
///     False
///     >>> has_parser_errors("INVALID SYNTAX")
///     True
///     >>> has_parser_errors("MATCH (n RETURN n")  # Missing closing parenthesis
///     True
#[pyfunction]
#[pyo3(text_signature = "(query, /)")]
pub fn has_parser_errors(query: &str) -> bool {
    // Simply check if parsing fails
    parse_query_rust(query).is_err()
}

#[pymodule]
fn cypher_guard(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add(
        "__doc__",
        "Cypher Guard: Cypher query validation and parsing library.
    
    **Parser Behavior**: The parser fails fast on the first syntax error encountered.
    If there are multiple syntax errors, you'll need to fix them iteratively:
    1. Call check_syntax() to find the first error
    2. Fix that error
    3. Call check_syntax() again to find the next error
    4. Repeat until no more errors
    
    For more information, see: https://github.com/neo4j-contrib/cypher-guard
    ",
    )?;

    m.add_class::<DbSchema>()?;
    m.add_class::<DbSchemaProperty>()?;
    m.add_class::<DbSchemaRelationshipPattern>()?;
    m.add_class::<DbSchemaConstraint>()?;
    m.add_class::<DbSchemaIndex>()?;
    m.add_class::<DbSchemaMetadata>()?;
    m.add_function(wrap_pyfunction!(has_valid_cypher, m)?)?;

    // Core API functions
    m.add_function(wrap_pyfunction!(check_syntax, m)?)?;
    m.add_function(wrap_pyfunction!(validate_cypher, m)?)?;
    m.add_function(wrap_pyfunction!(is_write, m)?)?;
    m.add_function(wrap_pyfunction!(is_read, m)?)?;
    m.add_function(wrap_pyfunction!(has_parser_errors, m)?)?;

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

    // Parsing error classes
    m.add("CypherParsingError", py.get_type::<CypherParsingError>())?;
    m.add("NomParsingError", py.get_type::<NomParsingError>())?;
    m.add(
        "UnexpectedEndOfInput",
        py.get_type::<UnexpectedEndOfInput>(),
    )?;
    m.add("ExpectedToken", py.get_type::<ExpectedToken>())?;
    m.add("InvalidSyntax", py.get_type::<InvalidSyntax>())?;
    m.add(
        "ParsingUndefinedVariable",
        py.get_type::<ParsingUndefinedVariable>(),
    )?;
    m.add(
        "MissingRequiredClause",
        py.get_type::<MissingRequiredClause>(),
    )?;
    m.add("InvalidClauseOrder", py.get_type::<InvalidClauseOrder>())?;
    m.add(
        "ReturnBeforeOtherClauses",
        py.get_type::<ReturnBeforeOtherClauses>(),
    )?;
    m.add("MatchAfterReturn", py.get_type::<MatchAfterReturn>())?;
    m.add("CreateAfterReturn", py.get_type::<CreateAfterReturn>())?;
    m.add("MergeAfterReturn", py.get_type::<MergeAfterReturn>())?;
    m.add("DeleteAfterReturn", py.get_type::<DeleteAfterReturn>())?;
    m.add("SetAfterReturn", py.get_type::<SetAfterReturn>())?;
    m.add("WhereAfterReturn", py.get_type::<WhereAfterReturn>())?;
    m.add("WithAfterReturn", py.get_type::<WithAfterReturn>())?;
    m.add("UnwindAfterReturn", py.get_type::<UnwindAfterReturn>())?;
    m.add("WhereBeforeMatch", py.get_type::<WhereBeforeMatch>())?;
    m.add("ReturnAfterReturn", py.get_type::<ReturnAfterReturn>())?;
    m.add("OrderByBeforeReturn", py.get_type::<OrderByBeforeReturn>())?;
    m.add("SkipBeforeReturn", py.get_type::<SkipBeforeReturn>())?;
    m.add("LimitBeforeReturn", py.get_type::<LimitBeforeReturn>())?;
    m.add("InvalidPattern", py.get_type::<InvalidPattern>())?;
    m.add(
        "InvalidWhereCondition",
        py.get_type::<InvalidWhereCondition>(),
    )?;
    m.add("InvalidExpression", py.get_type::<InvalidExpression>())?;

    // Schema error classes
    m.add("CypherSchemaError", py.get_type::<CypherSchemaError>())?;
    m.add("InvalidSchemaFormat", py.get_type::<InvalidSchemaFormat>())?;
    m.add("MissingSchemaField", py.get_type::<MissingSchemaField>())?;
    m.add(
        "InvalidSchemaPropertyType",
        py.get_type::<InvalidSchemaPropertyType>(),
    )?;
    m.add(
        "DuplicateSchemaDefinition",
        py.get_type::<DuplicateSchemaDefinition>(),
    )?;
    m.add(
        "InvalidSchemaPropertyName",
        py.get_type::<InvalidSchemaPropertyName>(),
    )?;
    m.add(
        "InvalidSchemaRelationshipPattern",
        py.get_type::<InvalidSchemaRelationshipPattern>(),
    )?;
    m.add(
        "InvalidSchemaConstraint",
        py.get_type::<InvalidSchemaConstraint>(),
    )?;
    m.add("InvalidSchemaIndex", py.get_type::<InvalidSchemaIndex>())?;
    m.add(
        "InvalidSchemaMetadata",
        py.get_type::<InvalidSchemaMetadata>(),
    )?;
    m.add(
        "InvalidSchemaEnumValues",
        py.get_type::<InvalidSchemaEnumValues>(),
    )?;
    m.add(
        "InvalidSchemaValueRange",
        py.get_type::<InvalidSchemaValueRange>(),
    )?;
    m.add(
        "InvalidSchemaDistinctValueCount",
        py.get_type::<InvalidSchemaDistinctValueCount>(),
    )?;
    m.add(
        "InvalidSchemaExampleValues",
        py.get_type::<InvalidSchemaExampleValues>(),
    )?;
    m.add("InvalidSchemaJson", py.get_type::<InvalidSchemaJson>())?;
    m.add("SchemaIoError", py.get_type::<SchemaIoError>())?;
    m.add("SchemaLabelNotFound", py.get_type::<SchemaLabelNotFound>())?;
    m.add(
        "DuplicateSchemaLabel",
        py.get_type::<DuplicateSchemaLabel>(),
    )?;
    m.add(
        "SchemaRelationshipNotFound",
        py.get_type::<SchemaRelationshipNotFound>(),
    )?;
    m.add(
        "DuplicateSchemaRelationship",
        py.get_type::<DuplicateSchemaRelationship>(),
    )?;
    m.add(
        "SchemaPropertyNotFound",
        py.get_type::<SchemaPropertyNotFound>(),
    )?;
    m.add(
        "DuplicateSchemaProperty",
        py.get_type::<DuplicateSchemaProperty>(),
    )?;
    m.add("SchemaFileOpenError", py.get_type::<SchemaFileOpenError>())?;
    m.add(
        "SchemaFileCreateError",
        py.get_type::<SchemaFileCreateError>(),
    )?;
    m.add("SchemaJsonReadError", py.get_type::<SchemaJsonReadError>())?;
    m.add(
        "SchemaSerializationError",
        py.get_type::<SchemaSerializationError>(),
    )?;

    Ok(())
}
