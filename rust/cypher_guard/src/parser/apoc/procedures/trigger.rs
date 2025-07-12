// APOC trigger procedures
// Handles apoc.trigger.* procedures for trigger operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc::types::{ApocType, ProcedureSignature};

// APOC trigger procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
pub const TRIGGER_PROCEDURES: &[ProcedureSignature] = &[
    // apoc.trigger.add(name STRING, statement STRING, selector MAP<STRING, ANY>, config MAP<STRING, ANY>)
    ("apoc.trigger.add", vec![
        ("name", ApocType::String),
        ("statement", ApocType::String),
        ("selector", ApocType::Map),
        ("config", ApocType::Map)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.drop(databaseName STRING, name STRING)
    ("apoc.trigger.drop", vec![
        ("databaseName", ApocType::String),
        ("name", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.dropAll(databaseName STRING)
    ("apoc.trigger.dropAll", vec![
        ("databaseName", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.install(databaseName STRING, name STRING, statement STRING, selector MAP<STRING, ANY>, config MAP<STRING, ANY>)
    ("apoc.trigger.install", vec![
        ("databaseName", ApocType::String),
        ("name", ApocType::String),
        ("statement", ApocType::String),
        ("selector", ApocType::Map),
        ("config", ApocType::Map)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.list()
    ("apoc.trigger.list", vec![], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.pause(name STRING) - Deprecated in Cypher 5
    ("apoc.trigger.pause", vec![
        ("name", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.remove(name STRING) - Deprecated in Cypher 5
    ("apoc.trigger.remove", vec![
        ("name", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.removeAll() - Deprecated in Cypher 5
    ("apoc.trigger.removeAll", vec![], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.resume(name STRING) - Deprecated in Cypher 5
    ("apoc.trigger.resume", vec![
        ("name", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.show(databaseName STRING)
    ("apoc.trigger.show", vec![
        ("databaseName", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.start(databaseName STRING, name STRING)
    ("apoc.trigger.start", vec![
        ("databaseName", ApocType::String),
        ("name", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.trigger.stop(databaseName STRING, name STRING)
    ("apoc.trigger.stop", vec![
        ("databaseName", ApocType::String),
        ("name", ApocType::String)
    ], vec![("result", ApocType::Any)]),
];

// TODO: Implement trigger procedure validation
// This will be implemented once we reference the APOC documentation
pub fn validate_trigger_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement trigger procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_add_signature() {
        let procedure = TRIGGER_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.trigger.add")
            .expect("apoc.trigger.add should be defined");
        
        assert_eq!(procedure.1.len(), 4); // 4 parameters
        assert_eq!(procedure.1[0].0, "name");
        assert_eq!(procedure.1[0].1, ApocType::String);
        assert_eq!(procedure.1[1].0, "statement");
        assert_eq!(procedure.1[1].1, ApocType::String);
        assert_eq!(procedure.1[2].0, "selector");
        assert_eq!(procedure.1[2].1, ApocType::Map);
        assert_eq!(procedure.1[3].0, "config");
        assert_eq!(procedure.1[3].1, ApocType::Map);
    }

    #[test]
    fn test_trigger_list_signature() {
        let procedure = TRIGGER_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.trigger.list")
            .expect("apoc.trigger.list should be defined");
        
        assert_eq!(procedure.1.len(), 0); // 0 parameters
    }

    #[test]
    fn test_trigger_install_signature() {
        let procedure = TRIGGER_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.trigger.install")
            .expect("apoc.trigger.install should be defined");
        
        assert_eq!(procedure.1.len(), 5); // 5 parameters
        assert_eq!(procedure.1[0].0, "databaseName");
        assert_eq!(procedure.1[0].1, ApocType::String);
        assert_eq!(procedure.1[1].0, "name");
        assert_eq!(procedure.1[1].1, ApocType::String);
        assert_eq!(procedure.1[2].0, "statement");
        assert_eq!(procedure.1[2].1, ApocType::String);
        assert_eq!(procedure.1[3].0, "selector");
        assert_eq!(procedure.1[3].1, ApocType::Map);
        assert_eq!(procedure.1[4].0, "config");
        assert_eq!(procedure.1[4].1, ApocType::Map);
    }

    #[test]
    fn test_all_trigger_procedures_have_signatures() {
        assert!(!TRIGGER_PROCEDURES.is_empty(), "Should have at least one trigger procedure");
        
        for (name, args, yields) in TRIGGER_PROCEDURES {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
        }
    }
}


