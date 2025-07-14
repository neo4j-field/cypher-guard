// APOC do procedures
// Handles apoc.do.* procedures for do operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC do procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static DO_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.do.when(condition BOOLEAN, ifQuery STRING, elseQuery STRING, params MAP)
        ("apoc.do.when", vec![
            ("condition", ApocType::Boolean),
            ("ifQuery", ApocType::String),
            ("elseQuery", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.do.case(cases LIST<MAP>, elseQuery STRING, params MAP)
        ("apoc.do.case", vec![
            ("cases", ApocType::List),
            ("elseQuery", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.do.until(conditionQuery STRING, actionQuery STRING, params MAP)
        ("apoc.do.until", vec![
            ("conditionQuery", ApocType::String),
            ("actionQuery", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
    ]
});

pub fn get_all_do_procedures() -> &'static [ProcedureSignature] {
    &DO_PROCEDURES
}

// TODO: Implement do procedure validation
pub fn validate_do_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement do procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_when_signature() {
        let procedures = get_all_do_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.do.when")
            .expect("apoc.do.when should be defined");
        assert_eq!(procedure.1.len(), 4);
        assert_eq!(procedure.1[0].0, "condition");
        assert_eq!(procedure.1[1].0, "ifQuery");
        assert_eq!(procedure.1[2].0, "elseQuery");
        assert_eq!(procedure.1[3].0, "params");
    }

    #[test]
    fn test_do_case_signature() {
        let procedures = get_all_do_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.do.case")
            .expect("apoc.do.case should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "cases");
        assert_eq!(procedure.1[1].0, "elseQuery");
        assert_eq!(procedure.1[2].0, "params");
    }

    #[test]
    fn test_do_until_signature() {
        let procedures = get_all_do_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.do.until")
            .expect("apoc.do.until should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "conditionQuery");
        assert_eq!(procedure.1[1].0, "actionQuery");
        assert_eq!(procedure.1[2].0, "params");
    }

    #[test]
    fn test_all_do_procedures_have_signatures() {
        let procedures = get_all_do_procedures();
        assert!(!procedures.is_empty(), "Should have at least one do procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
} 