// APOC math procedures
// Handles apoc.math.* procedures for mathematical operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC math procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
static MATH_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.math.regr(values LIST<FLOAT>, xValues LIST<FLOAT>)
        ("apoc.math.regr", vec![
            ("values", ApocType::List),
            ("xValues", ApocType::List)
        ], vec![("value", ApocType::Map)]),
        
        // apoc.math.sigmoid(value FLOAT)
        ("apoc.math.sigmoid", vec![
            ("value", ApocType::Float)
        ], vec![("value", ApocType::Float)]),
        
        // apoc.math.cosh(value FLOAT)
        ("apoc.math.cosh", vec![
            ("value", ApocType::Float)
        ], vec![("value", ApocType::Float)]),
    ]
});

pub fn get_all_math_procedures() -> &'static [ProcedureSignature] {
    &MATH_PROCEDURES
}

// TODO: Implement math procedure validation
pub fn validate_math_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement math procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_regr_signature() {
        let procedures = get_all_math_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.math.regr")
            .expect("apoc.math.regr should be defined");
        assert_eq!(procedure.1.len(), 2);
        assert_eq!(procedure.1[0].0, "values");
        assert_eq!(procedure.1[1].0, "xValues");
    }

    #[test]
    fn test_math_sigmoid_signature() {
        let procedures = get_all_math_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.math.sigmoid")
            .expect("apoc.math.sigmoid should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "value");
    }

    #[test]
    fn test_math_cosh_signature() {
        let procedures = get_all_math_procedures();
        let procedure = procedures.iter()
            .find(|(name, _, _)| *name == "apoc.math.cosh")
            .expect("apoc.math.cosh should be defined");
        assert_eq!(procedure.1.len(), 1);
        assert_eq!(procedure.1[0].0, "value");
    }

    #[test]
    fn test_all_math_procedures_have_signatures() {
        let procedures = get_all_math_procedures();
        assert!(!procedures.is_empty(), "Should have at least one math procedure");
        for (name, _args, yields) in procedures {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
        }
    }
}


