// APOC graph procedures
// Handles apoc.graph.* procedures for graph operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC graph procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static GRAPH_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.graph.fromDocument(document ANY, idProperty STRING, write BOOLEAN)
        ("apoc.graph.fromDocument", vec![
            ("document", ApocType::Any),
            ("idProperty", ApocType::String),
            ("write", ApocType::Boolean)
        ], vec![("graph", ApocType::Map)]),
        
        // apoc.graph.fromData(data LIST<MAP>, idProperty STRING, write BOOLEAN)
        ("apoc.graph.fromData", vec![
            ("data", ApocType::List),
            ("idProperty", ApocType::String),
            ("write", ApocType::Boolean)
        ], vec![("graph", ApocType::Map)]),
        
        // apoc.graph.fromPaths(paths LIST<PATH>)
        ("apoc.graph.fromPaths", vec![
            ("paths", ApocType::List)
        ], vec![("graph", ApocType::Map)]),
        
        // apoc.graph.fromCypher(cypher STRING, params MAP)
        ("apoc.graph.fromCypher", vec![
            ("cypher", ApocType::String),
            ("params", ApocType::Map)
        ], vec![("graph", ApocType::Map)]),
        
        // apoc.graph.toCypher(graph MAP, config MAP)
        ("apoc.graph.toCypher", vec![
            ("graph", ApocType::Map),
            ("config", ApocType::Map)
        ], vec![("cypher", ApocType::String)]),
        
        // apoc.graph.merge(graph MAP, config MAP)
        ("apoc.graph.merge", vec![
            ("graph", ApocType::Map),
            ("config", ApocType::Map)
        ], vec![("graph", ApocType::Map)]),
    ]
});

pub fn get_all_graph_procedures() -> &'static [ProcedureSignature] {
    &GRAPH_PROCEDURES
}

// TODO: Implement graph procedure validation
pub fn validate_graph_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement graph procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_from_document_signature() {
        let procedures = get_all_graph_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.graph.fromDocument")
            .expect("apoc.graph.fromDocument should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "document");
        assert_eq!(procedure.1[1].0, "idProperty");
        assert_eq!(procedure.1[2].0, "write");
    }

    #[test]
    fn test_graph_from_data_signature() {
        let procedures = get_all_graph_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.graph.fromData")
            .expect("apoc.graph.fromData should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "data");
        assert_eq!(procedure.1[1].0, "idProperty");
        assert_eq!(procedure.1[2].0, "write");
    }

    #[test]
    fn test_graph_from_paths_signature() {
        let procedures = get_all_graph_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.graph.fromPaths")
            .expect("apoc.graph.fromPaths should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "paths");
    }

    #[test]
    fn test_graph_from_cypher_signature() {
        let procedures = get_all_graph_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.graph.fromCypher")
            .expect("apoc.graph.fromCypher should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "cypher");
        assert_eq!(procedure.1[1].0, "params");
    }

    #[test]
    fn test_all_graph_procedures_have_signatures() {
        let procedures = get_all_graph_procedures();
        assert!(!procedures.is_empty(), "Should have at least one graph procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


