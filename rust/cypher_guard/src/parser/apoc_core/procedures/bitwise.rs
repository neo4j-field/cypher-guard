// APOC bitwise procedures
// Handles apoc.bitwise.* procedures for bitwise operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC bitwise procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
pub static BITWISE_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.bitwise.op(a INTEGER, b INTEGER, operator STRING)
        ("apoc.bitwise.op", vec![
            ("a", ApocType::Integer),
            ("b", ApocType::Integer),
            ("operator", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.bitwise.op(a INTEGER, operator STRING)
        ("apoc.bitwise.op", vec![
            ("a", ApocType::Integer),
            ("operator", ApocType::String)
        ], vec![("result", ApocType::Any)]),
    ]
});

// TODO: Implement bitwise procedure validation
// This will be implemented once we reference the APOC documentation
pub fn validate_bitwise_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement bitwise procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitwise_op_signature() {
        let procedure = BITWISE_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.bitwise.op")
            .expect("apoc.bitwise.op should be defined");
        
        assert_eq!(procedure.1.len(), 3); // 3 parameters
        assert_eq!(procedure.1[0].0, "a");
        assert_eq!(procedure.1[0].1, ApocType::Integer);
        assert_eq!(procedure.1[1].0, "b");
        assert_eq!(procedure.1[1].1, ApocType::Integer);
        assert_eq!(procedure.1[2].0, "operator");
        assert_eq!(procedure.1[2].1, ApocType::String);
    }

    #[test]
    fn test_all_bitwise_procedures_have_signatures() {
        assert!(!BITWISE_PROCEDURES.is_empty(), "Should have at least one bitwise procedure");
        
        for (name, args, yields) in BITWISE_PROCEDURES.iter() {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
        }
    }
}


