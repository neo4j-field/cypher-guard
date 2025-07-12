// APOC periodic procedures
// Handles apoc.periodic.* procedures for periodic operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC periodic procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static PERIODIC_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.periodic.iterate(cypherIterate STRING, cypherAction STRING, config MAP)
        ("apoc.periodic.iterate", vec![
            ("cypherIterate", ApocType::String),
            ("cypherAction", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("batch", ApocType::Integer)]),
        
        // apoc.periodic.commit(cypherIterate STRING, cypherAction STRING, config MAP)
        ("apoc.periodic.commit", vec![
            ("cypherIterate", ApocType::String),
            ("cypherAction", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("batch", ApocType::Integer)]),
        
        // apoc.periodic.repeat(name STRING, cypher STRING, period INTEGER, config MAP)
        ("apoc.periodic.repeat", vec![
            ("name", ApocType::String),
            ("cypher", ApocType::String),
            ("period", ApocType::Integer),
            ("config", ApocType::Map)
        ], vec![("name", ApocType::String)]),
    ]
});

pub fn get_all_periodic_procedures() -> &'static [ProcedureSignature] {
    &PERIODIC_PROCEDURES
}

// TODO: Implement periodic procedure validation
pub fn validate_periodic_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement periodic procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_periodic_iterate_signature() {
        let procedures = get_all_periodic_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.periodic.iterate")
            .expect("apoc.periodic.iterate should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "cypherIterate");
        assert_eq!(procedure.1[1].0, "cypherAction");
        assert_eq!(procedure.1[2].0, "config");
    }

    #[test]
    fn test_periodic_commit_signature() {
        let procedures = get_all_periodic_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.periodic.commit")
            .expect("apoc.periodic.commit should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "cypherIterate");
        assert_eq!(procedure.1[1].0, "cypherAction");
        assert_eq!(procedure.1[2].0, "config");
    }

    #[test]
    fn test_periodic_repeat_signature() {
        let procedures = get_all_periodic_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.periodic.repeat")
            .expect("apoc.periodic.repeat should be defined");
        assert_eq!(procedure.1.len(), 4);
        assert_eq!(procedure.1[0].0, "name");
        assert_eq!(procedure.1[1].0, "cypher");
        assert_eq!(procedure.1[2].0, "period");
        assert_eq!(procedure.1[3].0, "config");
    }

    #[test]
    fn test_all_periodic_procedures_have_signatures() {
        let procedures = get_all_periodic_procedures();
        assert!(!procedures.is_empty(), "Should have at least one periodic procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


