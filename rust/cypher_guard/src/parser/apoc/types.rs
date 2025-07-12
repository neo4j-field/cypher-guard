// APOC shared types
// Common types used across all APOC modules

use crate::parser::ast::PropertyValue;

// APOC type system for procedure arguments and return values
#[derive(Debug, Clone, PartialEq)]
pub enum ApocType {
    String,
    Integer,
    Float,
    Boolean,
    Map,
    List,
    Any,  // For values that can be any type
    Node,
    Relationship,
    Path,
}

// Procedure signature: (name, arguments, yield_fields)
// Arguments are (name, type) pairs
// Yield fields are (name, type) pairs
pub type ProcedureSignature = (&'static str, Vec<(&'static str, ApocType)>, Vec<(&'static str, ApocType)>);

// Function signature: (name, arguments, return_type)
// Arguments are (name, type) pairs
pub type FunctionSignature = (&'static str, Vec<(&'static str, ApocType)>, ApocType);

// Helper functions for type validation
impl ApocType {
    /// Check if a PropertyValue matches this ApocType
    pub fn matches(&self, value: &PropertyValue) -> bool {
        match (self, value) {
            (ApocType::String, PropertyValue::String(_)) => true,
            (ApocType::Integer, PropertyValue::Integer(_)) => true,
            (ApocType::Float, PropertyValue::Float(_)) => true,
            (ApocType::Boolean, PropertyValue::Boolean(_)) => true,
            (ApocType::Map, PropertyValue::Map(_)) => true,
            (ApocType::List, PropertyValue::List(_)) => true,
            (ApocType::Any, _) => true, // Any type accepts everything
            (ApocType::Node, PropertyValue::Node(_)) => true,
            (ApocType::Relationship, PropertyValue::Relationship(_)) => true,
            (ApocType::Path, PropertyValue::Path(_)) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Node, Relationship, Path};

    #[test]
    fn test_apoc_type_matching() {
        assert!(ApocType::String.matches(&PropertyValue::String("test".to_string())));
        assert!(ApocType::Integer.matches(&PropertyValue::Integer(42)));
        assert!(ApocType::Float.matches(&PropertyValue::Float(3.14)));
        assert!(ApocType::Boolean.matches(&PropertyValue::Boolean(true)));
        assert!(ApocType::Any.matches(&PropertyValue::String("anything".to_string())));
        assert!(!ApocType::String.matches(&PropertyValue::Integer(42)));
    }
} 