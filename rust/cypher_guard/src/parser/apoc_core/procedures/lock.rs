// APOC lock procedures
// Handles apoc.lock.* procedures for locking operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC lock procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static LOCK_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.lock.all()
        ("apoc.lock.all", vec![], vec![("lock", ApocType::Any)]),
        
        // apoc.lock.nodes(nodes LIST<NODE>)
        ("apoc.lock.nodes", vec![
            ("nodes", ApocType::List)
        ], vec![("lock", ApocType::Any)]),
        
        // apoc.lock.relationships(rels LIST<RELATIONSHIP>)
        ("apoc.lock.relationships", vec![
            ("rels", ApocType::List)
        ], vec![("lock", ApocType::Any)]),
    ]
});

pub fn get_all_lock_procedures() -> &'static [ProcedureSignature] {
    &LOCK_PROCEDURES
}

// TODO: Implement lock procedure validation
pub fn validate_lock_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement lock procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_all_signature() {
        let procedures = get_all_lock_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.lock.all")
            .expect("apoc.lock.all should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_lock_nodes_signature() {
        let procedures = get_all_lock_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.lock.nodes")
            .expect("apoc.lock.nodes should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "nodes");
    }

    #[test]
    fn test_lock_relationships_signature() {
        let procedures = get_all_lock_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.lock.relationships")
            .expect("apoc.lock.relationships should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "rels");
    }

    #[test]
    fn test_all_lock_procedures_have_signatures() {
        let procedures = get_all_lock_procedures();
        assert!(!procedures.is_empty(), "Should have at least one lock procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


