// APOC merge procedures
// Handles apoc.merge.* procedures for merge operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC merge procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static MERGE_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.merge.node(labels LIST<STRING>, identProps MAP, onCreateProps MAP, onMatchProps MAP)
        ("apoc.merge.node", vec![
            ("labels", ApocType::List),
            ("identProps", ApocType::Map),
            ("onCreateProps", ApocType::Map),
            ("onMatchProps", ApocType::Map)
        ], vec![("node", ApocType::Node)]),
        
        // apoc.merge.relationship(startNode NODE, relType STRING, identProps MAP, onCreateProps MAP, onMatchProps MAP, endNode NODE)
        ("apoc.merge.relationship", vec![
            ("startNode", ApocType::Node),
            ("relType", ApocType::String),
            ("identProps", ApocType::Map),
            ("onCreateProps", ApocType::Map),
            ("onMatchProps", ApocType::Map),
            ("endNode", ApocType::Node)
        ], vec![("relationship", ApocType::Relationship)]),
    ]
});

pub fn get_all_merge_procedures() -> &'static [ProcedureSignature] {
    &MERGE_PROCEDURES
}

// TODO: Implement merge procedure validation
pub fn validate_merge_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement merge procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_node_signature() {
        let procedures = get_all_merge_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.merge.node")
            .expect("apoc.merge.node should be defined");
        assert_eq!(procedure.1.len(), 4);
        assert_eq!(procedure.1[0].0, "labels");
        assert_eq!(procedure.1[1].0, "identProps");
        assert_eq!(procedure.1[2].0, "onCreateProps");
        assert_eq!(procedure.1[3].0, "onMatchProps");
    }

    #[test]
    fn test_merge_relationship_signature() {
        let procedures = get_all_merge_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.merge.relationship")
            .expect("apoc.merge.relationship should be defined");
        assert_eq!(procedure.1.len(), 6);
        assert_eq!(procedure.1[0].0, "startNode");
        assert_eq!(procedure.1[1].0, "relType");
        assert_eq!(procedure.1[2].0, "identProps");
        assert_eq!(procedure.1[3].0, "onCreateProps");
        assert_eq!(procedure.1[4].0, "onMatchProps");
        assert_eq!(procedure.1[5].0, "endNode");
    }

    #[test]
    fn test_all_merge_procedures_have_signatures() {
        let procedures = get_all_merge_procedures();
        assert!(!procedures.is_empty(), "Should have at least one merge procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


