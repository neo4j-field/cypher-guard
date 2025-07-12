// APOC log procedures
// Handles apoc.log.* procedures for logging operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC log procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static LOG_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.log.info(message STRING, params MAP)
        ("apoc.log.info", vec![
            ("message", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Boolean)]),
        
        // apoc.log.warn(message STRING, params MAP)
        ("apoc.log.warn", vec![
            ("message", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Boolean)]),
        
        // apoc.log.error(message STRING, params MAP)
        ("apoc.log.error", vec![
            ("message", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Boolean)]),
    ]
});

pub fn get_all_log_procedures() -> &'static [ProcedureSignature] {
    &LOG_PROCEDURES
}

// TODO: Implement log procedure validation
pub fn validate_log_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement log procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_info_signature() {
        let procedures = get_all_log_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.log.info")
            .expect("apoc.log.info should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "message");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_log_warn_signature() {
        let procedures = get_all_log_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.log.warn")
            .expect("apoc.log.warn should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "message");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_log_error_signature() {
        let procedures = get_all_log_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.log.error")
            .expect("apoc.log.error should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "message");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_all_log_procedures_have_signatures() {
        let procedures = get_all_log_procedures();
        assert!(!procedures.is_empty(), "Should have at least one log procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


