// APOC atomic procedures
// Handles apoc.atomic.* procedures for atomic operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC atomic procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
pub static ATOMIC_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.atomic.add(node NODE, property STRING, value NUMBER)
        ("apoc.atomic.add", vec![
            ("node", ApocType::Node),
            ("property", ApocType::String),
            ("value", ApocType::Number)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.atomic.concat(node NODE, property STRING, value STRING)
        ("apoc.atomic.concat", vec![
            ("node", ApocType::Node),
            ("property", ApocType::String),
            ("value", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.atomic.insert(node NODE, property STRING, value ANY, position INTEGER)
        ("apoc.atomic.insert", vec![
            ("node", ApocType::Node),
            ("property", ApocType::String),
            ("value", ApocType::Any),
            ("position", ApocType::Integer)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.atomic.remove(node NODE, property STRING, value ANY)
        ("apoc.atomic.remove", vec![
            ("node", ApocType::Node),
            ("property", ApocType::String),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.atomic.subtract(node NODE, property STRING, value NUMBER)
        ("apoc.atomic.subtract", vec![
            ("node", ApocType::Node),
            ("property", ApocType::String),
            ("value", ApocType::Number)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.atomic.update(node NODE, property STRING, value ANY)
        ("apoc.atomic.update", vec![
            ("node", ApocType::Node),
            ("property", ApocType::String),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
    ]
});

// TODO: Implement atomic procedure validation
// This will be implemented once we reference the APOC documentation
pub fn validate_atomic_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement atomic procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_add_signature() {
        let procedure = ATOMIC_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.atomic.add")
            .expect("apoc.atomic.add should be defined");
        
        assert_eq!(procedure.1.len(), 3); // 3 parameters
        assert_eq!(procedure.1[0].0, "node");
        assert_eq!(procedure.1[0].1, ApocType::Node);
        assert_eq!(procedure.1[1].0, "property");
        assert_eq!(procedure.1[1].1, ApocType::String);
        assert_eq!(procedure.1[2].0, "value");
        assert_eq!(procedure.1[2].1, ApocType::Number);
    }

    #[test]
    fn test_atomic_concat_signature() {
        let procedure = ATOMIC_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.atomic.concat")
            .expect("apoc.atomic.concat should be defined");
        
        assert_eq!(procedure.1.len(), 3); // 3 parameters
        assert_eq!(procedure.1[0].0, "node");
        assert_eq!(procedure.1[0].1, ApocType::Node);
        assert_eq!(procedure.1[1].0, "property");
        assert_eq!(procedure.1[1].1, ApocType::String);
        assert_eq!(procedure.1[2].0, "value");
        assert_eq!(procedure.1[2].1, ApocType::String);
    }

    #[test]
    fn test_atomic_insert_signature() {
        let procedure = ATOMIC_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.atomic.insert")
            .expect("apoc.atomic.insert should be defined");
        
        assert_eq!(procedure.1.len(), 4); // 4 parameters
        assert_eq!(procedure.1[0].0, "node");
        assert_eq!(procedure.1[0].1, ApocType::Node);
        assert_eq!(procedure.1[1].0, "property");
        assert_eq!(procedure.1[1].1, ApocType::String);
        assert_eq!(procedure.1[2].0, "value");
        assert_eq!(procedure.1[2].1, ApocType::Any);
        assert_eq!(procedure.1[3].0, "position");
        assert_eq!(procedure.1[3].1, ApocType::Integer);
    }

    #[test]
    fn test_all_atomic_procedures_have_signatures() {
        assert!(!ATOMIC_PROCEDURES.is_empty(), "Should have at least one atomic procedure");
        
        for (name, args, yields) in ATOMIC_PROCEDURES.iter() {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
            
            // All atomic procedures should have at least 3 parameters (node, property, value)
            assert!(args.len() >= 3, "Atomic procedures should have at least 3 parameters");
            assert_eq!(args[0].0, "node", "First parameter should be 'node'");
            assert_eq!(args[0].1, ApocType::Node, "First parameter should be Node type");
            assert_eq!(args[1].0, "property", "Second parameter should be 'property'");
            assert_eq!(args[1].1, ApocType::String, "Second parameter should be String type");
        }
    }
}


