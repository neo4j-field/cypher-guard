// APOC XML procedures
// Handles apoc.xml.* procedures for XML operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC XML procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static XML_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.xml.parse(xml STRING, path STRING, config MAP)
        ("apoc.xml.parse", vec![
            ("xml", ApocType::String),
            ("path", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.xml.import(url STRING, config MAP)
        ("apoc.xml.import", vec![
            ("url", ApocType::String),
            ("config", ApocType::Map)
        ], vec![("value", ApocType::Any)]),
        
        // apoc.xml.export(data ANY, config MAP)
        ("apoc.xml.export", vec![
            ("data", ApocType::Any),
            ("config", ApocType::Map)
        ], vec![("value", ApocType::String)]),
    ]
});

pub fn get_all_xml_procedures() -> &'static [ProcedureSignature] {
    &XML_PROCEDURES
}

// TODO: Implement XML procedure validation
pub fn validate_xml_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement XML procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_parse_signature() {
        let procedures = get_all_xml_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.xml.parse")
            .expect("apoc.xml.parse should be defined");
        assert_eq!(procedure.1.len(), 3);
        assert_eq!(procedure.1[0].0, "xml");
        assert_eq!(procedure.1[1].0, "path");
        assert_eq!(procedure.1[2].0, "config");
    }

    #[test]
    fn test_xml_import_signature() {
        let procedures = get_all_xml_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.xml.import")
            .expect("apoc.xml.import should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "url");
        assert_eq!(procedure.1[1].0, "config");
    }

    #[test]
    fn test_xml_export_signature() {
        let procedures = get_all_xml_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.xml.export")
            .expect("apoc.xml.export should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "data");
        assert_eq!(procedure.1[1].0, "config");
    }

    #[test]
    fn test_all_xml_procedures_have_signatures() {
        let procedures = get_all_xml_procedures();
        assert!(!procedures.is_empty(), "Should have at least one XML procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


