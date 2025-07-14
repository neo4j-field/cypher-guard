// APOC import procedures
// Handles apoc.import.* procedures for import operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC import procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static IMPORT_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.import.csv(url STRING, config MAP)
        ("apoc.import.csv", vec![
            ("url", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("value", ApocType::Map)]),
        
        // apoc.import.json(url STRING, config MAP)
        ("apoc.import.json", vec![
            ("url", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("value", ApocType::Map)]),
        
        // apoc.import.xml(url STRING, config MAP)
        ("apoc.import.xml", vec![
            ("url", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("value", ApocType::Map)]),
    ]
});

pub fn get_all_import_procedures() -> &'static [ProcedureSignature] {
    &IMPORT_PROCEDURES
}

// TODO: Implement import procedure validation
pub fn validate_import_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement import procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_csv_signature() {
        let procedures = get_all_import_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.import.csv")
            .expect("apoc.import.csv should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "url");
        assert_eq!(procedure.1[1].0, "config");
    }

    #[test]
    fn test_import_json_signature() {
        let procedures = get_all_import_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.import.json")
            .expect("apoc.import.json should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "url");
        assert_eq!(procedure.1[1].0, "config");
    }

    #[test]
    fn test_import_xml_signature() {
        let procedures = get_all_import_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.import.xml")
            .expect("apoc.import.xml should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "url");
        assert_eq!(procedure.1[1].0, "config");
    }

    #[test]
    fn test_all_import_procedures_have_signatures() {
        let procedures = get_all_import_procedures();
        assert!(!procedures.is_empty(), "Should have at least one import procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


