// APOC meta procedures
// Handles apoc.meta.* procedures for metadata operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC meta procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static META_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.meta.data()
        ("apoc.meta.data", vec![], vec![("label", ApocType::String)]),
        
        // apoc.meta.schema()
        ("apoc.meta.schema", vec![], vec![("label", ApocType::String)]),
        
        // apoc.meta.type(value ANY)
        ("apoc.meta.type", vec![
            ("value", ApocType::Any)
        ], vec![("type", ApocType::String)]),
    ]
});

pub fn get_all_meta_procedures() -> &'static [ProcedureSignature] {
    &META_PROCEDURES
}

// TODO: Implement meta procedure validation
pub fn validate_meta_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement meta procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_data_signature() {
        let procedures = get_all_meta_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.meta.data")
            .expect("apoc.meta.data should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_meta_schema_signature() {
        let procedures = get_all_meta_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.meta.schema")
            .expect("apoc.meta.schema should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_meta_type_signature() {
        let procedures = get_all_meta_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.meta.type")
            .expect("apoc.meta.type should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "value");
    }

    #[test]
    fn test_all_meta_procedures_have_signatures() {
        let procedures = get_all_meta_procedures();
        assert!(!procedures.is_empty(), "Should have at least one meta procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


