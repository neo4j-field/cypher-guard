// APOC export procedures
// Handles apoc.export.* procedures for export operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC export procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static EXPORT_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.export.csv.query(query STRING, file STRING, config MAP)
        ("apoc.export.csv.query", vec![
            ("query", ApocType::String),
            ("file", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("file", ApocType::String), ("source", ApocType::String), ("format", ApocType::String), ("nodes", ApocType::Integer), ("relationships", ApocType::Integer), ("properties", ApocType::Integer), ("time", ApocType::Integer), ("rows", ApocType::Integer)]),
        
        // apoc.export.csv.all(file STRING, config MAP)
        ("apoc.export.csv.all", vec![
            ("file", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("file", ApocType::String), ("source", ApocType::String), ("format", ApocType::String), ("nodes", ApocType::Integer), ("relationships", ApocType::Integer), ("properties", ApocType::Integer), ("time", ApocType::Integer), ("rows", ApocType::Integer)]),
        
        // apoc.export.json.query(query STRING, file STRING, config MAP)
        ("apoc.export.json.query", vec![
            ("query", ApocType::String),
            ("file", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("file", ApocType::String), ("source", ApocType::String), ("format", ApocType::String), ("nodes", ApocType::Integer), ("relationships", ApocType::Integer), ("properties", ApocType::Integer), ("time", ApocType::Integer), ("rows", ApocType::Integer)]),
        
        // apoc.export.json.all(file STRING, config MAP)
        ("apoc.export.json.all", vec![
            ("file", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("file", ApocType::String), ("source", ApocType::String), ("format", ApocType::String), ("nodes", ApocType::Integer), ("relationships", ApocType::Integer), ("properties", ApocType::Integer), ("time", ApocType::Integer), ("rows", ApocType::Integer)]),
    ]
});

pub fn get_all_export_procedures() -> &'static [ProcedureSignature] {
    &EXPORT_PROCEDURES
}

// TODO: Implement export procedure validation
pub fn validate_export_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement export procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_csv_query_signature() {
        let procedures = get_all_export_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.export.csv.query")
            .expect("apoc.export.csv.query should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "query");
        assert_eq!(procedure.1[1].0, "file");
        assert_eq!(procedure.1[2].0, "config");
    }

    #[test]
    fn test_export_csv_all_signature() {
        let procedures = get_all_export_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.export.csv.all")
            .expect("apoc.export.csv.all should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "file");
        assert_eq!(procedure.1[1].0, "config");
    }

    #[test]
    fn test_export_json_query_signature() {
        let procedures = get_all_export_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.export.json.query")
            .expect("apoc.export.json.query should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "query");
        assert_eq!(procedure.1[1].0, "file");
        assert_eq!(procedure.1[2].0, "config");
    }

    #[test]
    fn test_export_json_all_signature() {
        let procedures = get_all_export_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.export.json.all")
            .expect("apoc.export.json.all should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "file");
        assert_eq!(procedure.1[1].0, "config");
    }

    #[test]
    fn test_all_export_procedures_have_signatures() {
        let procedures = get_all_export_procedures();
        assert!(!procedures.is_empty(), "Should have at least one export procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


