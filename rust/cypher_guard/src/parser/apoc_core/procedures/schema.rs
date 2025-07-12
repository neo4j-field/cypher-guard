// APOC schema procedures
// Handles apoc.schema.* procedures for schema operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC schema procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static SCHEMA_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.schema.assert(indexes MAP, constraints MAP)
        ("apoc.schema.assert", vec![
            ("indexes", ApocType::Map),
            ("constraints", ApocType::Map)
        ], vec![("label", ApocType::String)]),
        
        // apoc.schema.assert.indexes(indexes MAP)
        ("apoc.schema.assert.indexes", vec![
            ("indexes", ApocType::Map)
        ], vec![("label", ApocType::String)]),
        
        // apoc.schema.assert.constraints(constraints MAP)
        ("apoc.schema.assert.constraints", vec![
            ("constraints", ApocType::Map)
        ], vec![("label", ApocType::String)]),
    ]
});

pub fn get_all_schema_procedures() -> &'static [ProcedureSignature] {
    &SCHEMA_PROCEDURES
}

// TODO: Implement schema procedure validation
pub fn validate_schema_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement schema procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_assert_signature() {
        let procedures = get_all_schema_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.schema.assert")
            .expect("apoc.schema.assert should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "indexes");
        assert_eq!(procedure.1[1].0, "constraints");
    }

    #[test]
    fn test_schema_assert_indexes_signature() {
        let procedures = get_all_schema_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.schema.assert.indexes")
            .expect("apoc.schema.assert.indexes should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "indexes");
    }

    #[test]
    fn test_schema_assert_constraints_signature() {
        let procedures = get_all_schema_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.schema.assert.constraints")
            .expect("apoc.schema.assert.constraints should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "constraints");
    }

    #[test]
    fn test_all_schema_procedures_have_signatures() {
        let procedures = get_all_schema_procedures();
        assert!(!procedures.is_empty(), "Should have at least one schema procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


