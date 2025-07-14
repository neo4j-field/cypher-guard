// APOC map procedures
// Handles apoc.map.* procedures for map operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC map procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static MAP_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.map.merge(first MAP, second MAP)
        ("apoc.map.merge", vec![
            ("first", ApocType::Map),
            ("second", ApocType::Map)
        ], vec![("map", ApocType::Any)]),
        
        // apoc.map.setKey(map MAP, key STRING, value ANY)
        ("apoc.map.setKey", vec![
            ("map", ApocType::Map),
            ("key", ApocType::String),
            ("value", ApocType::Any)
        ], vec![("map", ApocType::Any)]),
        
        // apoc.map.removeKey(map MAP, key STRING)
        ("apoc.map.removeKey", vec![
            ("map", ApocType::Map),
            ("key", ApocType::String)
        ], vec![("map", ApocType::Any)]),
        
        // apoc.map.fromPairs(pairs LIST<LIST<ANY>>)
        ("apoc.map.fromPairs", vec![
            ("pairs", ApocType::List)
        ], vec![("map", ApocType::Any)]),
        
        // apoc.map.fromValues(keys LIST<STRING>, values LIST<ANY>)
        ("apoc.map.fromValues", vec![
            ("keys", ApocType::List),
            ("values", ApocType::List)
        ], vec![("map", ApocType::Any)]),
    ]
});

pub fn get_all_map_procedures() -> &'static [ProcedureSignature] {
    &MAP_PROCEDURES
}

// TODO: Implement map procedure validation
pub fn validate_map_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement map procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_merge_signature() {
        let procedures = get_all_map_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.map.merge")
            .expect("apoc.map.merge should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "first");
        assert_eq!(procedure.1[1].0, "second");
    }

    #[test]
    fn test_map_set_key_signature() {
        let procedures = get_all_map_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.map.setKey")
            .expect("apoc.map.setKey should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "map");
        assert_eq!(procedure.1[1].0, "key");
        assert_eq!(procedure.1[2].0, "value");
    }

    #[test]
    fn test_map_remove_key_signature() {
        let procedures = get_all_map_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.map.removeKey")
            .expect("apoc.map.removeKey should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "map");
        assert_eq!(procedure.1[1].0, "key");
    }

    #[test]
    fn test_map_from_pairs_signature() {
        let procedures = get_all_map_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.map.fromPairs")
            .expect("apoc.map.fromPairs should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "pairs");
    }

    #[test]
    fn test_all_map_procedures_have_signatures() {
        let procedures = get_all_map_procedures();
        assert!(!procedures.is_empty(), "Should have at least one map procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


