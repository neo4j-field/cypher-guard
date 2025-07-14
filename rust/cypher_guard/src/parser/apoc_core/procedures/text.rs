// APOC text procedures
// Handles apoc.text.* procedures for text operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC text procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static TEXT_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.text.clean(text STRING)
        ("apoc.text.clean", vec![
            ("text", ApocType::String)
        ], vec![("value", ApocType::String)]),
        
        // apoc.text.compareCleaned(text1 STRING, text2 STRING)
        ("apoc.text.compareCleaned", vec![
            ("text1", ApocType::String),
            ("text2", ApocType::String)
        ], vec![("value", ApocType::Boolean)]),
        
        // apoc.text.fuzzyMatch(text1 STRING, text2 STRING)
        ("apoc.text.fuzzyMatch", vec![
            ("text1", ApocType::String),
            ("text2", ApocType::String)
        ], vec![("value", ApocType::Float)]),
        
        // apoc.text.sorensenDiceSimilarity(text1 STRING, text2 STRING)
        ("apoc.text.sorensenDiceSimilarity", vec![
            ("text1", ApocType::String),
            ("text2", ApocType::String)
        ], vec![("value", ApocType::Float)]),
        
        // apoc.text.jaroWinklerDistance(text1 STRING, text2 STRING)
        ("apoc.text.jaroWinklerDistance", vec![
            ("text1", ApocType::String),
            ("text2", ApocType::String)
        ], vec![("value", ApocType::Float)]),
    ]
});

pub fn get_all_text_procedures() -> &'static [ProcedureSignature] {
    &TEXT_PROCEDURES
}

// TODO: Implement text procedure validation
pub fn validate_text_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement text procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_clean_signature() {
        let procedures = get_all_text_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.text.clean")
            .expect("apoc.text.clean should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "text");
    }

    #[test]
    fn test_text_compare_cleaned_signature() {
        let procedures = get_all_text_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.text.compareCleaned")
            .expect("apoc.text.compareCleaned should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "text1");
        assert_eq!(procedure.1[1].0, "text2");
    }

    #[test]
    fn test_text_fuzzy_match_signature() {
        let procedures = get_all_text_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.text.fuzzyMatch")
            .expect("apoc.text.fuzzyMatch should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "text1");
        assert_eq!(procedure.1[1].0, "text2");
    }

    #[test]
    fn test_all_text_procedures_have_signatures() {
        let procedures = get_all_text_procedures();
        assert!(!procedures.is_empty(), "Should have at least one text procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
} 