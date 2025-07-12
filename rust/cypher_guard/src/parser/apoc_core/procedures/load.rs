// APOC load procedures
// Handles apoc.load.* procedures for load operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC load procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static LOAD_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.load.json(url STRING, path STRING, config MAP)
        ("apoc.load.json", vec![
            ("url", ApocType::String),
            ("path", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.load.csv(url STRING, config MAP)
        ("apoc.load.csv", vec![
            ("url", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("value", ApocType::Map)]),
        
        // apoc.load.xml(url STRING, path STRING, config MAP)
        ("apoc.load.xml", vec![
            ("url", ApocType::String),
            ("path", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
    ]
});

pub fn get_all_load_procedures() -> &'static [ProcedureSignature] {
    &LOAD_PROCEDURES
}

// TODO: Implement load procedure validation
pub fn validate_load_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement load procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_json_signature() {
        let procedures = get_all_load_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.load.json")
            .expect("apoc.load.json should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "url");
        assert_eq!(procedure.1[1].0, "path");
        assert_eq!(procedure.1[2].0, "config");
    }

    #[test]
    fn test_load_csv_signature() {
        let procedures = get_all_load_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.load.csv")
            .expect("apoc.load.csv should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "url");
        assert_eq!(procedure.1[1].0, "config");
    }

    #[test]
    fn test_load_xml_signature() {
        let procedures = get_all_load_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.load.xml")
            .expect("apoc.load.xml should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "url");
        assert_eq!(procedure.1[1].0, "path");
        assert_eq!(procedure.1[2].0, "config");
    }

    #[test]
    fn test_all_load_procedures_have_signatures() {
        let procedures = get_all_load_procedures();
        assert!(!procedures.is_empty(), "Should have at least one load procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
} 