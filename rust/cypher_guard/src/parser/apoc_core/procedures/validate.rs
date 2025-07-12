// APOC validate procedures
// Handles apoc.validate.* procedures for validation operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC validate procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static VALIDATE_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.validate.predicate(predicate BOOLEAN, message STRING, params LIST<ANY>)
        ("apoc.validate.predicate", vec![
            ("predicate", ApocType::Boolean),
            ("message", ApocType::String),
            ("params", ApocType::List)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.validate.jsonSchema(json ANY, schema ANY)
        ("apoc.validate.jsonSchema", vec![
            ("json", ApocType::Any),
            ("schema", ApocType::Any)
        ], vec![("value", ApocType::Boolean)]),
        
        // apoc.validate.email(email STRING)
        ("apoc.validate.email", vec![
            ("email", ApocType::String)
        ], vec![("value", ApocType::Boolean)]),
        
        // apoc.validate.url(url STRING)
        ("apoc.validate.url", vec![
            ("url", ApocType::String)
        ], vec![("value", ApocType::Boolean)]),
        
        // apoc.validate.phoneNumber(phoneNumber STRING, countryCode STRING)
        ("apoc.validate.phoneNumber", vec![
            ("phoneNumber", ApocType::String),
            ("countryCode", ApocType::String)
        ], vec![("value", ApocType::Boolean)]),
    ]
});

pub fn get_all_validate_procedures() -> &'static [ProcedureSignature] {
    &VALIDATE_PROCEDURES
}

// TODO: Implement validate procedure validation
pub fn validate_validate_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement validate procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_predicate_signature() {
        let procedures = get_all_validate_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.validate.predicate")
            .expect("apoc.validate.predicate should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "predicate");
        assert_eq!(procedure.1[1].0, "message");
        assert_eq!(procedure.1[2].0, "params");
    }

    #[test]
    fn test_validate_json_schema_signature() {
        let procedures = get_all_validate_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.validate.jsonSchema")
            .expect("apoc.validate.jsonSchema should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "json");
        assert_eq!(procedure.1[1].0, "schema");
    }

    #[test]
    fn test_validate_email_signature() {
        let procedures = get_all_validate_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.validate.email")
            .expect("apoc.validate.email should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "email");
    }

    #[test]
    fn test_validate_url_signature() {
        let procedures = get_all_validate_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.validate.url")
            .expect("apoc.validate.url should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "url");
    }

    #[test]
    fn test_all_validate_procedures_have_signatures() {
        let procedures = get_all_validate_procedures();
        assert!(!procedures.is_empty(), "Should have at least one validate procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
} 