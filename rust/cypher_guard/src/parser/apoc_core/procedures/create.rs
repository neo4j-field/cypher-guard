// APOC create procedures
// Handles apoc.create.* procedures for creation operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC create procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static CREATE_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.create.addLabels(node NODE, labels LIST<STRING>)
        ("apoc.create.addLabels", vec![
            ("node", ApocType::Node),
            ("labels", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.clonePathToVirtual(path PATH)
        ("apoc.create.clonePathToVirtual", vec![
            ("path", ApocType::Path)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.clonePathsToVirtual(paths LIST<PATH>)
        ("apoc.create.clonePathsToVirtual", vec![
            ("paths", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.node(labels LIST<STRING>, properties MAP<STRING, ANY>)
        ("apoc.create.node", vec![
            ("labels", ApocType::List),
            ("properties", ApocType::Map)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.nodes(labels LIST<STRING>, properties LIST<MAP<STRING, ANY>>)
        ("apoc.create.nodes", vec![
            ("labels", ApocType::List),
            ("properties", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.relationship(startNode NODE, endNode NODE, type STRING, properties MAP<STRING, ANY>)
        ("apoc.create.relationship", vec![
            ("startNode", ApocType::Node),
            ("endNode", ApocType::Node),
            ("type", ApocType::String),
            ("properties", ApocType::Map)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.removeLabels(node NODE, labels LIST<STRING>)
        ("apoc.create.removeLabels", vec![
            ("node", ApocType::Node),
            ("labels", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.removeProperties(node NODE, keys LIST<STRING>)
        ("apoc.create.removeProperties", vec![
            ("node", ApocType::Node),
            ("keys", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.removeRelProperties(relationship RELATIONSHIP, keys LIST<STRING>)
        ("apoc.create.removeRelProperties", vec![
            ("relationship", ApocType::Relationship),
            ("keys", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.setLabels(node NODE, labels LIST<STRING>)
        ("apoc.create.setLabels", vec![
            ("node", ApocType::Node),
            ("labels", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.setProperties(node NODE, properties MAP<STRING, ANY>)
        ("apoc.create.setProperties", vec![
            ("node", ApocType::Node),
            ("properties", ApocType::Map)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.setProperty(node NODE, key STRING, value ANY)
        ("apoc.create.setProperty", vec![
            ("node", ApocType::Node),
            ("key", ApocType::String),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.setRelProperties(relationship RELATIONSHIP, properties MAP<STRING, ANY>)
        ("apoc.create.setRelProperties", vec![
            ("relationship", ApocType::Relationship),
            ("properties", ApocType::Map)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.setRelProperty(relationship RELATIONSHIP, key STRING, value ANY)
        ("apoc.create.setRelProperty", vec![
            ("relationship", ApocType::Relationship),
            ("key", ApocType::String),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.uuids(count INTEGER)
        ("apoc.create.uuids", vec![
            ("count", ApocType::Integer)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.vNode(labels LIST<STRING>, properties MAP<STRING, ANY>)
        ("apoc.create.vNode", vec![
            ("labels", ApocType::List),
            ("properties", ApocType::Map)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.vNodes(labels LIST<STRING>, properties LIST<MAP<STRING, ANY>>)
        ("apoc.create.vNodes", vec![
            ("labels", ApocType::List),
            ("properties", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.vRelationship(startNode NODE, endNode NODE, type STRING, properties MAP<STRING, ANY>)
        ("apoc.create.vRelationship", vec![
            ("startNode", ApocType::Node),
            ("endNode", ApocType::Node),
            ("type", ApocType::String),
            ("properties", ApocType::Map)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.virtualPath(startNode NODE, relationships LIST<RELATIONSHIP>, endNode NODE)
        ("apoc.create.virtualPath", vec![
            ("startNode", ApocType::Node),
            ("relationships", ApocType::List),
            ("endNode", ApocType::Node)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.uuid()
        ("apoc.create.uuid", vec![], vec![("result", ApocType::Any)]),
        
        // apoc.create.uuidBase64()
        ("apoc.create.uuidBase64", vec![], vec![("result", ApocType::Any)]),
        
        // apoc.create.uuidBase64ToHex(base64 STRING)
        ("apoc.create.uuidBase64ToHex", vec![
            ("base64", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.uuidHexToBase64(hex STRING)
        ("apoc.create.uuidHexToBase64", vec![
            ("hex", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.create.virtual.fromNode(node NODE)
        ("apoc.create.virtual.fromNode", vec![
            ("node", ApocType::Node)
        ], vec![("result", ApocType::Any)]),
    ]
});

pub fn get_all_create_procedures() -> &'static [ProcedureSignature] {
    &CREATE_PROCEDURES
}

// TODO: Implement create procedure validation
pub fn validate_create_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement create procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_node_signature() {
        let procedures = get_all_create_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.create.node")
            .expect("apoc.create.node should be defined");
        
        assert_eq!(procedure.1.len(), 2); // 2 parameters
        assert_eq!(procedure.1[0].0, "labels");
        assert_eq!(procedure.1[0].1, ApocType::List);
        assert_eq!(procedure.1[1].0, "properties");
        assert_eq!(procedure.1[1].1, ApocType::Map);
    }

    #[test]
    fn test_create_relationship_signature() {
        let procedures = get_all_create_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.create.relationship")
            .expect("apoc.create.relationship should be defined");
        
        assert_eq!(procedure.1.len(), 4); // 4 parameters
        assert_eq!(procedure.1[0].0, "startNode");
        assert_eq!(procedure.1[0].1, ApocType::Node);
        assert_eq!(procedure.1[1].0, "endNode");
        assert_eq!(procedure.1[1].1, ApocType::Node);
        assert_eq!(procedure.1[2].0, "type");
        assert_eq!(procedure.1[2].1, ApocType::String);
        assert_eq!(procedure.1[3].0, "properties");
        assert_eq!(procedure.1[3].1, ApocType::Map);
    }

    #[test]
    fn test_create_uuid_signature() {
        let procedures = get_all_create_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.create.uuid")
            .expect("apoc.create.uuid should be defined");
        
        assert_eq!(procedure.1.len(), 0); // 0 parameters
    }

    #[test]
    fn test_all_create_procedures_have_signatures() {
        let procedures = get_all_create_procedures();
        assert!(!procedures.is_empty(), "Should have at least one create procedure");
        
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
        }
    }
}


