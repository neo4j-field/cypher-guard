// APOC generate procedures
// Handles apoc.generate.* procedures for generation operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC generate procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static GENERATE_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.generate.ba(nodes INTEGER, edges INTEGER, labels LIST<STRING>)
        ("apoc.generate.ba", vec![
            ("nodes", ApocType::Integer),
            ("edges", ApocType::Integer),
            ("labels", ApocType::List)
        ], vec![("value", ApocType::Map)]),
        
        // apoc.generate.er(nodes INTEGER, edges INTEGER, labels LIST<STRING>)
        ("apoc.generate.er", vec![
            ("nodes", ApocType::Integer),
            ("edges", ApocType::Integer),
            ("labels", ApocType::List)
        ], vec![("value", ApocType::Map)]),
        
        // apoc.generate.ws(nodes INTEGER, edges INTEGER, labels LIST<STRING>)
        ("apoc.generate.ws", vec![
            ("nodes", ApocType::Integer),
            ("edges", ApocType::Integer),
            ("labels", ApocType::List)
        ], vec![("value", ApocType::Map)]),
    ]
});

pub fn get_all_generate_procedures() -> &'static [ProcedureSignature] {
    &GENERATE_PROCEDURES
}

// TODO: Implement generate procedure validation
pub fn validate_generate_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement generate procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ba_signature() {
        let procedures = get_all_generate_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.generate.ba")
            .expect("apoc.generate.ba should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "nodes");
        assert_eq!(procedure.1[1].0, "edges");
        assert_eq!(procedure.1[2].0, "labels");
    }

    #[test]
    fn test_generate_er_signature() {
        let procedures = get_all_generate_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.generate.er")
            .expect("apoc.generate.er should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "nodes");
        assert_eq!(procedure.1[1].0, "edges");
        assert_eq!(procedure.1[2].0, "labels");
    }

    #[test]
    fn test_generate_ws_signature() {
        let procedures = get_all_generate_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.generate.ws")
            .expect("apoc.generate.ws should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "nodes");
        assert_eq!(procedure.1[1].0, "edges");
        assert_eq!(procedure.1[2].0, "labels");
    }

    #[test]
    fn test_all_generate_procedures_have_signatures() {
        let procedures = get_all_generate_procedures();
        assert!(!procedures.is_empty(), "Should have at least one generate procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


