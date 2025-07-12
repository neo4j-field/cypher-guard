// APOC number procedures
// Handles apoc.number.* procedures for number operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC number procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static NUMBER_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.number.format(number FLOAT, pattern STRING)
        ("apoc.number.format", vec![
            ("number", ApocType::Float),
            ("pattern", ApocType::String)
        ], vec![("value", ApocType::String)]),
        
        // apoc.number.parseFloat(text STRING)
        ("apoc.number.parseFloat", vec![
            ("text", ApocType::String)
        ], vec![("value", ApocType::Float)]),
        
        // apoc.number.parseInt(text STRING)
        ("apoc.number.parseInt", vec![
            ("text", ApocType::String)
        ], vec![("value", ApocType::Integer)]),
        
        // apoc.number.exact.add(a FLOAT, b FLOAT)
        ("apoc.number.exact.add", vec![
            ("a", ApocType::Float),
            ("b", ApocType::Float)
        ], vec![("value", ApocType::Float)]),
        
        // apoc.number.exact.sub(a FLOAT, b FLOAT)
        ("apoc.number.exact.sub", vec![
            ("a", ApocType::Float),
            ("b", ApocType::Float)
        ], vec![("value", ApocType::Float)]),
    ]
});

pub fn get_all_number_procedures() -> &'static [ProcedureSignature] {
    &NUMBER_PROCEDURES
}

// TODO: Implement number procedure validation
pub fn validate_number_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement number procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_format_signature() {
        let procedures = get_all_number_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.number.format")
            .expect("apoc.number.format should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "number");
        assert_eq!(procedure.1[1].0, "pattern");
    }

    #[test]
    fn test_number_parse_float_signature() {
        let procedures = get_all_number_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.number.parseFloat")
            .expect("apoc.number.parseFloat should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "text");
    }

    #[test]
    fn test_number_parse_int_signature() {
        let procedures = get_all_number_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.number.parseInt")
            .expect("apoc.number.parseInt should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "text");
    }

    #[test]
    fn test_number_exact_add_signature() {
        let procedures = get_all_number_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.number.exact.add")
            .expect("apoc.number.exact.add should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "a");
        assert_eq!(procedure.1[1].0, "b");
    }

    #[test]
    fn test_all_number_procedures_have_signatures() {
        let procedures = get_all_number_procedures();
        assert!(!procedures.is_empty(), "Should have at least one number procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
} 