// APOC convert procedures
// Handles apoc.convert.* procedures for data conversion operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC convert procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static CONVERT_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.convert.setJsonProperty(node NODE, key STRING, value ANY)
        ("apoc.convert.setJsonProperty", vec![
            ("node", ApocType::Node),
            ("key", ApocType::String),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.toTree(list LIST<MAP>)
        ("apoc.convert.toTree", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.fromJsonList(json STRING)
        ("apoc.convert.fromJsonList", vec![
            ("json", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.fromJsonMap(json STRING)
        ("apoc.convert.fromJsonMap", vec![
            ("json", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.getJsonProperty(node NODE, key STRING)
        ("apoc.convert.getJsonProperty", vec![
            ("node", ApocType::Node),
            ("key", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.getJsonPropertyMap(node NODE, keys LIST<STRING>)
        ("apoc.convert.getJsonPropertyMap", vec![
            ("node", ApocType::Node),
            ("keys", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.toJson(value ANY)
        ("apoc.convert.toJson", vec![
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.toList(value ANY)
        ("apoc.convert.toList", vec![
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.toMap(value ANY)
        ("apoc.convert.toMap", vec![
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.toNode(value ANY)
        ("apoc.convert.toNode", vec![
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.toNodeList(value ANY)
        ("apoc.convert.toNodeList", vec![
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.toRelationship(value ANY)
        ("apoc.convert.toRelationship", vec![
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.toRelationshipList(value ANY)
        ("apoc.convert.toRelationshipList", vec![
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.toSet(value ANY)
        ("apoc.convert.toSet", vec![
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.convert.toSortedJsonMap(value ANY)
        ("apoc.convert.toSortedJsonMap", vec![
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
    ]
});

pub fn get_all_convert_procedures() -> &'static [ProcedureSignature] {
    &CONVERT_PROCEDURES
}

// TODO: Implement convert procedure validation
pub fn validate_convert_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement convert procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_json_signature() {
        let procedures = get_all_convert_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.convert.toJson")
            .expect("apoc.convert.toJson should be defined");
        
        assert_eq!(procedure.1.len(), 1); // 1 parameter
        assert_eq!(procedure.1[0].0, "value");
        assert_eq!(procedure.1[0].1, ApocType::Any);
    }

    #[test]
    fn test_convert_set_json_property_signature() {
        let procedures = get_all_convert_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.convert.setJsonProperty")
            .expect("apoc.convert.setJsonProperty should be defined");
        
        assert_eq!(procedure.1.len(), 3); // 3 parameters
        assert_eq!(procedure.1[0].0, "node");
        assert_eq!(procedure.1[0].1, ApocType::Node);
        assert_eq!(procedure.1[1].0, "key");
        assert_eq!(procedure.1[1].1, ApocType::String);
        assert_eq!(procedure.1[2].0, "value");
        assert_eq!(procedure.1[2].1, ApocType::Any);
    }

    #[test]
    fn test_convert_get_json_property_map_signature() {
        let procedures = get_all_convert_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.convert.getJsonPropertyMap")
            .expect("apoc.convert.getJsonPropertyMap should be defined");
        
        assert_eq!(procedure.1.len(), 2); // 2 parameters
        assert_eq!(procedure.1[0].0, "node");
        assert_eq!(procedure.1[0].1, ApocType::Node);
        assert_eq!(procedure.1[1].0, "keys");
        assert_eq!(procedure.1[1].1, ApocType::List);
    }

    #[test]
    fn test_all_convert_procedures_have_signatures() {
        let procedures = get_all_convert_procedures();
        assert!(!procedures.is_empty(), "Should have at least one convert procedure");
        
        for (name, args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
            
            // All convert procedures should have at least one parameter
            assert!(!args.is_empty(), "Convert procedures should have at least one parameter");
        }
    }
}


