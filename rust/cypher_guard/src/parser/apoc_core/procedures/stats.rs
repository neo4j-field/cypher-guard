// APOC stats procedures
// Handles apoc.stats.* procedures for statistics operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC stats procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static STATS_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.stats.degrees()
        ("apoc.stats.degrees", vec![], vec![("value", ApocType::Map)]),
        
        // apoc.stats.collect(cypher STRING, params MAP)
        ("apoc.stats.collect", vec![
            ("cypher", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Map)]),
        
        // apoc.stats.distinct(cypher STRING, params MAP)
        ("apoc.stats.distinct", vec![
            ("cypher", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Map)]),
    ]
});

pub fn get_all_stats_procedures() -> &'static [ProcedureSignature] {
    &STATS_PROCEDURES
}

// TODO: Implement stats procedure validation
pub fn validate_stats_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement stats procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_degrees_signature() {
        let procedures = get_all_stats_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.stats.degrees")
            .expect("apoc.stats.degrees should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_stats_collect_signature() {
        let procedures = get_all_stats_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.stats.collect")
            .expect("apoc.stats.collect should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "cypher");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_stats_distinct_signature() {
        let procedures = get_all_stats_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.stats.distinct")
            .expect("apoc.stats.distinct should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "cypher");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_all_stats_procedures_have_signatures() {
        let procedures = get_all_stats_procedures();
        assert!(!procedures.is_empty(), "Should have at least one stats procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


