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
    ByteArray, // Added for backward compatibility
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
            (ApocType::Boolean, PropertyValue::Boolean(_)) => true,
            (ApocType::Any, _) => true,
            (ApocType::List, PropertyValue::List(_)) => true,
            (ApocType::Map, PropertyValue::Map(_)) => true,
            // Handle both Number and Integer for backward compatibility
            (ApocType::Integer, PropertyValue::Integer(_)) => true,
            (ApocType::Integer, PropertyValue::Number(_)) => true,
            (ApocType::Float, PropertyValue::Float(_)) => true,
            (ApocType::ByteArray, PropertyValue::String(_)) => true, // Treat as string for now
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
    use crate::parser::ast::{NodePattern, RelationshipPattern, PatternElement};

    #[test]
    fn test_type_matching() {
        // Test basic type matching
        assert!(ApocType::String.matches(&PropertyValue::String("test".to_string())));
        assert!(ApocType::Boolean.matches(&PropertyValue::Boolean(true)));
        assert!(ApocType::Integer.matches(&PropertyValue::Integer(42)));
        assert!(ApocType::Integer.matches(&PropertyValue::Number(42))); // Backward compatibility
        assert!(ApocType::Float.matches(&PropertyValue::Float(3.14)));
        
        // Test type mismatches
        assert!(!ApocType::String.matches(&PropertyValue::Integer(42)));
        assert!(!ApocType::Integer.matches(&PropertyValue::String("42".to_string())));
        
        // Test Any type accepts everything
        assert!(ApocType::Any.matches(&PropertyValue::String("test".to_string())));
        assert!(ApocType::Any.matches(&PropertyValue::Integer(42)));
        assert!(ApocType::Any.matches(&PropertyValue::Boolean(true)));
    }
} 