// APOC monitoring procedures
// Handles apoc.monitoring.* procedures for monitoring operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC monitoring procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static MONITORING_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.monitoring.kernel()
        ("apoc.monitoring.kernel", vec![], vec![("value", ApocType::Map)]),
        
        // apoc.monitoring.store()
        ("apoc.monitoring.store", vec![], vec![("value", ApocType::Map)]),
        
        // apoc.monitoring.transactions()
        ("apoc.monitoring.transactions", vec![], vec![("value", ApocType::Map)]),
        
        // apoc.monitoring.ids()
        ("apoc.monitoring.ids", vec![], vec![("value", ApocType::Map)]),
    ]
});

pub fn get_all_monitoring_procedures() -> &'static [ProcedureSignature] {
    &MONITORING_PROCEDURES
}

// TODO: Implement monitoring procedure validation
pub fn validate_monitoring_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement monitoring procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitoring_kernel_signature() {
        let procedures = get_all_monitoring_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.monitoring.kernel")
            .expect("apoc.monitoring.kernel should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_monitoring_store_signature() {
        let procedures = get_all_monitoring_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.monitoring.store")
            .expect("apoc.monitoring.store should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_monitoring_transactions_signature() {
        let procedures = get_all_monitoring_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.monitoring.transactions")
            .expect("apoc.monitoring.transactions should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_monitoring_ids_signature() {
        let procedures = get_all_monitoring_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.monitoring.ids")
            .expect("apoc.monitoring.ids should be defined");
        assert_eq!(procedure.1.len(), 0);
    }

    #[test]
    fn test_all_monitoring_procedures_have_signatures() {
        let procedures = get_all_monitoring_procedures();
        assert!(!procedures.is_empty(), "Should have at least one monitoring procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


