// APOC scheduler procedures
// Handles apoc.scheduler.* procedures for scheduling operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC scheduler procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static SCHEDULER_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.scheduler.fixed(name STRING, cypher STRING, params MAP, interval INTEGER)
        ("apoc.scheduler.fixed", vec![
            ("name", ApocType::String),
            ("cypher", ApocType::String),
            ("params", ApocType::Map),
            ("interval", ApocType::Integer)
        ], vec![("value", ApocType::Boolean)]),
        
        // apoc.scheduler.cron(name STRING, cypher STRING, params MAP, cronExpression STRING)
        ("apoc.scheduler.cron", vec![
            ("name", ApocType::String),
            ("cypher", ApocType::String),
            ("params", ApocType::Map),
            ("cronExpression", ApocType::String)
        ], vec![("value", ApocType::Boolean)]),
        
        // apoc.scheduler.remove(name STRING)
        ("apoc.scheduler.remove", vec![
            ("name", ApocType::String)
        ], vec![("value", ApocType::Boolean)]),
    ]
});

pub fn get_all_scheduler_procedures() -> &'static [ProcedureSignature] {
    &SCHEDULER_PROCEDURES
}

// TODO: Implement scheduler procedure validation
pub fn validate_scheduler_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement scheduler procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_fixed_signature() {
        let procedures = get_all_scheduler_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.scheduler.fixed")
            .expect("apoc.scheduler.fixed should be defined");
        assert_eq!(procedure.1.len(), 4);
        assert_eq!(procedure.1[0].0, "name");
        assert_eq!(procedure.1[1].0, "cypher");
        assert_eq!(procedure.1[2].0, "params");
        assert_eq!(procedure.1[3].0, "interval");
    }

    #[test]
    fn test_scheduler_cron_signature() {
        let procedures = get_all_scheduler_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.scheduler.cron")
            .expect("apoc.scheduler.cron should be defined");
        assert_eq!(procedure.1.len(), 4);
        assert_eq!(procedure.1[0].0, "name");
        assert_eq!(procedure.1[1].0, "cypher");
        assert_eq!(procedure.1[2].0, "params");
        assert_eq!(procedure.1[3].0, "cronExpression");
    }

    #[test]
    fn test_scheduler_remove_signature() {
        let procedures = get_all_scheduler_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.scheduler.remove")
            .expect("apoc.scheduler.remove should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "name");
    }

    #[test]
    fn test_all_scheduler_procedures_have_signatures() {
        let procedures = get_all_scheduler_procedures();
        assert!(!procedures.is_empty(), "Should have at least one scheduler procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


