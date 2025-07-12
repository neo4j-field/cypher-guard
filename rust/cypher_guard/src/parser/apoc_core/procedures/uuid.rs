// APOC UUID procedures
// Handles apoc.uuid.* procedures for UUID operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC UUID procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static UUID_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.uuid.generate()
        ("apoc.uuid.generate", vec![], vec![("uuid", ApocType::String)]),
        
        // apoc.uuid.format(uuid STRING, format STRING)
        ("apoc.uuid.format", vec![
            ("uuid", ApocType::String),
            ("format", ApocType::String)
        ], vec![("formatted", ApocType::String)]),
        
        // apoc.uuid.parse(uuid STRING)
        ("apoc.uuid.parse", vec![
            ("uuid", ApocType::String)
        ], vec![("parsed", ApocType::Map)]),
    ]
});

pub fn get_all_uuid_procedures() -> &'static [ProcedureSignature] {
    &UUID_PROCEDURES
}

// TODO: Implement UUID procedure validation
pub fn validate_uuid_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement UUID procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_generate_signature() {
        let procedures = get_all_uuid_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.uuid.generate")
            .expect("apoc.uuid.generate should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_uuid_format_signature() {
        let procedures = get_all_uuid_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.uuid.format")
            .expect("apoc.uuid.format should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "uuid");
        assert_eq!(procedure.1[1].0, "format");
    }

    #[test]
    fn test_uuid_parse_signature() {
        let procedures = get_all_uuid_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.uuid.parse")
            .expect("apoc.uuid.parse should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "uuid");
    }

    #[test]
    fn test_all_uuid_procedures_have_signatures() {
        let procedures = get_all_uuid_procedures();
        assert!(!procedures.is_empty(), "Should have at least one UUID procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


