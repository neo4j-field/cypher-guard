// APOC algorithm procedures
// Handles apoc.algo.* procedures for graph algorithms

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC algorithm procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
pub static ALGO_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.algo.dijkstra(startNode NODE, endNode NODE, relationshipTypesAndDirections STRING, weightPropertyName STRING)
        ("apoc.algo.dijkstra", vec![
            ("startNode", ApocType::Node),
            ("endNode", ApocType::Node),
            ("relationshipTypesAndDirections", ApocType::String),
            ("weightPropertyName", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.algo.aStar(startNode NODE, endNode NODE, relationshipTypesAndDirections STRING, weightPropertyName STRING, latPropertyName STRING, lonPropertyName STRING)
        ("apoc.algo.aStar", vec![
            ("startNode", ApocType::Node),
            ("endNode", ApocType::Node),
            ("relationshipTypesAndDirections", ApocType::String),
            ("weightPropertyName", ApocType::String),
            ("latPropertyName", ApocType::String),
            ("lonPropertyName", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.algo.betweenness(rels RELATIONSHIP[], nodes NODE[], direction STRING)
        ("apoc.algo.betweenness", vec![
            ("rels", ApocType::List),
            ("nodes", ApocType::List),
            ("direction", ApocType::String)
        ], vec![("result", ApocType::Any)]),
    ]
});

// TODO: Implement algorithm procedure validation
// This will be implemented once we reference the APOC documentation
pub fn validate_algo_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement algorithm procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algo_dijkstra_signature() {
        let procedure = ALGO_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.algo.dijkstra")
            .expect("apoc.algo.dijkstra should be defined");
        
        assert_eq!(procedure.1.len(), 4); // 4 parameters
        assert_eq!(procedure.1[0].0, "startNode");
        assert_eq!(procedure.1[0].1, ApocType::Node);
        assert_eq!(procedure.1[1].0, "endNode");
        assert_eq!(procedure.1[1].1, ApocType::Node);
        assert_eq!(procedure.1[2].0, "relationshipTypesAndDirections");
        assert_eq!(procedure.1[2].1, ApocType::String);
        assert_eq!(procedure.1[3].0, "weightPropertyName");
        assert_eq!(procedure.1[3].1, ApocType::String);
    }

    #[test]
    fn test_algo_astar_signature() {
        let procedure = ALGO_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.algo.aStar")
            .expect("apoc.algo.aStar should be defined");
        
        assert_eq!(procedure.1.len(), 6); // 6 parameters
        assert_eq!(procedure.1[0].0, "startNode");
        assert_eq!(procedure.1[0].1, ApocType::Node);
        assert_eq!(procedure.1[1].0, "endNode");
        assert_eq!(procedure.1[1].1, ApocType::Node);
        assert_eq!(procedure.1[2].0, "relationshipTypesAndDirections");
        assert_eq!(procedure.1[2].1, ApocType::String);
        assert_eq!(procedure.1[3].0, "weightPropertyName");
        assert_eq!(procedure.1[3].1, ApocType::String);
        assert_eq!(procedure.1[4].0, "latPropertyName");
        assert_eq!(procedure.1[4].1, ApocType::String);
        assert_eq!(procedure.1[5].0, "lonPropertyName");
        assert_eq!(procedure.1[5].1, ApocType::String);
    }

    #[test]
    fn test_all_algo_procedures_have_signatures() {
        assert!(!ALGO_PROCEDURES.is_empty(), "Should have at least one algorithm procedure");
        
        for (name, args, yields) in ALGO_PROCEDURES.iter() {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
        }
    }
}


