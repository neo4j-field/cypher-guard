// APOC systemdb procedures
// Handles apoc.systemdb.* procedures for system database operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC systemdb procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static SYSTEMDB_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.systemdb.execute(cypher STRING, params MAP)
        ("apoc.systemdb.execute", vec![
            ("cypher", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.systemdb.executeRead(cypher STRING, params MAP)
        ("apoc.systemdb.executeRead", vec![
            ("cypher", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.systemdb.executeWrite(cypher STRING, params MAP)
        ("apoc.systemdb.executeWrite", vec![
            ("cypher", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
    ]
});

pub fn get_all_systemdb_procedures() -> &'static [ProcedureSignature] {
    &SYSTEMDB_PROCEDURES
}

// TODO: Implement systemdb procedure validation
pub fn validate_systemdb_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement systemdb procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systemdb_execute_signature() {
        let procedures = get_all_systemdb_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.systemdb.execute")
            .expect("apoc.systemdb.execute should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "cypher");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_systemdb_execute_read_signature() {
        let procedures = get_all_systemdb_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.systemdb.executeRead")
            .expect("apoc.systemdb.executeRead should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "cypher");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_systemdb_execute_write_signature() {
        let procedures = get_all_systemdb_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.systemdb.executeWrite")
            .expect("apoc.systemdb.executeWrite should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "cypher");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_all_systemdb_procedures_have_signatures() {
        let procedures = get_all_systemdb_procedures();
        assert!(!procedures.is_empty(), "Should have at least one systemdb procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


