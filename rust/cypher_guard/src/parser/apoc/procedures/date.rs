// APOC date procedures
// Handles apoc.date.* procedures for date/time operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc::types::{ApocType, ProcedureSignature};

// APOC date procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
pub const DATE_PROCEDURES: &[ProcedureSignature] = &[
    // apoc.date.add(timestamp INTEGER, unit STRING, value INTEGER)
    ("apoc.date.add", vec![
        ("timestamp", ApocType::Integer),
        ("unit", ApocType::String),
        ("value", ApocType::Integer)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.date.convert(timestamp INTEGER, unit STRING, timezone STRING)
    ("apoc.date.convert", vec![
        ("timestamp", ApocType::Integer),
        ("unit", ApocType::String),
        ("timezone", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.date.convertFormat(timestamp INTEGER, format STRING, timezone STRING)
    ("apoc.date.convertFormat", vec![
        ("timestamp", ApocType::Integer),
        ("format", ApocType::String),
        ("timezone", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.date.currentTimestamp()
    ("apoc.date.currentTimestamp", vec![], vec![("result", ApocType::Any)]),
    
    // apoc.date.field(timestamp INTEGER, unit STRING)
    ("apoc.date.field", vec![
        ("timestamp", ApocType::Integer),
        ("unit", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.date.fields(timestamp INTEGER, timezone STRING)
    ("apoc.date.fields", vec![
        ("timestamp", ApocType::Integer),
        ("timezone", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.date.format(timestamp INTEGER, format STRING, timezone STRING)
    ("apoc.date.format", vec![
        ("timestamp", ApocType::Integer),
        ("format", ApocType::String),
        ("timezone", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.date.fromISO8601(timestamp STRING)
    ("apoc.date.fromISO8601", vec![
        ("timestamp", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.date.parse(timestamp STRING, format STRING, timezone STRING)
    ("apoc.date.parse", vec![
        ("timestamp", ApocType::String),
        ("format", ApocType::String),
        ("timezone", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.date.systemTimezone()
    ("apoc.date.systemTimezone", vec![], vec![("result", ApocType::Any)]),
    
    // apoc.date.toISO8601(timestamp INTEGER, timezone STRING)
    ("apoc.date.toISO8601", vec![
        ("timestamp", ApocType::Integer),
        ("timezone", ApocType::String)
    ], vec![("result", ApocType::Any)]),
    
    // apoc.date.toYears(timestamp INTEGER, timezone STRING)
    ("apoc.date.toYears", vec![
        ("timestamp", ApocType::Integer),
        ("timezone", ApocType::String)
    ], vec![("result", ApocType::Any)]),
];

// TODO: Implement date procedure validation
// This will be implemented once we reference the APOC documentation
pub fn validate_date_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement date procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_add_signature() {
        let procedure = DATE_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.date.add")
            .expect("apoc.date.add should be defined");
        
        assert_eq!(procedure.1.len(), 3); // 3 parameters
        assert_eq!(procedure.1[0].0, "timestamp");
        assert_eq!(procedure.1[0].1, ApocType::Integer);
        assert_eq!(procedure.1[1].0, "unit");
        assert_eq!(procedure.1[1].1, ApocType::String);
        assert_eq!(procedure.1[2].0, "value");
        assert_eq!(procedure.1[2].1, ApocType::Integer);
    }

    #[test]
    fn test_date_current_timestamp_signature() {
        let procedure = DATE_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.date.currentTimestamp")
            .expect("apoc.date.currentTimestamp should be defined");
        
        assert_eq!(procedure.1.len(), 0); // 0 parameters
    }

    #[test]
    fn test_date_format_signature() {
        let procedure = DATE_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.date.format")
            .expect("apoc.date.format should be defined");
        
        assert_eq!(procedure.1.len(), 3); // 3 parameters
        assert_eq!(procedure.1[0].0, "timestamp");
        assert_eq!(procedure.1[0].1, ApocType::Integer);
        assert_eq!(procedure.1[1].0, "format");
        assert_eq!(procedure.1[1].1, ApocType::String);
        assert_eq!(procedure.1[2].0, "timezone");
        assert_eq!(procedure.1[2].1, ApocType::String);
    }

    #[test]
    fn test_all_date_procedures_have_signatures() {
        assert!(!DATE_PROCEDURES.is_empty(), "Should have at least one date procedure");
        
        for (name, args, yields) in DATE_PROCEDURES {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
        }
    }
}


