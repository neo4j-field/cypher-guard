// APOC aggregation procedures
// Handles apoc.agg.* procedures for aggregation operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC aggregation procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static AGG_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.agg.graphStats(graphName STRING)
        ("apoc.agg.graphStats", vec![
            ("graphName", ApocType::String),
        ], vec![("nodeCount", ApocType::Integer)]),
        
        // apoc.agg.relationships(graphName STRING, types LIST<STRING>)
        ("apoc.agg.relationships", vec![
            ("graphName", ApocType::String),
            ("types", ApocType::List),
        ], vec![("type", ApocType::String), ("count", ApocType::Integer)]),
    ]
});

pub fn get_all_agg_procedures() -> &'static [ProcedureSignature] {
    &AGG_PROCEDURES
}

// TODO: Implement aggregation procedure validation
pub fn validate_agg_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement aggregation procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agg_graph_stats_signature() {
        let procedures = get_all_agg_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.agg.graphStats")
            .expect("apoc.agg.graphStats should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "graphName");
    }

    #[test]
    fn test_agg_relationships_signature() {
        let procedures = get_all_agg_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.agg.relationships")
            .expect("apoc.agg.relationships should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "graphName");
        assert_eq!(procedure.1[1].0, "types");
    }

    #[test]
    fn test_all_agg_procedures_have_signatures() {
        let procedures = get_all_agg_procedures();
        assert!(!procedures.is_empty(), "Should have at least one aggregation procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
} 