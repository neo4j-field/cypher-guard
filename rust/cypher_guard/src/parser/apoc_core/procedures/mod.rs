// APOC procedures module
// Contains procedure definitions organized by category

pub mod agg;        // apoc.agg.* procedures
pub mod algo;       // apoc.algo.* procedures
pub mod atomic;     // apoc.atomic.* procedures
pub mod bitwise;    // apoc.bitwise.* procedures
pub mod coll;       // apoc.coll.* procedures
pub mod convert;    // apoc.convert.* procedures
pub mod create;     // apoc.create.* procedures
pub mod cypher;     // apoc.cypher.* procedures
pub mod data;       // apoc.data.* procedures
pub mod date;       // apoc.date.* procedures
pub mod diff;       // apoc.diff.* procedures
pub mod export;     // apoc.export.* procedures
pub mod generate;   // apoc.generate.* procedures
pub mod graph;      // apoc.graph.* procedures
pub mod hash;       // apoc.hash.* procedures
pub mod import;     // apoc.import.* procedures
pub mod json;       // apoc.json.* procedures
pub mod lock;       // apoc.lock.* procedures
pub mod log;        // apoc.log.* procedures
pub mod map;        // apoc.map.* procedures
pub mod math;       // apoc.math.* procedures
pub mod merge;      // apoc.merge.* procedures
pub mod meta;       // apoc.meta.* procedures
pub mod monitoring; // apoc.monitoring.* procedures
pub mod periodic;   // apoc.periodic.* procedures
pub mod refactor;   // apoc.refactor.* procedures
pub mod run;        // apoc.run.* procedures
pub mod schema;     // apoc.schema.* procedures
pub mod scheduler;  // apoc.scheduler.* procedures
pub mod stats;      // apoc.stats.* procedures
pub mod systemdb;   // apoc.systemdb.* procedures
pub mod trigger;    // apoc.trigger.* procedures
pub mod util;       // apoc.util.* procedures
pub mod uuid;       // apoc.uuid.* procedures
pub mod xml;        // apoc.xml.* procedures

// Re-export common types and functions
pub use crate::parser::ast::PropertyValue;
pub use crate::parser::apoc_core::types::{ApocType, ProcedureSignature, FunctionSignature};

// TODO: Implement main APOC procedure validation function
pub fn validate_apoc_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement main APOC procedure validation")
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add integration tests for APOC procedure validation
    // TODO: Add tests that span multiple APOC categories
    // TODO: Add error case tests for invalid APOC procedures
} 