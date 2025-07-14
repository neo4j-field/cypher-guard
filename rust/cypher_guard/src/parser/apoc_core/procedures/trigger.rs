// APOC trigger procedures
// Handles apoc.trigger.* procedures for trigger operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC trigger procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static TRIGGER_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.trigger.add(name STRING, statement STRING, selector MAP)
        ("apoc.trigger.add", vec![
            ("name", ApocType::String),
            ("statement", ApocType::String),
            ("selector", ApocType::Map)
        ], vec![("name", ApocType::String)]),
        
        // apoc.trigger.remove(name STRING)
        ("apoc.trigger.remove", vec![
            ("name", ApocType::String)
        ], vec![("name", ApocType::String)]),
        
        // apoc.trigger.removeAll()
        ("apoc.trigger.removeAll", vec![], vec![("name", ApocType::String)]),
        
        // apoc.trigger.list()
        ("apoc.trigger.list", vec![], vec![("name", ApocType::String), ("statement", ApocType::String)]),
        
        // apoc.trigger.install(database STRING, name STRING, statement STRING, selector MAP)
        ("apoc.trigger.install", vec![
            ("database", ApocType::String),
            ("name", ApocType::String),
            ("statement", ApocType::String),
            ("selector", ApocType::Map)
        ], vec![("name", ApocType::String)]),
        
        // apoc.trigger.drop(database STRING, name STRING)
        ("apoc.trigger.drop", vec![
            ("database", ApocType::String),
            ("name", ApocType::String)
        ], vec![("name", ApocType::String)]),
    ]
});

pub fn get_all_trigger_procedures() -> &'static [ProcedureSignature] {
    &TRIGGER_PROCEDURES
}

// TODO: Implement trigger procedure validation
pub fn validate_trigger_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement trigger procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_add_signature() {
        let procedures = get_all_trigger_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.trigger.add")
            .expect("apoc.trigger.add should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "name");
        assert_eq!(procedure.1[1].0, "statement");
        assert_eq!(procedure.1[2].0, "selector");
    }

    #[test]
    fn test_trigger_remove_signature() {
        let procedures = get_all_trigger_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.trigger.remove")
            .expect("apoc.trigger.remove should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "name");
    }

    #[test]
    fn test_trigger_remove_all_signature() {
        let procedures = get_all_trigger_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.trigger.removeAll")
            .expect("apoc.trigger.removeAll should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_trigger_list_signature() {
        let procedures = get_all_trigger_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.trigger.list")
            .expect("apoc.trigger.list should be defined");
        assert_eq!(procedure.1.len(), 0);
        assert_eq!(procedure.2.len(), 2);
        assert_eq!(procedure.2[0].0, "name");
        assert_eq!(procedure.2[1].0, "statement");
    }

    #[test]
    fn test_all_trigger_procedures_have_signatures() {
        let procedures = get_all_trigger_procedures();
        assert!(!procedures.is_empty(), "Should have at least one trigger procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


