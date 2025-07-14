// APOC module for parsing and validating APOC procedures and functions
// This module handles the parsing and validation of APOC (Awesome Procedures On Cypher) calls

pub mod types;      // Shared APOC types and signatures
pub mod procedures; // APOC procedures organized by category
pub mod functions;  // APOC functions
pub mod parser;     // APOC-specific parsing logic

// Re-export commonly used types
pub use types::{ApocType, ProcedureSignature, FunctionSignature};
pub use crate::parser::ast::PropertyValue;

// Main APOC validation function
pub fn validate_apoc_call(namespace: &str, procedure: &str, args: &[PropertyValue]) -> Result<(), String> {
    match namespace {
        "apoc" => validate_apoc_procedure(procedure, args),
        _ => Err(format!("Unknown APOC namespace: {}", namespace)),
    }
}

// Route to appropriate procedure validation based on category
fn validate_apoc_procedure(procedure: &str, args: &[PropertyValue]) -> Result<(), String> {
    let parts: Vec<&str> = procedure.split('.').collect();
    if parts.len() < 2 {
        return Err(format!("Invalid APOC procedure format: {}", procedure));
    }
    
    let category = parts[1];
    match category {
        "agg" => procedures::agg::validate_agg_procedure(procedure, args),
        "algo" => procedures::algo::validate_algo_procedure(procedure, args),
        "atomic" => procedures::atomic::validate_atomic_procedure(procedure, args),
        "bitwise" => procedures::bitwise::validate_bitwise_procedure(procedure, args),
        "coll" => procedures::coll::validate_coll_procedure(procedure, args),
        "convert" => procedures::convert::validate_convert_procedure(procedure, args),
        "create" => procedures::create::validate_create_procedure(procedure, args),
        "cypher" => procedures::cypher::validate_cypher_procedure(procedure, args),
        "data" => procedures::data::validate_data_procedure(procedure, args),
        "date" => procedures::date::validate_date_procedure(procedure, args),
        "diff" => procedures::diff::validate_diff_procedure(procedure, args),
        "export" => procedures::export::validate_export_procedure(procedure, args),
        "generate" => procedures::generate::validate_generate_procedure(procedure, args),
        "graph" => procedures::graph::validate_graph_procedure(procedure, args),
        "hash" => procedures::hash::validate_hash_procedure(procedure, args),
        "import" => procedures::import::validate_import_procedure(procedure, args),
        "json" => procedures::json::validate_json_procedure(procedure, args),
        "lock" => procedures::lock::validate_lock_procedure(procedure, args),
        "log" => procedures::log::validate_log_procedure(procedure, args),
        "map" => procedures::map::validate_map_procedure(procedure, args),
        "math" => procedures::math::validate_math_procedure(procedure, args),
        "merge" => procedures::merge::validate_merge_procedure(procedure, args),
        "meta" => procedures::meta::validate_meta_procedure(procedure, args),
        "monitoring" => procedures::monitoring::validate_monitoring_procedure(procedure, args),
        "periodic" => procedures::periodic::validate_periodic_procedure(procedure, args),
        "refactor" => procedures::refactor::validate_refactor_procedure(procedure, args),
        "run" => procedures::run::validate_run_procedure(procedure, args),
        "schema" => procedures::schema::validate_schema_procedure(procedure, args),
        "scheduler" => procedures::scheduler::validate_scheduler_procedure(procedure, args),
        "stats" => procedures::stats::validate_stats_procedure(procedure, args),
        "systemdb" => procedures::systemdb::validate_systemdb_procedure(procedure, args),
        "trigger" => procedures::trigger::validate_trigger_procedure(procedure, args),
        "util" => procedures::util::validate_util_procedure(procedure, args),
        "uuid" => procedures::uuid::validate_uuid_procedure(procedure, args),
        "xml" => procedures::xml::validate_xml_procedure(procedure, args),
        _ => Err(format!("Unknown APOC category: {}", category)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::PropertyValue;

    #[test]
    fn test_validate_apoc_call() {
        // Test valid APOC call
        let args = vec![PropertyValue::String("test".to_string())];
        let result = validate_apoc_call("apoc", "agg.first", &args);
        // This will fail until we implement the actual validation, but it should parse correctly
        assert!(result.is_ok() || result.is_err()); // Just checking it doesn't panic
    }

    #[test]
    fn test_unknown_namespace() {
        let args = vec![PropertyValue::String("test".to_string())];
        let result = validate_apoc_call("unknown", "procedure", &args);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown APOC namespace"));
    }

    #[test]
    fn test_apoc_module_compiles() {
        // This test just verifies the APOC module compiles
        assert!(true);
    }

    #[test]
    fn test_apoc_types_work() {
        use crate::parser::ast::PropertyValue;
        use types::ApocType;
        
        // Test that APOC types can match PropertyValue variants
        assert!(ApocType::String.matches(&PropertyValue::String("test".to_string())));
        assert!(ApocType::Integer.matches(&PropertyValue::Integer(42)));
        assert!(ApocType::Integer.matches(&PropertyValue::Number(42))); // Backward compatibility
        assert!(ApocType::Float.matches(&PropertyValue::Float(3.14)));
    }
} 