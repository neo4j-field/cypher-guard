// APOC load procedures
// Handles apoc.load.* procedures for loading data from various sources

use crate::parser::ast::PropertyValue;
use crate::parser::apoc::types::{ApocType, ProcedureSignature};

// APOC load procedures
// TODO: Add actual procedure signatures from APOC documentation
pub const LOAD_PROCEDURES: &[ProcedureSignature] = &[
    // TODO: Add load procedures like:
    // - apoc.load.json()
    // - apoc.load.csv()
    // - apoc.load.xml()
];

// TODO: Implement load procedure validation
// This will be implemented once we reference the APOC documentation
pub fn validate_load_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement load procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add unit tests for validate_load_procedure
    // TODO: Add tests for each load procedure signature
    // TODO: Add error case tests for invalid load procedures
} 