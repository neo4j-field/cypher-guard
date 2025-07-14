// APOC help procedures
// Handles apoc.help.* procedures for help operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC help procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static HELP_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.help(text STRING)
        ("apoc.help", vec![
            ("text", ApocType::String)
        ], vec![("name", ApocType::String), ("text", ApocType::String)]),
        
        // apoc.help.procedure(name STRING)
        ("apoc.help.procedure", vec![
            ("name", ApocType::String)
        ], vec![("name", ApocType::String), ("signature", ApocType::String)]),
        
        // apoc.help.function(name STRING)
        ("apoc.help.function", vec![
            ("name", ApocType::String)
        ], vec![("name", ApocType::String), ("signature", ApocType::String)]),
    ]
});

pub fn get_all_help_procedures() -> &'static [ProcedureSignature] {
    &HELP_PROCEDURES
}

// TODO: Implement help procedure validation
pub fn validate_help_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement help procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_signature() {
        let procedures = get_all_help_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.help")
            .expect("apoc.help should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "text");
    }

    #[test]
    fn test_help_procedure_signature() {
        let procedures = get_all_help_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.help.procedure")
            .expect("apoc.help.procedure should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "name");
    }

    #[test]
    fn test_help_function_signature() {
        let procedures = get_all_help_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.help.function")
            .expect("apoc.help.function should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "name");
    }

    #[test]
    fn test_all_help_procedures_have_signatures() {
        let procedures = get_all_help_procedures();
        assert!(!procedures.is_empty(), "Should have at least one help procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
} 