use thiserror::Error;

#[derive(Debug, Error)]
pub enum CypherGuardError {
    #[error("Validation error: {0}")]
    Validation(#[from] CypherGuardValidationError),

    #[error("Parsing error: {0}")]
    Parsing(#[from] CypherGuardParsingError),

    #[error("Schema error: {0}")]
    Schema(#[from] CypherGuardSchemaError),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),
}

impl CypherGuardError {
    /// Returns true if this is a validation error
    pub fn is_validation(&self) -> bool {
        matches!(self, Self::Validation(_))
    }

    /// Returns true if this is a parsing error
    pub fn is_parsing(&self) -> bool {
        matches!(self, Self::Parsing(_))
    }

    /// Returns true if this is a schema error
    pub fn is_schema(&self) -> bool {
        matches!(self, Self::Schema(_))
    }

    /// Returns true if this is an invalid query error
    pub fn is_invalid_query(&self) -> bool {
        matches!(self, Self::InvalidQuery(_))
    }

    /// Returns the invalid query message if this is an InvalidQuery error
    pub fn invalid_query_msg(&self) -> Option<&str> {
        match self {
            Self::InvalidQuery(msg) => Some(msg),
            _ => None,
        }
    }
}

#[derive(Debug, Error)]
pub enum CypherGuardValidationError {
    #[error("Invalid property name: {0}")]
    InvalidPropertyName(String),

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Invalid relationship: {0}")]
    InvalidRelationship(String),

    #[error("Invalid label: {0}")]
    InvalidLabel(String),

    #[error("Invalid node label: {0}")]
    InvalidNodeLabel(String),

    #[error("Invalid relationship type: {0}")]
    InvalidRelationshipType(String),

    #[error("Invalid node property '{property}' on label '{label}'")]
    InvalidNodeProperty { label: String, property: String },

    #[error("Invalid relationship property '{property}' on type '{rel_type}'")]
    InvalidRelationshipProperty { rel_type: String, property: String },

    #[error("Invalid property access '{variable}.{property}' in {context} clause")]
    InvalidPropertyAccess {
        variable: String,
        property: String,
        context: String,
    },

    #[error("Invalid property type for '{variable}.{property}': expected {expected_type}, got value '{actual_value}'")]
    InvalidPropertyType {
        variable: String,
        property: String,
        expected_type: String,
        actual_value: String,
    },

    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
}

impl CypherGuardValidationError {
    pub fn invalid_property_name(name: impl Into<String>) -> Self {
        Self::InvalidPropertyName(name.into())
    }

    pub fn type_mismatch(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::TypeMismatch {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    pub fn invalid_relationship(rel: impl Into<String>) -> Self {
        Self::InvalidRelationship(rel.into())
    }

    pub fn invalid_label(label: impl Into<String>) -> Self {
        Self::InvalidLabel(label.into())
    }

    pub fn invalid_node_label(label: impl Into<String>) -> Self {
        Self::InvalidNodeLabel(label.into())
    }

    pub fn invalid_relationship_type(rel_type: impl Into<String>) -> Self {
        Self::InvalidRelationshipType(rel_type.into())
    }

    pub fn invalid_node_property(label: impl Into<String>, property: impl Into<String>) -> Self {
        Self::InvalidNodeProperty {
            label: label.into(),
            property: property.into(),
        }
    }

    pub fn invalid_relationship_property(
        rel_type: impl Into<String>,
        property: impl Into<String>,
    ) -> Self {
        Self::InvalidRelationshipProperty {
            rel_type: rel_type.into(),
            property: property.into(),
        }
    }

    pub fn invalid_property_access(
        variable: impl Into<String>,
        property: impl Into<String>,
        context: impl Into<String>,
    ) -> Self {
        Self::InvalidPropertyAccess {
            variable: variable.into(),
            property: property.into(),
            context: context.into(),
        }
    }

    pub fn invalid_property_type(
        variable: impl Into<String>,
        property: impl Into<String>,
        expected_type: impl Into<String>,
        actual_value: impl Into<String>,
    ) -> Self {
        Self::InvalidPropertyType {
            variable: variable.into(),
            property: property.into(),
            expected_type: expected_type.into(),
            actual_value: actual_value.into(),
        }
    }

    /// Returns the property name if this is an InvalidPropertyName error
    pub fn property_name(&self) -> Option<&str> {
        match self {
            Self::InvalidPropertyName(name) => Some(name),
            _ => None,
        }
    }

    /// Returns the expected and actual types if this is a TypeMismatch error
    pub fn type_mismatch_details(&self) -> Option<(&str, &str)> {
        match self {
            Self::TypeMismatch { expected, actual } => Some((expected, actual)),
            _ => None,
        }
    }

    /// Returns the relationship name if this is an InvalidRelationship error
    pub fn relationship_name(&self) -> Option<&str> {
        match self {
            Self::InvalidRelationship(rel) => Some(rel),
            _ => None,
        }
    }

    /// Returns the label name if this is an InvalidLabel error
    pub fn label_name(&self) -> Option<&str> {
        match self {
            Self::InvalidLabel(label) => Some(label),
            _ => None,
        }
    }

    /// Returns the node label name if this is an InvalidNodeLabel error
    pub fn node_label_name(&self) -> Option<&str> {
        match self {
            Self::InvalidNodeLabel(label) => Some(label),
            _ => None,
        }
    }

    /// Returns the relationship type name if this is an InvalidRelationshipType error
    pub fn relationship_type_name(&self) -> Option<&str> {
        match self {
            Self::InvalidRelationshipType(rel_type) => Some(rel_type),
            _ => None,
        }
    }

    /// Returns the node property details if this is an InvalidNodeProperty error
    pub fn node_property_details(&self) -> Option<(&str, &str)> {
        match self {
            Self::InvalidNodeProperty { label, property } => Some((label, property)),
            _ => None,
        }
    }

    /// Returns the relationship property details if this is an InvalidRelationshipProperty error
    pub fn relationship_property_details(&self) -> Option<(&str, &str)> {
        match self {
            Self::InvalidRelationshipProperty { rel_type, property } => Some((rel_type, property)),
            _ => None,
        }
    }

    /// Returns the property access details if this is an InvalidPropertyAccess error
    pub fn property_access_details(&self) -> Option<(&str, &str, &str)> {
        match self {
            Self::InvalidPropertyAccess {
                variable,
                property,
                context,
            } => Some((variable, property, context)),
            _ => None,
        }
    }
}

#[derive(Debug, Error)]
pub enum CypherGuardParsingError {
    #[error("Nom parsing error: {0}")]
    Nom(#[from] nom::error::Error<String>),

    #[error("Unexpected end of input")]
    UnexpectedEnd,

    #[error("Expected {expected}, found {found}")]
    ExpectedToken { expected: String, found: String },

    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),

    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),

    #[error("Missing required clause: {clause}")]
    MissingRequiredClause { clause: String },

    #[error("Invalid clause order: {context} - {details}")]
    InvalidClauseOrder { context: String, details: String },

    // Specific clause order error variants
    #[error("RETURN clause must come after all other clauses except ORDER BY, SKIP, LIMIT, and writing clauses (found at line {line}, column {column})")]
    ReturnBeforeOtherClauses { line: usize, column: usize },

    #[error(
        "MATCH clause cannot come after RETURN clause (found at line {line}, column {column})"
    )]
    MatchAfterReturn { line: usize, column: usize },

    #[error(
        "CREATE clause cannot come after RETURN clause (found at line {line}, column {column})"
    )]
    CreateAfterReturn { line: usize, column: usize },

