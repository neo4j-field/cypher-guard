// APOC run procedures
// Handles apoc.run.* procedures for run operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC run procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static RUN_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.run(cypher STRING, params MAP)
        ("apoc.run", vec![
            ("cypher", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.runMany(cypher STRING, params MAP)
        ("apoc.runMany", vec![
            ("cypher", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
    ]
});

pub fn get_all_run_procedures() -> &'static [ProcedureSignature] {
    &RUN_PROCEDURES
}

// TODO: Implement run procedure validation
pub fn validate_run_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement run procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_signature() {
        let procedures = get_all_run_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.run")
            .expect("apoc.run should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "cypher");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_run_many_signature() {
        let procedures = get_all_run_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.runMany")
            .expect("apoc.runMany should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "cypher");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_all_run_procedures_have_signatures() {
        let procedures = get_all_run_procedures();
        assert!(!procedures.is_empty(), "Should have at least one run procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


