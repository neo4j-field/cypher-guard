// APOC JSON procedures
// Handles apoc.json.* procedures for JSON operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC JSON procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static JSON_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.json.path(json STRING, path STRING)
        ("apoc.json.path", vec![
            ("json", ApocType::String),
            ("path", ApocType::String)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.json.setProperty(json STRING, path STRING, value ANY)
        ("apoc.json.setProperty", vec![
            ("json", ApocType::String),
            ("path", ApocType::String),
            ("value", ApocType::Any)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.json.removeProperty(json STRING, path STRING)
        ("apoc.json.removeProperty", vec![
            ("json", ApocType::String),
            ("path", ApocType::String)
        ], vec![("value", ApocType::Any)]),
    ]
});

pub fn get_all_json_procedures() -> &'static [ProcedureSignature] {
    &JSON_PROCEDURES
}

// TODO: Implement JSON procedure validation
pub fn validate_json_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement JSON procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_path_signature() {
        let procedures = get_all_json_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.json.path")
            .expect("apoc.json.path should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "json");
        assert_eq!(procedure.1[1].0, "path");
    }

    #[test]
    fn test_json_set_property_signature() {
        let procedures = get_all_json_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.json.setProperty")
            .expect("apoc.json.setProperty should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "json");
        assert_eq!(procedure.1[1].0, "path");
        assert_eq!(procedure.1[2].0, "value");
    }

    #[test]
    fn test_all_json_procedures_have_signatures() {
        let procedures = get_all_json_procedures();
        assert!(!procedures.is_empty(), "Should have at least one JSON procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


