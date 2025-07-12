// APOC cypher procedures
// Handles apoc.cypher.* procedures for dynamic Cypher execution

use crate::parser::ast::PropertyValue;
use crate::parser::apoc::types::{ApocType, ProcedureSignature};

// APOC cypher procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
pub const CYPHER_PROCEDURES: &[ProcedureSignature] = &[
    // apoc.cypher.doIt(query STRING, params MAP<STRING, ANY>)
    ("apoc.cypher.doIt", vec![
        ("query", ApocType::String),
        ("params", ApocType::Map)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.cypher.run(query STRING, params MAP<STRING, ANY>)
    ("apoc.cypher.run", vec![
        ("query", ApocType::String),
        ("params", ApocType::Map)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.cypher.runMany(queries LIST<STRING>, params LIST<MAP<STRING, ANY>>)
    ("apoc.cypher.runMany", vec![
        ("queries", ApocType::List),
        ("params", ApocType::List)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.cypher.runManyReadOnly(queries LIST<STRING>, params LIST<MAP<STRING, ANY>>)
    ("apoc.cypher.runManyReadOnly", vec![
        ("queries", ApocType::List),
        ("params", ApocType::List)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.cypher.runSchema(query STRING, params MAP<STRING, ANY>)
    ("apoc.cypher.runSchema", vec![
        ("query", ApocType::String),
        ("params", ApocType::Map)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.cypher.runTimeboxed(query STRING, params MAP<STRING, ANY>, timeout INTEGER)
    ("apoc.cypher.runTimeboxed", vec![
        ("query", ApocType::String),
        ("params", ApocType::Map),
        ("timeout", ApocType::Integer)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.cypher.runWrite(query STRING, params MAP<STRING, ANY>)
    ("apoc.cypher.runWrite", vec![
        ("query", ApocType::String),
        ("params", ApocType::Map)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.cypher.runFirstColumnMany(query STRING, params MAP<STRING, ANY>)
    ("apoc.cypher.runFirstColumnMany", vec![
        ("query", ApocType::String),
        ("params", ApocType::Map)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.cypher.runFirstColumnSingle(query STRING, params MAP<STRING, ANY>)
    ("apoc.cypher.runFirstColumnSingle", vec![
        ("query", ApocType::String),
        ("params", ApocType::Map)
    ], vec![("result", ApocType::Any)]),
];

// TODO: Implement cypher procedure validation
// This will be implemented once we reference the APOC documentation
pub fn validate_cypher_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement cypher procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cypher_doit_signature() {
        let procedure = CYPHER_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.cypher.doIt")
            .expect("apoc.cypher.doIt should be defined");
        
        assert_eq!(procedure.1.len(), 2); // 2 parameters
        assert_eq!(procedure.1[0].0, "query");
        assert_eq!(procedure.1[0].1, ApocType::String);
        assert_eq!(procedure.1[1].0, "params");
        assert_eq!(procedure.1[1].1, ApocType::Map);
    }

    #[test]
    fn test_cypher_run_signature() {
        let procedure = CYPHER_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.cypher.run")
            .expect("apoc.cypher.run should be defined");
        
        assert_eq!(procedure.1.len(), 2); // 2 parameters
        assert_eq!(procedure.1[0].0, "query");
        assert_eq!(procedure.1[0].1, ApocType::String);
        assert_eq!(procedure.1[1].0, "params");
        assert_eq!(procedure.1[1].1, ApocType::Map);
    }

    #[test]
    fn test_cypher_run_timeboxed_signature() {
        let procedure = CYPHER_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.cypher.runTimeboxed")
            .expect("apoc.cypher.runTimeboxed should be defined");
        
        assert_eq!(procedure.1.len(), 3); // 3 parameters
        assert_eq!(procedure.1[0].0, "query");
        assert_eq!(procedure.1[0].1, ApocType::String);
        assert_eq!(procedure.1[1].0, "params");
        assert_eq!(procedure.1[1].1, ApocType::Map);
        assert_eq!(procedure.1[2].0, "timeout");
        assert_eq!(procedure.1[2].1, ApocType::Integer);
    }

    #[test]
    fn test_all_cypher_procedures_have_signatures() {
        assert!(!CYPHER_PROCEDURES.is_empty(), "Should have at least one cypher procedure");
        
        for (name, args, yields) in CYPHER_PROCEDURES {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
        }
    }
} 