    #[error(
        "MERGE clause cannot come after RETURN clause (found at line {line}, column {column})"
    )]
    MergeAfterReturn { line: usize, column: usize },

    #[error(
        "DELETE clause cannot come after RETURN clause (found at line {line}, column {column})"
    )]
    DeleteAfterReturn { line: usize, column: usize },

    #[error("SET clause cannot come after RETURN clause (found at line {line}, column {column})")]
    SetAfterReturn { line: usize, column: usize },

    #[error(
        "WHERE clause cannot come after RETURN clause (found at line {line}, column {column})"
    )]
    WhereAfterReturn { line: usize, column: usize },

    #[error("WITH clause cannot come after RETURN clause (found at line {line}, column {column})")]
    WithAfterReturn { line: usize, column: usize },

    #[error(
        "UNWIND clause cannot come after RETURN clause (found at line {line}, column {column})"
    )]
    UnwindAfterReturn { line: usize, column: usize },

    #[error("WHERE clause must come after MATCH, UNWIND, or WITH clause (found at line {line}, column {column})")]
    WhereBeforeMatch { line: usize, column: usize },

    #[error("RETURN clause cannot appear after another RETURN clause (found at line {line}, column {column})")]
    ReturnAfterReturn { line: usize, column: usize },

    #[error("ORDER BY clause must come after RETURN or WITH clause")]
    OrderByBeforeReturn,

    #[error("SKIP clause must come after RETURN, WITH, or ORDER BY clause")]
    SkipBeforeReturn,

    #[error("LIMIT clause must come after RETURN, WITH, ORDER BY, or SKIP clause")]
    LimitBeforeReturn,

    #[error("Invalid pattern: {context} - {details}")]
    InvalidPattern { context: String, details: String },

    #[error("Invalid WHERE condition: {context} - {details}")]
    InvalidWhereCondition { context: String, details: String },

    #[error("Invalid expression: {context} - {details}")]
    InvalidExpression { context: String, details: String },
}

impl CypherGuardParsingError {
    // Basic parsing errors (most fundamental)
    pub fn expected_token(expected: impl Into<String>, found: impl Into<String>) -> Self {
        Self::ExpectedToken {
            expected: expected.into(),
            found: found.into(),
        }
    }

    pub fn invalid_syntax(msg: impl Into<String>) -> Self {
        Self::InvalidSyntax(msg.into())
    }

    pub fn undefined_variable(var: impl Into<String>) -> Self {
        let var_name = var.into();
        eprintln!("🔥 CREATING UndefinedVariable ERROR for: '{}'", var_name);
        Self::UndefinedVariable(var_name)
    }

    // Query structure errors
    pub fn missing_required_clause(clause: impl Into<String>) -> Self {
        Self::MissingRequiredClause {
            clause: clause.into(),
        }
    }

    pub fn invalid_clause_order(context: impl Into<String>, details: impl Into<String>) -> Self {
        Self::InvalidClauseOrder {
            context: context.into(),
            details: details.into(),
        }
    }

    // Pattern and expression errors (most specific)
    pub fn invalid_pattern(context: impl Into<String>, details: impl Into<String>) -> Self {
        Self::InvalidPattern {
            context: context.into(),
            details: details.into(),
        }
    }

    pub fn invalid_where_condition(context: impl Into<String>, details: impl Into<String>) -> Self {
        Self::InvalidWhereCondition {
            context: context.into(),
            details: details.into(),
        }
    }

    pub fn invalid_expression(context: impl Into<String>, details: impl Into<String>) -> Self {
        Self::InvalidExpression {
            context: context.into(),
            details: details.into(),
        }
    }

    // Specific clause order error helper methods
    pub fn return_before_other_clauses() -> Self {
        Self::ReturnBeforeOtherClauses { line: 0, column: 0 }
    }

    pub fn return_before_other_clauses_at(line: usize, column: usize) -> Self {
        Self::ReturnBeforeOtherClauses { line, column }
    }

    pub fn match_after_return() -> Self {
        Self::MatchAfterReturn { line: 0, column: 0 }
    }

    pub fn match_after_return_at(line: usize, column: usize) -> Self {
        Self::MatchAfterReturn { line, column }
    }

    pub fn create_after_return() -> Self {
        Self::CreateAfterReturn { line: 0, column: 0 }
    }

    pub fn create_after_return_at(line: usize, column: usize) -> Self {
        Self::CreateAfterReturn { line, column }
    }

    pub fn merge_after_return() -> Self {
        Self::MergeAfterReturn { line: 0, column: 0 }
    }

    pub fn merge_after_return_at(line: usize, column: usize) -> Self {
        Self::MergeAfterReturn { line, column }
    }

    pub fn delete_after_return() -> Self {
        Self::DeleteAfterReturn { line: 0, column: 0 }
    }

    pub fn delete_after_return_at(line: usize, column: usize) -> Self {
        Self::DeleteAfterReturn { line, column }
    }

    pub fn set_after_return() -> Self {
        Self::SetAfterReturn { line: 0, column: 0 }
    }

    pub fn set_after_return_at(line: usize, column: usize) -> Self {
        Self::SetAfterReturn { line, column }
    }

