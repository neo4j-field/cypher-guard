// APOC diff procedures
// Handles apoc.diff.* procedures for diff operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC diff procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static DIFF_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.diff.graphs(graph1 MAP, graph2 MAP, config MAP)
        ("apoc.diff.graphs", vec![
            ("graph1", ApocType::Map),
            ("graph2", ApocType::Map),
            ("config", ApocType::Map)
        ], vec![("diff", ApocType::Map)]),
        
        // apoc.diff.nodes(node1 NODE, node2 NODE)
        ("apoc.diff.nodes", vec![
            ("node1", ApocType::Node),
            ("node2", ApocType::Node)
        ], vec![("diff", ApocType::Map)]),
        
        // apoc.diff.relationships(rel1 RELATIONSHIP, rel2 RELATIONSHIP)
        ("apoc.diff.relationships", vec![
            ("rel1", ApocType::Relationship),
            ("rel2", ApocType::Relationship)
        ], vec![("diff", ApocType::Map)]),
    ]
});

pub fn get_all_diff_procedures() -> &'static [ProcedureSignature] {
    &DIFF_PROCEDURES
}

// TODO: Implement diff procedure validation
pub fn validate_diff_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement diff procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_graphs_signature() {
        let procedures = get_all_diff_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.diff.graphs")
            .expect("apoc.diff.graphs should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "graph1");
        assert_eq!(procedure.1[1].0, "graph2");
        assert_eq!(procedure.1[2].0, "config");
    }

    #[test]
    fn test_diff_nodes_signature() {
        let procedures = get_all_diff_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.diff.nodes")
            .expect("apoc.diff.nodes should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "node1");
        assert_eq!(procedure.1[1].0, "node2");
    }

    #[test]
    fn test_diff_relationships_signature() {
        let procedures = get_all_diff_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.diff.relationships")
            .expect("apoc.diff.relationships should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "rel1");
        assert_eq!(procedure.1[1].0, "rel2");
    }

    #[test]
    fn test_all_diff_procedures_have_signatures() {
        let procedures = get_all_diff_procedures();
        assert!(!procedures.is_empty(), "Should have at least one diff procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


