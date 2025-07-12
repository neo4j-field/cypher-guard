// APOC hash procedures
// Handles apoc.hash.* procedures for hashing operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC hash procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static HASH_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.hash.sha256(value STRING)
        ("apoc.hash.sha256", vec![
            ("value", ApocType::String)
        ], vec![("hash", ApocType::String)]),
        
        // apoc.hash.sha1(value STRING)
        ("apoc.hash.sha1", vec![
            ("value", ApocType::String)
        ], vec![("hash", ApocType::String)]),
        
        // apoc.hash.md5(value STRING)
        ("apoc.hash.md5", vec![
            ("value", ApocType::String)
        ], vec![("hash", ApocType::String)]),
    ]
});

pub fn get_all_hash_procedures() -> &'static [ProcedureSignature] {
    &HASH_PROCEDURES
}

// TODO: Implement hash procedure validation
pub fn validate_hash_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement hash procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_sha256_signature() {
        let procedures = get_all_hash_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.hash.sha256")
            .expect("apoc.hash.sha256 should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "value");
    }

    #[test]
    fn test_hash_sha1_signature() {
        let procedures = get_all_hash_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.hash.sha1")
            .expect("apoc.hash.sha1 should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "value");
    }

    #[test]
    fn test_hash_md5_signature() {
        let procedures = get_all_hash_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.hash.md5")
            .expect("apoc.hash.md5 should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "value");
    }

    #[test]
    fn test_all_hash_procedures_have_signatures() {
        let procedures = get_all_hash_procedures();
        assert!(!procedures.is_empty(), "Should have at least one hash procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