    pub fn where_after_return() -> Self {
        Self::WhereAfterReturn { line: 0, column: 0 }
    }

    pub fn where_after_return_at(line: usize, column: usize) -> Self {
        Self::WhereAfterReturn { line, column }
    }

    pub fn with_after_return() -> Self {
        Self::WithAfterReturn { line: 0, column: 0 }
    }

    pub fn with_after_return_at(line: usize, column: usize) -> Self {
        Self::WithAfterReturn { line, column }
    }

    pub fn unwind_after_return() -> Self {
        Self::UnwindAfterReturn { line: 0, column: 0 }
    }

    pub fn unwind_after_return_at(line: usize, column: usize) -> Self {
        Self::UnwindAfterReturn { line, column }
    }

    pub fn where_before_match() -> Self {
        Self::WhereBeforeMatch { line: 0, column: 0 }
    }

    pub fn where_before_match_at(line: usize, column: usize) -> Self {
        Self::WhereBeforeMatch { line, column }
    }

    pub fn return_after_return() -> Self {
        Self::ReturnAfterReturn { line: 0, column: 0 }
    }

    pub fn return_after_return_at(line: usize, column: usize) -> Self {
        Self::ReturnAfterReturn { line, column }
    }

    pub fn order_by_before_return() -> Self {
        Self::OrderByBeforeReturn
    }

    pub fn skip_before_return() -> Self {
        Self::SkipBeforeReturn
    }

    pub fn limit_before_return() -> Self {
        Self::LimitBeforeReturn
    }

    // Query methods (organized by error type)
    /// Returns true if this is a nom parsing error
    pub fn is_nom_error(&self) -> bool {
        matches!(self, Self::Nom { .. })
    }

    /// Returns true if this is an UnexpectedEnd error
    pub fn is_unexpected_end(&self) -> bool {
        matches!(self, Self::UnexpectedEnd)
    }

    /// Returns the expected and found tokens if this is an ExpectedToken error
    pub fn expected_token_details(&self) -> Option<(&str, &str)> {
        match self {
            Self::ExpectedToken { expected, found } => Some((expected, found)),
            _ => None,
        }
    }

    /// Returns the syntax error message if this is an InvalidSyntax error
    pub fn syntax_error(&self) -> Option<&str> {
        match self {
            Self::InvalidSyntax(msg) => Some(msg),
            _ => None,
        }
    }

    // Query structure query methods
    /// Returns the missing clause if this is a MissingRequiredClause error
    pub fn missing_clause(&self) -> Option<&str> {
        match self {
            Self::MissingRequiredClause { clause } => Some(clause),
            _ => None,
        }
    }

    /// Returns the clause order error message if this is an InvalidClauseOrder error
    pub fn clause_order_error(&self) -> Option<String> {
        match self {
            Self::InvalidClauseOrder { context, details } => {
                Some(format!("{} - {}", context, details))
            }
            _ => None,
        }
    }

    // Pattern and expression query methods
    /// Returns the pattern error if this is an InvalidPattern error
    pub fn pattern_error(&self) -> Option<String> {
        match self {
            Self::InvalidPattern { context, details } => Some(format!("{} - {}", context, details)),
            _ => None,
        }
    }

    /// Returns the WHERE condition error if this is an InvalidWhereCondition error
    pub fn where_condition_error(&self) -> Option<String> {
        match self {
            Self::InvalidWhereCondition { context, details } => {
                Some(format!("{} - {}", context, details))
            }
            _ => None,
        }
    }

    /// Returns the expression error if this is an InvalidExpression error
    pub fn expression_error(&self) -> Option<String> {
        match self {
            Self::InvalidExpression { context, details } => {
                Some(format!("{} - {}", context, details))
            }
            _ => None,
        }
    }

    // Specific clause order error query methods
    /// Returns true if this is a ReturnBeforeOtherClauses error
    pub fn is_return_before_other_clauses(&self) -> bool {
        matches!(self, Self::ReturnBeforeOtherClauses { .. })
    }

    /// Returns true if this is a MatchAfterReturn error
    pub fn is_match_after_return(&self) -> bool {
        matches!(self, Self::MatchAfterReturn { .. })
    }

    /// Returns true if this is a CreateAfterReturn error
    pub fn is_create_after_return(&self) -> bool {
        matches!(self, Self::CreateAfterReturn { .. })
    }

    /// Returns true if this is a MergeAfterReturn error
    pub fn is_merge_after_return(&self) -> bool {
        matches!(self, Self::MergeAfterReturn { .. })
    }

    /// Returns true if this is a DeleteAfterReturn error
    pub fn is_delete_after_return(&self) -> bool {
        matches!(self, Self::DeleteAfterReturn { .. })
    }

    /// Returns true if this is a SetAfterReturn error
    pub fn is_set_after_return(&self) -> bool {
        matches!(self, Self::SetAfterReturn { .. })
    }

    /// Returns true if this is a WhereAfterReturn error
    pub fn is_where_after_return(&self) -> bool {
        matches!(self, Self::WhereAfterReturn { .. })
    }

    /// Returns true if this is a WithAfterReturn error
    pub fn is_with_after_return(&self) -> bool {
        matches!(self, Self::WithAfterReturn { .. })
    }

    /// Returns true if this is an UnwindAfterReturn error
    pub fn is_unwind_after_return(&self) -> bool {
        matches!(self, Self::UnwindAfterReturn { .. })
    }

    /// Returns true if this is a WhereBeforeMatch error
    pub fn is_where_before_match(&self) -> bool {
        matches!(self, Self::WhereBeforeMatch { .. })
    }

    /// Returns true if this is a ReturnAfterReturn error
    pub fn is_return_after_return(&self) -> bool {
        matches!(self, Self::ReturnAfterReturn { .. })
    }

    /// Returns true if this is an OrderByBeforeReturn error
    pub fn is_order_by_before_return(&self) -> bool {
        matches!(self, Self::OrderByBeforeReturn)
    }

    /// Returns true if this is a SkipBeforeReturn error
    pub fn is_skip_before_return(&self) -> bool {
        matches!(self, Self::SkipBeforeReturn)
    }

    /// Returns true if this is a LimitBeforeReturn error
    pub fn is_limit_before_return(&self) -> bool {
        matches!(self, Self::LimitBeforeReturn)
    }

