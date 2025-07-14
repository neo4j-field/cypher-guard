// APOC date procedures
// Handles apoc.date.* procedures for date operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC date procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static DATE_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.date.format(timestamp INTEGER, unit STRING, format STRING, timezone STRING)
        ("apoc.date.format", vec![
            ("timestamp", ApocType::Integer),
            ("unit", ApocType::String),
            ("format", ApocType::String),
            ("timezone", ApocType::String)
        ], vec![("value", ApocType::String)]),
        
        // apoc.date.parse(time STRING, unit STRING, format STRING, timezone STRING)
        ("apoc.date.parse", vec![
            ("time", ApocType::String),
            ("unit", ApocType::String),
            ("format", ApocType::String),
            ("timezone", ApocType::String)
        ], vec![("value", ApocType::Integer)]),
        
        // apoc.date.fields(date STRING, format STRING, timezone STRING)
        ("apoc.date.fields", vec![
            ("date", ApocType::String),
            ("format", ApocType::String),
            ("timezone", ApocType::String)
        ], vec![("value", ApocType::Map)]),
        
        // apoc.date.add(timestamp INTEGER, unit STRING, addValue INTEGER, timezone STRING)
        ("apoc.date.add", vec![
            ("timestamp", ApocType::Integer),
            ("unit", ApocType::String),
            ("addValue", ApocType::Integer),
            ("timezone", ApocType::String)
        ], vec![("value", ApocType::Integer)]),
        
        // apoc.date.subtract(timestamp INTEGER, unit STRING, subtractValue INTEGER, timezone STRING)
        ("apoc.date.subtract", vec![
            ("timestamp", ApocType::Integer),
            ("unit", ApocType::String),
            ("subtractValue", ApocType::Integer),
            ("timezone", ApocType::String)
        ], vec![("value", ApocType::Integer)]),
        
        // apoc.date.diff(timestamp1 INTEGER, timestamp2 INTEGER, unit STRING, timezone STRING)
        ("apoc.date.diff", vec![
            ("timestamp1", ApocType::Integer),
            ("timestamp2", ApocType::Integer),
            ("unit", ApocType::String),
            ("timezone", ApocType::String)
        ], vec![("value", ApocType::Integer)]),
        
        // apoc.date.systemTimezone()
        ("apoc.date.systemTimezone", vec![], vec![("value", ApocType::String)]),
        
        // apoc.date.currentTimestamp()
        ("apoc.date.currentTimestamp", vec![], vec![("value", ApocType::Integer)]),
    ]
});

pub fn get_all_date_procedures() -> &'static [ProcedureSignature] {
    &DATE_PROCEDURES
}

// TODO: Implement date procedure validation
pub fn validate_date_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement date procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_format_signature() {
        let procedures = get_all_date_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.date.format")
            .expect("apoc.date.format should be defined");
        assert_eq!(procedure.1.len(), 4);
        assert_eq!(procedure.1[0].0, "timestamp");
        assert_eq!(procedure.1[1].0, "unit");
        assert_eq!(procedure.1[2].0, "format");
        assert_eq!(procedure.1[3].0, "timezone");
    }

    #[test]
    fn test_date_parse_signature() {
        let procedures = get_all_date_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.date.parse")
            .expect("apoc.date.parse should be defined");
        assert_eq!(procedure.1.len(), 4);
        assert_eq!(procedure.1[0].0, "time");
        assert_eq!(procedure.1[1].0, "unit");
        assert_eq!(procedure.1[2].0, "format");
        assert_eq!(procedure.1[3].0, "timezone");
    }

    #[test]
    fn test_date_fields_signature() {
        let procedures = get_all_date_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.date.fields")
            .expect("apoc.date.fields should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "date");
        assert_eq!(procedure.1[1].0, "format");
        assert_eq!(procedure.1[2].0, "timezone");
    }

    #[test]
    fn test_date_add_signature() {
        let procedures = get_all_date_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.date.add")
            .expect("apoc.date.add should be defined");
        assert_eq!(procedure.1.len(), 4);
        assert_eq!(procedure.1[0].0, "timestamp");
        assert_eq!(procedure.1[1].0, "unit");
        assert_eq!(procedure.1[2].0, "addValue");
        assert_eq!(procedure.1[3].0, "timezone");
    }

    #[test]
    fn test_date_subtract_signature() {
        let procedures = get_all_date_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.date.subtract")
            .expect("apoc.date.subtract should be defined");
        assert_eq!(procedure.1.len(), 4);
        assert_eq!(procedure.1[0].0, "timestamp");
        assert_eq!(procedure.1[1].0, "unit");
        assert_eq!(procedure.1[2].0, "subtractValue");
        assert_eq!(procedure.1[3].0, "timezone");
    }

    #[test]
    fn test_date_diff_signature() {
        let procedures = get_all_date_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.date.diff")
            .expect("apoc.date.diff should be defined");
        assert_eq!(procedure.1.len(), 4);
        assert_eq!(procedure.1[0].0, "timestamp1");
        assert_eq!(procedure.1[1].0, "timestamp2");
        assert_eq!(procedure.1[2].0, "unit");
        assert_eq!(procedure.1[3].0, "timezone");
    }

    #[test]
    fn test_date_system_timezone_signature() {
        let procedures = get_all_date_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.date.systemTimezone")
            .expect("apoc.date.systemTimezone should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_date_current_timestamp_signature() {
        let procedures = get_all_date_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.date.currentTimestamp")
            .expect("apoc.date.currentTimestamp should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_all_date_procedures_have_signatures() {
        let procedures = get_all_date_procedures();
        assert!(!procedures.is_empty(), "Should have at least one date procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


