// APOC path procedures
// Handles apoc.path.* procedures for path operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC path procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static PATH_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.path.subgraphAll(startNode NODE, maxLevel INTEGER)
        ("apoc.path.subgraphAll", vec![
            ("startNode", ApocType::Node),
            ("maxLevel", ApocType::Integer)
        ], vec![("node", ApocType::Node), ("relationship", ApocType::Relationship)]),
        
        // apoc.path.subgraphNodes(startNode NODE, maxLevel INTEGER)
        ("apoc.path.subgraphNodes", vec![
            ("startNode", ApocType::Node),
            ("maxLevel", ApocType::Integer)
        ], vec![("node", ApocType::Node)]),
        
        // apoc.path.subgraphRelationships(startNode NODE, maxLevel INTEGER)
        ("apoc.path.subgraphRelationships", vec![
            ("startNode", ApocType::Node),
            ("maxLevel", ApocType::Integer)
        ], vec![("relationship", ApocType::Relationship)]),
    ]
});

pub fn get_all_path_procedures() -> &'static [ProcedureSignature] {
    &PATH_PROCEDURES
}

// TODO: Implement path procedure validation
pub fn validate_path_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement path procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_subgraph_all_signature() {
        let procedures = get_all_path_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.path.subgraphAll")
            .expect("apoc.path.subgraphAll should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "startNode");
        assert_eq!(procedure.1[1].0, "maxLevel");
    }

    #[test]
    fn test_path_subgraph_nodes_signature() {
        let procedures = get_all_path_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.path.subgraphNodes")
            .expect("apoc.path.subgraphNodes should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "startNode");
        assert_eq!(procedure.1[1].0, "maxLevel");
    }

    #[test]
    fn test_path_subgraph_relationships_signature() {
        let procedures = get_all_path_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.path.subgraphRelationships")
            .expect("apoc.path.subgraphRelationships should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "startNode");
        assert_eq!(procedure.1[1].0, "maxLevel");
    }

    #[test]
    fn test_all_path_procedures_have_signatures() {
        let procedures = get_all_path_procedures();
        assert!(!procedures.is_empty(), "Should have at least one path procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
} 