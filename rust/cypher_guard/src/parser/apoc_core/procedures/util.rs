// APOC util procedures
// Handles apoc.util.* procedures for utility operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC util procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static UTIL_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.util.sleep(duration INTEGER)
        ("apoc.util.sleep", vec![
            ("duration", ApocType::Integer)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.util.validatePredicate(predicate ANY, message STRING)
        ("apoc.util.validatePredicate", vec![
            ("predicate", ApocType::Any),
            ("message", ApocType::String)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.util.decompress(data BYTEARRAY, compression STRING)
        ("apoc.util.decompress", vec![
            ("data", ApocType::ByteArray),
            ("compression", ApocType::String)
        ], vec![("value", ApocType::ByteArray)]),
        
        // apoc.util.compress(data BYTEARRAY, compression STRING)
        ("apoc.util.compress", vec![
            ("data", ApocType::ByteArray),
            ("compression", ApocType::String)
        ], vec![("value", ApocType::ByteArray)]),
        
        // apoc.util.md5(text STRING)
        ("apoc.util.md5", vec![
            ("text", ApocType::String)
        ], vec![("value", ApocType::String)]),
        
        // apoc.util.sha1(text STRING)
        ("apoc.util.sha1", vec![
            ("text", ApocType::String)
        ], vec![("value", ApocType::String)]),
        
        // apoc.util.sha256(text STRING)
        ("apoc.util.sha256", vec![
            ("text", ApocType::String)
        ], vec![("value", ApocType::String)]),
        
        // apoc.util.sha384(text STRING)
        ("apoc.util.sha384", vec![
            ("text", ApocType::String)
        ], vec![("value", ApocType::String)]),
        
        // apoc.util.sha512(text STRING)
        ("apoc.util.sha512", vec![
            ("text", ApocType::String)
        ], vec![("value", ApocType::String)]),
    ]
});

pub fn get_all_util_procedures() -> &'static [ProcedureSignature] {
    &UTIL_PROCEDURES
}

// TODO: Implement util procedure validation
pub fn validate_util_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement util procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_util_sleep_signature() {
        let procedures = get_all_util_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.util.sleep")
            .expect("apoc.util.sleep should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "duration");
    }

    #[test]
    fn test_util_validate_predicate_signature() {
        let procedures = get_all_util_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.util.validatePredicate")
            .expect("apoc.util.validatePredicate should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "predicate");
        assert_eq!(procedure.1[1].0, "message");
    }

    #[test]
    fn test_util_compress_signature() {
        let procedures = get_all_util_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.util.compress")
            .expect("apoc.util.compress should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "data");
        assert_eq!(procedure.1[1].0, "compression");
    }

    #[test]
    fn test_util_md5_signature() {
        let procedures = get_all_util_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.util.md5")
            .expect("apoc.util.md5 should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "text");
    }

    #[test]
    fn test_all_util_procedures_have_signatures() {
        let procedures = get_all_util_procedures();
        assert!(!procedures.is_empty(), "Should have at least one util procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "value", "First yield field should be 'value'");
        }
    }
}