    /// Returns true if this is any clause order error
    pub fn is_clause_order_error(&self) -> bool {
        matches!(
            self,
            Self::InvalidClauseOrder { .. }
                | Self::ReturnBeforeOtherClauses { .. }
                | Self::MatchAfterReturn { .. }
                | Self::CreateAfterReturn { .. }
                | Self::MergeAfterReturn { .. }
                | Self::DeleteAfterReturn { .. }
                | Self::SetAfterReturn { .. }
                | Self::WhereAfterReturn { .. }
                | Self::WithAfterReturn { .. }
                | Self::UnwindAfterReturn { .. }
                | Self::WhereBeforeMatch { .. }
                | Self::OrderByBeforeReturn
                | Self::SkipBeforeReturn
                | Self::LimitBeforeReturn
                | Self::ReturnAfterReturn { .. }
        )
    }
}

/// Helper function to convert nom errors to CypherGuardParsingError with context
#[allow(dead_code)]
pub fn convert_nom_error(
    nom_err: nom::Err<nom::error::Error<&str>>,
    _context: &str,
    _input: &str,
) -> CypherGuardParsingError {
    match nom_err {
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            // Convert the input to String to match our error type
            let input_str = e.input.to_string();
            let code = e.code;
            CypherGuardParsingError::Nom(nom::error::Error::new(input_str, code))
        }
        nom::Err::Incomplete(_) => {
            // Incomplete input means unexpected end
            CypherGuardParsingError::UnexpectedEnd
        }
    }
}

