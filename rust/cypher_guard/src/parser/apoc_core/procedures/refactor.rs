// APOC refactor procedures
// Handles apoc.refactor.* procedures for refactoring operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC refactor procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static REFACTOR_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.refactor.rename.label(oldLabel STRING, newLabel STRING, nodes LIST<NODE>)
        ("apoc.refactor.rename.label", vec![
            ("oldLabel", ApocType::String),
            ("newLabel", ApocType::String),
            ("nodes", ApocType::List)
        ], vec![("node", ApocType::Node)]),
        
        // apoc.refactor.rename.type(oldType STRING, newType STRING, relationships LIST<RELATIONSHIP>)
        ("apoc.refactor.rename.type", vec![
            ("oldType", ApocType::String),
            ("newType", ApocType::String),
            ("relationships", ApocType::List)
        ], vec![("relationship", ApocType::Relationship)]),
        
        // apoc.refactor.mergeNodes(nodes LIST<NODE>, properties MAP)
        ("apoc.refactor.mergeNodes", vec![
            ("nodes", ApocType::List),
            ("properties", ApocType::Map)
        ], vec![("node", ApocType::Node)]),
        
        // apoc.refactor.collapsePath(paths LIST<PATH>, type STRING, direction STRING)
        ("apoc.refactor.collapsePath", vec![
            ("paths", ApocType::List),
            ("type", ApocType::String),
            ("direction", ApocType::String)
        ], vec![("relationship", ApocType::Relationship)]),
    ]
});

pub fn get_all_refactor_procedures() -> &'static [ProcedureSignature] {
    &REFACTOR_PROCEDURES
}

// TODO: Implement refactor procedure validation
pub fn validate_refactor_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement refactor procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refactor_rename_label_signature() {
        let procedures = get_all_refactor_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.refactor.rename.label")
            .expect("apoc.refactor.rename.label should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "oldLabel");
        assert_eq!(procedure.1[1].0, "newLabel");
        assert_eq!(procedure.1[2].0, "nodes");
    }

    #[test]
    fn test_refactor_rename_type_signature() {
        let procedures = get_all_refactor_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.refactor.rename.type")
            .expect("apoc.refactor.rename.type should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "oldType");
        assert_eq!(procedure.1[1].0, "newType");
        assert_eq!(procedure.1[2].0, "relationships");
    }

    #[test]
    fn test_refactor_merge_nodes_signature() {
        let procedures = get_all_refactor_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.refactor.mergeNodes")
            .expect("apoc.refactor.mergeNodes should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "nodes");
        assert_eq!(procedure.1[1].0, "properties");
    }

    #[test]
    fn test_all_refactor_procedures_have_signatures() {
        let procedures = get_all_refactor_procedures();
        assert!(!procedures.is_empty(), "Should have at least one refactor procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


