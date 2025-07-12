// APOC util procedures
// Handles apoc.util.* procedures for utility operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc::types::{ApocType, ProcedureSignature};

// APOC util procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
pub const UTIL_PROCEDURES: &[ProcedureSignature] = &[
    // apoc.util.sleep(duration INTEGER)
    ("apoc.util.sleep", vec![
        ("duration", ApocType::Integer)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.util.validate(predicate BOOLEAN, message STRING, params LIST<ANY>)
    ("apoc.util.validate", vec![
        ("predicate", ApocType::Boolean),
        ("message", ApocType::String),
        ("params", ApocType::List)
    ], vec![("result", ApocType::Any)]),
];

// APOC util functions (these are functions, not procedures)
// Note: These would typically go in a separate functions module
// apoc.util.compress(data STRING, config MAP<STRING, ANY>) -> Function
// apoc.util.decompress(data LIST<INTEGER>, config MAP<STRING, ANY>) -> Function  
// apoc.util.md5(values LIST<ANY>) -> Function
// apoc.util.sha1(values LIST<ANY>) -> Function
// apoc.util.sha256(values LIST<ANY>) -> Function
// apoc.util.sha384(values LIST<ANY>) -> Function
// apoc.util.sha512(values LIST<ANY>) -> Function
// apoc.util.validatePredicate(predicate BOOLEAN, message STRING, params LIST<ANY>) -> Function

// TODO: Implement util procedure validation
// This will be implemented once we reference the APOC documentation
pub fn validate_util_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement util procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_util_sleep_signature() {
        let procedure = UTIL_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.util.sleep")
            .expect("apoc.util.sleep should be defined");
        
        assert_eq!(procedure.1.len(), 1); // 1 parameter
        assert_eq!(procedure.1[0].0, "duration");
        assert_eq!(procedure.1[0].1, ApocType::Integer);
    }

    #[test]
    fn test_util_validate_signature() {
        let procedure = UTIL_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.util.validate")
            .expect("apoc.util.validate should be defined");
        
        assert_eq!(procedure.1.len(), 3); // 3 parameters
        assert_eq!(procedure.1[0].0, "predicate");
        assert_eq!(procedure.1[0].1, ApocType::Boolean);
        assert_eq!(procedure.1[1].0, "message");
        assert_eq!(procedure.1[1].1, ApocType::String);
        assert_eq!(procedure.1[2].0, "params");
        assert_eq!(procedure.1[2].1, ApocType::List);
    }

    #[test]
    fn test_all_util_procedures_have_signatures() {
        assert!(!UTIL_PROCEDURES.is_empty(), "Should have at least one util procedure");
        
        for (name, args, yields) in UTIL_PROCEDURES {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
        }
    }
}


