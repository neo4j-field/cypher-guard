// APOC node procedures
// Handles apoc.node.* procedures for node operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC node procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static NODE_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.node.degree(node NODE, relType STRING)
        ("apoc.node.degree", vec![
            ("node", ApocType::Node),
            ("relType", ApocType::String)
        ], vec![("value", ApocType::Integer)]),
        
        // apoc.node.degree.in(node NODE, relType STRING)
        ("apoc.node.degree.in", vec![
            ("node", ApocType::Node),
            ("relType", ApocType::String)
        ], vec![("value", ApocType::Integer)]),
        
        // apoc.node.degree.out(node NODE, relType STRING)
        ("apoc.node.degree.out", vec![
            ("node", ApocType::Node),
            ("relType", ApocType::String)
        ], vec![("value", ApocType::Integer)]),
        
        // apoc.node.relationship.types(node NODE)
        ("apoc.node.relationship.types", vec![
            ("node", ApocType::Node)
        ], vec![("value", ApocType::String)]),
        
        // apoc.node.labels(node NODE)
        ("apoc.node.labels", vec![
            ("node", ApocType::Node)
        ], vec![("value", ApocType::String)]),
    ]
});

pub fn get_all_node_procedures() -> &'static [ProcedureSignature] {
    &NODE_PROCEDURES
}

// TODO: Implement node procedure validation
pub fn validate_node_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement node procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_degree_signature() {
        let procedures = get_all_node_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.node.degree")
            .expect("apoc.node.degree should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "node");
        assert_eq!(procedure.1[1].0, "relType");
    }

    #[test]
    fn test_node_degree_in_signature() {
        let procedures = get_all_node_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.node.degree.in")
            .expect("apoc.node.degree.in should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "node");
        assert_eq!(procedure.1[1].0, "relType");
    }

    #[test]
    fn test_node_degree_out_signature() {
        let procedures = get_all_node_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.node.degree.out")
            .expect("apoc.node.degree.out should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "node");
        assert_eq!(procedure.1[1].0, "relType");
    }

    #[test]
    fn test_node_relationship_types_signature() {
        let procedures = get_all_node_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.node.relationship.types")
            .expect("apoc.node.relationship.types should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "node");
    }

    #[test]
    fn test_all_node_procedures_have_signatures() {
        let procedures = get_all_node_procedures();
        assert!(!procedures.is_empty(), "Should have at least one node procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
} 