#[derive(Debug, Error)]
pub enum CypherGuardSchemaError {
    #[error("Invalid schema format: {0}")]
    InvalidFormat(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid property type: {0}")]
    InvalidPropertyType(String),

    #[error("Duplicate definition: {0}")]
    DuplicateDefinition(String),

    #[error("Invalid property name: {0}")]
    InvalidPropertyName(String),

    #[error("Invalid relationship pattern: {0}")]
    InvalidRelationshipPattern(String),

    #[error("Invalid constraint: {0}")]
    InvalidConstraint(String),

    #[error("Invalid index: {0}")]
    InvalidIndex(String),

    #[error("Invalid metadata: {0}")]
    InvalidMetadata(String),

    #[error("Invalid enum values: {0}")]
    InvalidEnumValues(String),

    #[error("Invalid value range: min {min} is greater than max {max}")]
    InvalidValueRange { min: f64, max: f64 },

    #[error("Invalid distinct value count: {0}")]
    InvalidDistinctValueCount(i64),

    #[error("Invalid example values: {0}")]
    InvalidExampleValues(String),

    #[error("Invalid JSON format: {0}")]
    InvalidJson(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Label not found: {0}")]
    LabelNotFound(String),

    #[error("Duplicate label: {0}")]
    DuplicateLabel(String),

    #[error("Relationship not found: {0}")]
    RelationshipNotFound(String),

    #[error("Duplicate relationship: {0}")]
    DuplicateRelationship(String),

    #[error("Property not found: {0}")]
    PropertyNotFound(String),

    #[error("Duplicate property: {0}")]
    DuplicateProperty(String),

    #[error("File open error: {0}")]
    FileOpenError(String),

    #[error("File create error: {0}")]
    FileCreateError(String),

    #[error("JSON read error: {0}")]
    JsonReadError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl CypherGuardSchemaError {
    pub fn invalid_format(msg: impl Into<String>) -> Self {
        Self::InvalidFormat(msg.into())
    }

    pub fn missing_field(field: impl Into<String>) -> Self {
        Self::MissingField(field.into())
    }

    pub fn invalid_property_type(prop_type: impl Into<String>) -> Self {
        Self::InvalidPropertyType(prop_type.into())
    }

    pub fn duplicate_definition(def: impl Into<String>) -> Self {
        Self::DuplicateDefinition(def.into())
    }

    pub fn invalid_property_name(name: impl Into<String>) -> Self {
        Self::InvalidPropertyName(name.into())
    }

    pub fn invalid_relationship_pattern(pattern: impl Into<String>) -> Self {
        Self::InvalidRelationshipPattern(pattern.into())
    }

    pub fn invalid_constraint(constraint: impl Into<String>) -> Self {
        Self::InvalidConstraint(constraint.into())
    }

    pub fn invalid_index(index: impl Into<String>) -> Self {
        Self::InvalidIndex(index.into())
    }

    pub fn invalid_metadata(metadata: impl Into<String>) -> Self {
        Self::InvalidMetadata(metadata.into())
    }

    pub fn invalid_enum_values(values: impl Into<String>) -> Self {
        Self::InvalidEnumValues(values.into())
    }

    pub fn invalid_value_range(min: f64, max: f64) -> Self {
        Self::InvalidValueRange { min, max }
    }

    pub fn invalid_distinct_value_count(count: i64) -> Self {
        Self::InvalidDistinctValueCount(count)
    }

    pub fn invalid_example_values(values: impl Into<String>) -> Self {
        Self::InvalidExampleValues(values.into())
    }

    pub fn invalid_json(msg: impl Into<String>) -> Self {
        Self::InvalidJson(msg.into())
    }

    pub fn io_error(msg: impl Into<String>) -> Self {
        Self::IoError(msg.into())
    }

    pub fn label_not_found(label: impl Into<String>) -> Self {
        Self::LabelNotFound(label.into())
    }

    pub fn duplicate_label(label: impl Into<String>) -> Self {
        Self::DuplicateLabel(label.into())
    }

    pub fn relationship_not_found(rel_type: impl Into<String>) -> Self {
        Self::RelationshipNotFound(rel_type.into())
    }

    pub fn duplicate_relationship(rel_type: impl Into<String>) -> Self {
        Self::DuplicateRelationship(rel_type.into())
    }

    pub fn property_not_found(name: impl Into<String>) -> Self {
        Self::PropertyNotFound(name.into())
    }

    pub fn duplicate_property(name: impl Into<String>) -> Self {
        Self::DuplicateProperty(name.into())
    }

    pub fn file_open_error(msg: impl Into<String>) -> Self {
        Self::FileOpenError(msg.into())
    }

    pub fn file_create_error(msg: impl Into<String>) -> Self {
        Self::FileCreateError(msg.into())
    }

    pub fn json_read_error(msg: impl Into<String>) -> Self {
        Self::JsonReadError(msg.into())
    }

    pub fn serialization_error(msg: impl Into<String>) -> Self {
        Self::SerializationError(msg.into())
    }

    /// Returns the format error message if this is an InvalidFormat error
    pub fn format_error(&self) -> Option<&str> {
        match self {
            Self::InvalidFormat(msg) => Some(msg),
            _ => None,
        }
    }

    /// Returns the missing field name if this is a MissingField error
    pub fn missing_field_name(&self) -> Option<&str> {
        match self {
            Self::MissingField(field) => Some(field),
            _ => None,
        }
    }

    /// Returns the property type if this is an InvalidPropertyType error
    pub fn property_type(&self) -> Option<&str> {
        match self {
            Self::InvalidPropertyType(prop_type) => Some(prop_type),
            _ => None,
        }
    }

    /// Returns the duplicate definition name if this is a DuplicateDefinition error
    pub fn duplicate_name(&self) -> Option<&str> {
        match self {
            Self::DuplicateDefinition(def) => Some(def),
            _ => None,
        }
    }

    /// Returns the property name if this is an InvalidPropertyName error
    pub fn property_name(&self) -> Option<&str> {
        match self {
            Self::InvalidPropertyName(name) | Self::PropertyNotFound(name) => Some(name),
            _ => None,
        }
    }

    /// Returns the relationship pattern if this is an InvalidRelationshipPattern error
    pub fn relationship_pattern(&self) -> Option<&str> {
        match self {
            Self::InvalidRelationshipPattern(pattern) => Some(pattern),
            _ => None,
        }
    }

    /// Returns the constraint if this is an InvalidConstraint error
    pub fn constraint(&self) -> Option<&str> {
        match self {
            Self::InvalidConstraint(constraint) => Some(constraint),
            _ => None,
        }
    }

    /// Returns the index if this is an InvalidIndex error
    pub fn index(&self) -> Option<&str> {
        match self {
            Self::InvalidIndex(index) => Some(index),
            _ => None,
        }
    }

    /// Returns the metadata if this is an InvalidMetadata error
    pub fn metadata(&self) -> Option<&str> {
        match self {
            Self::InvalidMetadata(metadata) => Some(metadata),
            _ => None,
        }
    }

    /// Returns the enum values if this is an InvalidEnumValues error
    pub fn enum_values(&self) -> Option<&str> {
        match self {
            Self::InvalidEnumValues(values) => Some(values),
            _ => None,
        }
    }

    /// Returns the value range if this is an InvalidValueRange error
    pub fn value_range(&self) -> Option<(f64, f64)> {
        match self {
            Self::InvalidValueRange { min, max } => Some((*min, *max)),
            _ => None,
        }
    }

    /// Returns the distinct value count if this is an InvalidDistinctValueCount error
    pub fn distinct_value_count(&self) -> Option<i64> {
        match self {
            Self::InvalidDistinctValueCount(count) => Some(*count),
            _ => None,
        }
    }

    /// Returns the example values if this is an InvalidExampleValues error
    pub fn example_values(&self) -> Option<&str> {
        match self {
            Self::InvalidExampleValues(values) => Some(values),
            _ => None,
        }
    }

    /// Returns the JSON error message if this is an InvalidJson error
    pub fn json_error(&self) -> Option<&str> {
        match self {
            Self::InvalidJson(msg) => Some(msg),
            _ => None,
        }
    }

    /// Returns the IO error message if this is an IoError error
    pub fn io_error_msg(&self) -> Option<&str> {
        match self {
            Self::IoError(msg) => Some(msg),
            _ => None,
        }
    }

    /// Returns the label name if this is a LabelNotFound error
    pub fn label_name(&self) -> Option<&str> {
        match self {
            Self::LabelNotFound(label) => Some(label),
            _ => None,
        }
    }

    /// Returns the duplicate label name if this is a DuplicateLabel error
    pub fn duplicate_label_name(&self) -> Option<&str> {
        match self {
            Self::DuplicateLabel(label) => Some(label),
            _ => None,
        }
    }

    /// Returns the relationship type if this is a RelationshipNotFound error
    pub fn relationship_type(&self) -> Option<&str> {
        match self {
            Self::RelationshipNotFound(rel_type) => Some(rel_type),
            _ => None,
        }
    }

    /// Returns the duplicate relationship type if this is a DuplicateRelationship error
    pub fn duplicate_relationship_type(&self) -> Option<&str> {
        match self {
            Self::DuplicateRelationship(rel_type) => Some(rel_type),
            _ => None,
        }
    }

    /// Returns the file open error message if this is a FileOpenError
    pub fn file_open_error_msg(&self) -> Option<&str> {
        match self {
            Self::FileOpenError(msg) => Some(msg),
            _ => None,
        }
    }

    /// Returns the file create error message if this is a FileCreateError
    pub fn file_create_error_msg(&self) -> Option<&str> {
        match self {
            Self::FileCreateError(msg) => Some(msg),
            _ => None,
        }
    }

    /// Returns the JSON read error message if this is a JsonReadError
    pub fn json_read_error_msg(&self) -> Option<&str> {
        match self {
            Self::JsonReadError(msg) => Some(msg),
            _ => None,
        }
    }

    /// Returns the serialization error message if this is a SerializationError
    pub fn serialization_error_msg(&self) -> Option<&str> {
        match self {
            Self::SerializationError(msg) => Some(msg),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_messages() {
        let prop_error = CypherGuardValidationError::invalid_property_name("my_prop");
        assert_eq!(prop_error.to_string(), "Invalid property name: my_prop");
        assert_eq!(prop_error.property_name(), Some("my_prop"));

        // undefined_variable is now a parsing error, not validation error

        let type_error = CypherGuardValidationError::type_mismatch("String", "Integer");
        assert_eq!(
            type_error.to_string(),
            "Type mismatch: expected String, got Integer"
        );
        assert_eq!(
            type_error.type_mismatch_details(),
            Some(("String", "Integer"))
        );

        let rel_error = CypherGuardValidationError::invalid_relationship("KNOWS");
        assert_eq!(rel_error.to_string(), "Invalid relationship: KNOWS");
        assert_eq!(rel_error.relationship_name(), Some("KNOWS"));

        let label_error = CypherGuardValidationError::invalid_label("Person");
        assert_eq!(label_error.to_string(), "Invalid label: Person");
        assert_eq!(label_error.label_name(), Some("Person"));
    }

    #[test]
    fn test_parsing_error_messages() {
        let token_error = CypherGuardParsingError::expected_token("MATCH", "WITH");
        assert_eq!(token_error.to_string(), "Expected MATCH, found WITH");
        assert_eq!(
            token_error.expected_token_details(),
            Some(("MATCH", "WITH"))
        );

        let syntax_error = CypherGuardParsingError::invalid_syntax("Invalid clause order");
        assert_eq!(
            syntax_error.to_string(),
            "Invalid syntax: Invalid clause order"
        );
        assert_eq!(syntax_error.syntax_error(), Some("Invalid clause order"));

        let unexpected_end = CypherGuardParsingError::UnexpectedEnd;
        assert_eq!(unexpected_end.to_string(), "Unexpected end of input");
        assert!(unexpected_end.is_unexpected_end());

        // Test new error variants
        let clause_order = CypherGuardParsingError::invalid_clause_order(
            "query structure",
            "RETURN must come after MATCH",
        );
        assert_eq!(
            clause_order.to_string(),
            "Invalid clause order: query structure - RETURN must come after MATCH"
        );
        assert_eq!(
            clause_order.clause_order_error(),
            Some("query structure - RETURN must come after MATCH".to_string())
        );

        let missing_clause = CypherGuardParsingError::missing_required_clause("RETURN");
        assert_eq!(
            missing_clause.to_string(),
            "Missing required clause: RETURN"
        );
        assert_eq!(missing_clause.missing_clause(), Some("RETURN"));

        let invalid_where =
            CypherGuardParsingError::invalid_where_condition("parsing comparison", "a.age >");
        assert_eq!(
            invalid_where.to_string(),
            "Invalid WHERE condition: parsing comparison - a.age >"
        );
        assert_eq!(
            invalid_where.where_condition_error(),
            Some("parsing comparison - a.age >".to_string())
        );

        let invalid_pattern = CypherGuardParsingError::invalid_pattern(
            "relationship pattern",
            "invalid direction <-",
        );
        assert_eq!(
            invalid_pattern.to_string(),
            "Invalid pattern: relationship pattern - invalid direction <-"
        );
        assert_eq!(
            invalid_pattern.pattern_error(),
            Some("relationship pattern - invalid direction <-".to_string())
        );

        let invalid_expression =
            CypherGuardParsingError::invalid_expression("function call", "count(");
        assert_eq!(
            invalid_expression.to_string(),
            "Invalid expression: function call - count("
        );
        assert_eq!(
            invalid_expression.expression_error(),
            Some("function call - count(".to_string())
        );
    }

    #[test]
    fn test_schema_error_messages() {
        let format_error = CypherGuardSchemaError::invalid_format("Invalid JSON");
        assert_eq!(
            format_error.to_string(),
            "Invalid schema format: Invalid JSON"
        );
        assert_eq!(format_error.format_error(), Some("Invalid JSON"));

        let field_error = CypherGuardSchemaError::missing_field("name");
        assert_eq!(field_error.to_string(), "Missing required field: name");
        assert_eq!(field_error.missing_field_name(), Some("name"));

        let type_error = CypherGuardSchemaError::invalid_property_type("CustomType");
        assert_eq!(type_error.to_string(), "Invalid property type: CustomType");
        assert_eq!(type_error.property_type(), Some("CustomType"));

        let dup_error = CypherGuardSchemaError::duplicate_definition("Person");
        assert_eq!(dup_error.to_string(), "Duplicate definition: Person");
        assert_eq!(dup_error.duplicate_name(), Some("Person"));

        let prop_error = CypherGuardSchemaError::invalid_property_name("my_prop");
        assert_eq!(prop_error.to_string(), "Invalid property name: my_prop");
        assert_eq!(prop_error.property_name(), Some("my_prop"));

        let rel_error = CypherGuardSchemaError::invalid_relationship_pattern("KNOWS");
        assert_eq!(rel_error.to_string(), "Invalid relationship pattern: KNOWS");
        assert_eq!(rel_error.relationship_pattern(), Some("KNOWS"));

        let constraint_error = CypherGuardSchemaError::invalid_constraint("unique_name");
        assert_eq!(
            constraint_error.to_string(),
            "Invalid constraint: unique_name"
        );
        assert_eq!(constraint_error.constraint(), Some("unique_name"));

        let index_error = CypherGuardSchemaError::invalid_index("name_index");
        assert_eq!(index_error.to_string(), "Invalid index: name_index");
        assert_eq!(index_error.index(), Some("name_index"));

        let metadata_error = CypherGuardSchemaError::invalid_metadata("Invalid metadata");
        assert_eq!(
            metadata_error.to_string(),
            "Invalid metadata: Invalid metadata"
        );
        assert_eq!(metadata_error.metadata(), Some("Invalid metadata"));

        let enum_error = CypherGuardSchemaError::invalid_enum_values("Invalid enum");
        assert_eq!(enum_error.to_string(), "Invalid enum values: Invalid enum");
        assert_eq!(enum_error.enum_values(), Some("Invalid enum"));

        let range_error = CypherGuardSchemaError::invalid_value_range(10.0, 5.0);
        assert_eq!(
            range_error.to_string(),
            "Invalid value range: min 10 is greater than max 5"
        );
        assert_eq!(range_error.value_range(), Some((10.0, 5.0)));

        let count_error = CypherGuardSchemaError::invalid_distinct_value_count(-1);
        assert_eq!(count_error.to_string(), "Invalid distinct value count: -1");
        assert_eq!(count_error.distinct_value_count(), Some(-1));

        let example_error = CypherGuardSchemaError::invalid_example_values("Invalid examples");
        assert_eq!(
            example_error.to_string(),
            "Invalid example values: Invalid examples"
        );
        assert_eq!(example_error.example_values(), Some("Invalid examples"));

        let json_error = CypherGuardSchemaError::invalid_json("Invalid JSON");
        assert_eq!(json_error.to_string(), "Invalid JSON format: Invalid JSON");
        assert_eq!(json_error.json_error(), Some("Invalid JSON"));

        let io_error = CypherGuardSchemaError::io_error("File not found");
        assert_eq!(io_error.to_string(), "IO error: File not found");
        assert_eq!(io_error.io_error_msg(), Some("File not found"));
    }

    #[test]
    fn test_error_conversion() {
        // Test conversion from ValidationError to CypherGuardError
        let validation_error = CypherGuardValidationError::invalid_property_name("test");
        let cypher_error: CypherGuardError = validation_error.into();
        assert!(cypher_error.is_validation());
        assert!(cypher_error
            .to_string()
            .contains("Invalid property name: test"));

        // Test conversion from ParsingError to CypherGuardError
        let parsing_error = CypherGuardParsingError::invalid_syntax("test");
        let cypher_error: CypherGuardError = parsing_error.into();
        assert!(cypher_error.is_parsing());
        assert!(cypher_error.to_string().contains("Invalid syntax: test"));

        // Test conversion from SchemaError to CypherGuardError
        let schema_error = CypherGuardSchemaError::missing_field("test");
        let cypher_error: CypherGuardError = schema_error.into();
        assert!(cypher_error.is_schema());
        assert!(cypher_error
            .to_string()
            .contains("Missing required field: test"));
    }

    #[test]
    fn test_nom_error_conversion() {
        // Test conversion of different nom error types
        let nom_error = nom::error::Error::new("test", nom::error::ErrorKind::Char);
        let nom_err = nom::Err::Error(nom_error);

        let converted = convert_nom_error(nom_err, "parsing identifier", "test");
        assert!(converted.is_nom_error());

        // Test UnexpectedEnd conversion
        let incomplete_err: nom::Err<nom::error::Error<&str>> =
            nom::Err::Incomplete(nom::Needed::Size(std::num::NonZeroUsize::new(1).unwrap()));
        let converted = convert_nom_error(incomplete_err, "parsing query", "MATCH");
        assert!(converted.is_unexpected_end());

        // Test failure conversion
        let failure_error = nom::error::Error::new("test", nom::error::ErrorKind::Tag);
        let failure_err = nom::Err::Failure(failure_error);
        let converted = convert_nom_error(failure_err, "parsing clause", "test");
        assert!(converted.is_nom_error());
    }

    #[test]
    fn test_nom_error_variant() {
        // Test the Nom variant specifically
        let nom_error = nom::error::Error::new("test".to_string(), nom::error::ErrorKind::Char);
        let cypher_error = CypherGuardParsingError::Nom(nom_error);
        assert!(cypher_error.is_nom_error());
        assert!(cypher_error.to_string().contains("Nom parsing error"));
    }

    #[test]
    fn test_parsing_error_hierarchy() {
        // Test that high-level errors provide better context than low-level ones
        let high_level = CypherGuardParsingError::invalid_clause_order(
            "query structure",
            "RETURN must come after MATCH",
        );
        let low_level = CypherGuardParsingError::expected_token("(", ")");

        // High-level errors should have more descriptive messages
        assert!(high_level.to_string().contains("Invalid clause order"));
        assert!(low_level.to_string().contains("Expected ("));

        // Test that we can extract context from high-level errors
        assert!(high_level.clause_order_error().is_some());
        assert!(low_level.expected_token_details().is_some());
    }

    #[test]
    fn test_error_position_tracking() {
        // Test that error conversion preserves the original nom error
        let nom_error = nom::error::Error::new("xyz", nom::error::ErrorKind::Digit);
        let nom_err: nom::Err<nom::error::Error<&str>> = nom::Err::Error(nom_error);

        let converted = convert_nom_error(nom_err, "parsing number", "abc123xyz");
        assert!(converted.is_nom_error());

        // The original error information should be preserved
        if let CypherGuardParsingError::Nom(error) = converted {
            assert_eq!(error.input, "xyz");
            assert_eq!(error.code, nom::error::ErrorKind::Digit);
        } else {
            panic!("Expected Nom error");
        }
    }

    #[test]
    fn test_context_specific_errors() {
        // Test that the context parameter is available for future use
        // but doesn't affect the current simple conversion
        let alt_error = nom::error::Error::new("test", nom::error::ErrorKind::Alt);
        let alt_err: nom::Err<nom::error::Error<&str>> = nom::Err::Error(alt_error);

        let converted = convert_nom_error(alt_err, "parsing WHERE condition", "test");
        assert!(converted.is_nom_error());

        // The context could be used in the future for more sophisticated error handling
        // but for now we just preserve the original nom error
    }

    #[test]
    fn test_specific_clause_order_errors() {
        // Test specific clause order error variants
        let return_before = CypherGuardParsingError::return_before_other_clauses();
        assert_eq!(
            return_before.to_string(),
            "RETURN clause must come after all other clauses except ORDER BY, SKIP, LIMIT, and writing clauses (found at line 0, column 0)"
        );
        assert!(return_before.is_return_before_other_clauses());
        assert!(return_before.is_clause_order_error());

        let return_before_at = CypherGuardParsingError::return_before_other_clauses_at(1, 2);
        assert_eq!(
            return_before_at.to_string(),
            "RETURN clause must come after all other clauses except ORDER BY, SKIP, LIMIT, and writing clauses (found at line 1, column 2)"
        );
        assert!(return_before_at.is_return_before_other_clauses());
        assert!(return_before_at.is_clause_order_error());

        let match_after = CypherGuardParsingError::match_after_return();
        assert_eq!(
            match_after.to_string(),
            "MATCH clause cannot come after RETURN clause (found at line 0, column 0)"
        );
        assert!(match_after.is_match_after_return());
        assert!(match_after.is_clause_order_error());

        let match_after_at = CypherGuardParsingError::match_after_return_at(3, 4);
        assert_eq!(
            match_after_at.to_string(),
            "MATCH clause cannot come after RETURN clause (found at line 3, column 4)"
        );
        assert!(match_after_at.is_match_after_return());
        assert!(match_after_at.is_clause_order_error());

        let create_after = CypherGuardParsingError::create_after_return();
        assert_eq!(
            create_after.to_string(),
            "CREATE clause cannot come after RETURN clause (found at line 0, column 0)"
        );
        assert!(create_after.is_create_after_return());
        assert!(create_after.is_clause_order_error());

        let create_after_at = CypherGuardParsingError::create_after_return_at(5, 6);
        assert_eq!(
            create_after_at.to_string(),
            "CREATE clause cannot come after RETURN clause (found at line 5, column 6)"
        );
        assert!(create_after_at.is_create_after_return());
        assert!(create_after_at.is_clause_order_error());

        let merge_after = CypherGuardParsingError::merge_after_return();
        assert_eq!(
            merge_after.to_string(),
            "MERGE clause cannot come after RETURN clause (found at line 0, column 0)"
        );
        assert!(merge_after.is_merge_after_return());
        assert!(merge_after.is_clause_order_error());

        let merge_after_at = CypherGuardParsingError::merge_after_return_at(7, 8);
        assert_eq!(
            merge_after_at.to_string(),
            "MERGE clause cannot come after RETURN clause (found at line 7, column 8)"
        );
        assert!(merge_after_at.is_merge_after_return());
        assert!(merge_after_at.is_clause_order_error());

        let delete_after = CypherGuardParsingError::delete_after_return();
        assert_eq!(
            delete_after.to_string(),
            "DELETE clause cannot come after RETURN clause (found at line 0, column 0)"
        );
        assert!(delete_after.is_delete_after_return());
        assert!(delete_after.is_clause_order_error());

        let delete_after_at = CypherGuardParsingError::delete_after_return_at(9, 10);
        assert_eq!(
            delete_after_at.to_string(),
            "DELETE clause cannot come after RETURN clause (found at line 9, column 10)"
        );
        assert!(delete_after_at.is_delete_after_return());
        assert!(delete_after_at.is_clause_order_error());

        let set_after = CypherGuardParsingError::set_after_return();
        assert_eq!(
            set_after.to_string(),
            "SET clause cannot come after RETURN clause (found at line 0, column 0)"
        );
        assert!(set_after.is_set_after_return());
        assert!(set_after.is_clause_order_error());

        let set_after_at = CypherGuardParsingError::set_after_return_at(11, 12);
        assert_eq!(
            set_after_at.to_string(),
            "SET clause cannot come after RETURN clause (found at line 11, column 12)"
        );
        assert!(set_after_at.is_set_after_return());
        assert!(set_after_at.is_clause_order_error());

        let where_after = CypherGuardParsingError::where_after_return();
        assert_eq!(
            where_after.to_string(),
            "WHERE clause cannot come after RETURN clause (found at line 0, column 0)"
        );
        assert!(where_after.is_where_after_return());
        assert!(where_after.is_clause_order_error());

        let where_after_at = CypherGuardParsingError::where_after_return_at(13, 14);
        assert_eq!(
            where_after_at.to_string(),
            "WHERE clause cannot come after RETURN clause (found at line 13, column 14)"
        );
        assert!(where_after_at.is_where_after_return());
        assert!(where_after_at.is_clause_order_error());

        let with_after = CypherGuardParsingError::with_after_return();
        assert_eq!(
            with_after.to_string(),
            "WITH clause cannot come after RETURN clause (found at line 0, column 0)"
        );
        assert!(with_after.is_with_after_return());
        assert!(with_after.is_clause_order_error());

        let with_after_at = CypherGuardParsingError::with_after_return_at(15, 16);
        assert_eq!(
            with_after_at.to_string(),
            "WITH clause cannot come after RETURN clause (found at line 15, column 16)"
        );
        assert!(with_after_at.is_with_after_return());
        assert!(with_after_at.is_clause_order_error());

        let unwind_after = CypherGuardParsingError::unwind_after_return();
        assert_eq!(
            unwind_after.to_string(),
            "UNWIND clause cannot come after RETURN clause (found at line 0, column 0)"
        );
        assert!(unwind_after.is_unwind_after_return());
        assert!(unwind_after.is_clause_order_error());

        let unwind_after_at = CypherGuardParsingError::unwind_after_return_at(17, 18);
        assert_eq!(
            unwind_after_at.to_string(),
            "UNWIND clause cannot come after RETURN clause (found at line 17, column 18)"
        );
        assert!(unwind_after_at.is_unwind_after_return());
        assert!(unwind_after_at.is_clause_order_error());

        let where_before = CypherGuardParsingError::where_before_match();
        assert_eq!(
            where_before.to_string(),
            "WHERE clause must come after MATCH, UNWIND, or WITH clause (found at line 0, column 0)"
        );
        assert!(where_before.is_where_before_match());
        assert!(where_before.is_clause_order_error());

        let where_before_at = CypherGuardParsingError::where_before_match_at(19, 20);
        assert_eq!(
            where_before_at.to_string(),
            "WHERE clause must come after MATCH, UNWIND, or WITH clause (found at line 19, column 20)"
        );
        assert!(where_before_at.is_where_before_match());
        assert!(where_before_at.is_clause_order_error());

        let return_after = CypherGuardParsingError::return_after_return();
        assert_eq!(
            return_after.to_string(),
            "RETURN clause cannot appear after another RETURN clause (found at line 0, column 0)"
        );
        assert!(return_after.is_return_after_return());
        assert!(return_after.is_clause_order_error());

        let return_after_at = CypherGuardParsingError::return_after_return_at(21, 22);
        assert_eq!(
            return_after_at.to_string(),
            "RETURN clause cannot appear after another RETURN clause (found at line 21, column 22)"
        );
        assert!(return_after_at.is_return_after_return());
        assert!(return_after_at.is_clause_order_error());

        let order_by_before = CypherGuardParsingError::order_by_before_return();
        assert_eq!(
            order_by_before.to_string(),
            "ORDER BY clause must come after RETURN or WITH clause"
        );
        assert!(order_by_before.is_order_by_before_return());
        assert!(order_by_before.is_clause_order_error());

        let skip_before = CypherGuardParsingError::skip_before_return();
        assert_eq!(
            skip_before.to_string(),
            "SKIP clause must come after RETURN, WITH, or ORDER BY clause"
        );
        assert!(skip_before.is_skip_before_return());
        assert!(skip_before.is_clause_order_error());

        let limit_before = CypherGuardParsingError::limit_before_return();
        assert_eq!(
            limit_before.to_string(),
            "LIMIT clause must come after RETURN, WITH, ORDER BY, or SKIP clause"
        );
        assert!(limit_before.is_limit_before_return());
        assert!(limit_before.is_clause_order_error());
    }

    #[test]
    fn test_clause_order_error_detection() {
        // Test that the general clause order error detection works
        let generic_error = CypherGuardParsingError::invalid_clause_order("test", "details");
        assert!(generic_error.is_clause_order_error());

        // Test that non-clause order errors are not detected
        let syntax_error = CypherGuardParsingError::invalid_syntax("test");
        assert!(!syntax_error.is_clause_order_error());

        let nom_error = CypherGuardParsingError::Nom(nom::error::Error::new(
            "test".to_string(),
            nom::error::ErrorKind::Char,
        ));
        assert!(!nom_error.is_clause_order_error());
    }
}
