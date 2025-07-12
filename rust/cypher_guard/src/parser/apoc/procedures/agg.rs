// APOC aggregation functions
// Handles apoc.agg.* functions for aggregation operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc::types::{ApocType, FunctionSignature};

// APOC aggregation functions
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
pub const AGG_FUNCTIONS: &[FunctionSignature] = &[
    // apoc.agg.first(value ANY)
    ("apoc.agg.first", vec![("value", ApocType::Any)], ApocType::Any),
    
    // apoc.agg.graph(value ANY)
    ("apoc.agg.graph", vec![("value", ApocType::Any)], ApocType::Any),
    
    // apoc.agg.last(value ANY)
    ("apoc.agg.last", vec![("value", ApocType::Any)], ApocType::Any),
    
    // apoc.agg.maxItems(value ANY, limit INTEGER)
    ("apoc.agg.maxItems", vec![
        ("value", ApocType::Any),
        ("limit", ApocType::Integer)
    ], ApocType::Any),
    
    // apoc.agg.median(value ANY)
    ("apoc.agg.median", vec![("value", ApocType::Any)], ApocType::Any),
    
    // apoc.agg.minItems(value ANY, limit INTEGER)
    ("apoc.agg.minItems", vec![
        ("value", ApocType::Any),
        ("limit", ApocType::Integer)
    ], ApocType::Any),
    
    // apoc.agg.nth(value ANY, index INTEGER)
    ("apoc.agg.nth", vec![
        ("value", ApocType::Any),
        ("index", ApocType::Integer)
    ], ApocType::Any),
    
    // apoc.agg.percentiles(value ANY, percentiles LIST<FLOAT>)
    ("apoc.agg.percentiles", vec![
        ("value", ApocType::Any),
        ("percentiles", ApocType::List)
    ], ApocType::Any),
    
    // apoc.agg.product(value ANY)
    ("apoc.agg.product", vec![("value", ApocType::Any)], ApocType::Any),
    
    // apoc.agg.slice(value ANY, start INTEGER, end INTEGER)
    ("apoc.agg.slice", vec![
        ("value", ApocType::Any),
        ("start", ApocType::Integer),
        ("end", ApocType::Integer)
    ], ApocType::Any),
    
    // apoc.agg.statistics(value ANY)
    ("apoc.agg.statistics", vec![("value", ApocType::Any)], ApocType::Any),
];

// TODO: Implement aggregation function validation
// This will be implemented once we reference the APOC documentation
pub fn validate_agg_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement aggregation function validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agg_first_signature() {
        let function = AGG_FUNCTIONS.iter()
            .find(|(name, _, _)| *name == "apoc.agg.first")
            .expect("apoc.agg.first should be defined");
        
        assert_eq!(function.1.len(), 1); // 1 parameter
        assert_eq!(function.1[0].0, "value");
        assert_eq!(function.1[0].1, ApocType::Any);
        assert_eq!(function.2, ApocType::Any); // return type
    }

    #[test]
    fn test_agg_max_items_signature() {
        let function = AGG_FUNCTIONS.iter()
            .find(|(name, _, _)| *name == "apoc.agg.maxItems")
            .expect("apoc.agg.maxItems should be defined");
        
        assert_eq!(function.1.len(), 2); // 2 parameters
        assert_eq!(function.1[0].0, "value");
        assert_eq!(function.1[0].1, ApocType::Any);
        assert_eq!(function.1[1].0, "limit");
        assert_eq!(function.1[1].1, ApocType::Integer);
        assert_eq!(function.2, ApocType::Any); // return type
    }

    #[test]
    fn test_agg_slice_signature() {
        let function = AGG_FUNCTIONS.iter()
            .find(|(name, _, _)| *name == "apoc.agg.slice")
            .expect("apoc.agg.slice should be defined");
        
        assert_eq!(function.1.len(), 3); // 3 parameters
        assert_eq!(function.1[0].0, "value");
        assert_eq!(function.1[0].1, ApocType::Any);
        assert_eq!(function.1[1].0, "start");
        assert_eq!(function.1[1].1, ApocType::Integer);
        assert_eq!(function.1[2].0, "end");
        assert_eq!(function.1[2].1, ApocType::Integer);
        assert_eq!(function.2, ApocType::Any); // return type
    }

    #[test]
    fn test_all_agg_functions_have_signatures() {
        assert!(!AGG_FUNCTIONS.is_empty(), "Should have at least one aggregation function");
        
        for (name, args, return_type) in AGG_FUNCTIONS {
            assert!(!name.is_empty(), "Function name should not be empty");
            assert!(!args.is_empty(), "Function should have at least one parameter");
            assert_eq!(*return_type, ApocType::Any, "All agg functions return Any type");
            
            // All agg functions should have 'value' as first parameter
            assert_eq!(args[0].0, "value", "First parameter should be 'value'");
            assert_eq!(args[0].1, ApocType::Any, "First parameter should be Any type");
        }
    }
} 