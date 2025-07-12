// APOC collection procedures
// Handles apoc.coll.* procedures for collection operations

use crate::parser::ast::PropertyValue;
use crate::parser::apoc_core::types::{ApocType, ProcedureSignature};
use std::sync::LazyLock;

// APOC collection procedures
// Based on APOC documentation: https://neo4j.com/docs/apoc/2025.06/overview/
pub static COLL_PROCEDURES: LazyLock<Vec<ProcedureSignature>> = LazyLock::new(|| {
    vec![
        // apoc.coll.elements(list LIST<ANY>, index INTEGER)
        ("apoc.coll.elements", vec![
            ("list", ApocType::List),
            ("index", ApocType::Integer)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.partition(list LIST<ANY>, batchSize INTEGER)
        ("apoc.coll.partition", vec![
            ("list", ApocType::List),
            ("batchSize", ApocType::Integer)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.split(list LIST<ANY>, delimiter ANY)
        ("apoc.coll.split", vec![
            ("list", ApocType::List),
            ("delimiter", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.zipToRows(list1 LIST<ANY>, list2 LIST<ANY>)
        ("apoc.coll.zipToRows", vec![
            ("list1", ApocType::List),
            ("list2", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.avg(list LIST<NUMBER>)
        ("apoc.coll.avg", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.combinations(list LIST<ANY>, minSelect INTEGER, maxSelect INTEGER)
        ("apoc.coll.combinations", vec![
            ("list", ApocType::List),
            ("minSelect", ApocType::Integer),
            ("maxSelect", ApocType::Integer)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.contains(list LIST<ANY>, value ANY)
        ("apoc.coll.contains", vec![
            ("list", ApocType::List),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.containsAll(list LIST<ANY>, values LIST<ANY>)
        ("apoc.coll.containsAll", vec![
            ("list", ApocType::List),
            ("values", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.containsAllSorted(list LIST<ANY>, values LIST<ANY>)
        ("apoc.coll.containsAllSorted", vec![
            ("list", ApocType::List),
            ("values", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.containsDuplicates(list LIST<ANY>)
        ("apoc.coll.containsDuplicates", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.containsSorted(list LIST<ANY>, value ANY)
        ("apoc.coll.containsSorted", vec![
            ("list", ApocType::List),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.different(list1 LIST<ANY>, list2 LIST<ANY>)
        ("apoc.coll.different", vec![
            ("list1", ApocType::List),
            ("list2", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.disjunction(list1 LIST<ANY>, list2 LIST<ANY>)
        ("apoc.coll.disjunction", vec![
            ("list1", ApocType::List),
            ("list2", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.dropDuplicateNeighbors(list LIST<ANY>)
        ("apoc.coll.dropDuplicateNeighbors", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.duplicates(list LIST<ANY>)
        ("apoc.coll.duplicates", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.duplicatesWithCount(list LIST<ANY>)
        ("apoc.coll.duplicatesWithCount", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.fill(list LIST<ANY>, value ANY, size INTEGER)
        ("apoc.coll.fill", vec![
            ("list", ApocType::List),
            ("value", ApocType::Any),
            ("size", ApocType::Integer)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.flatten(list LIST<ANY>)
        ("apoc.coll.flatten", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.frequencies(list LIST<ANY>)
        ("apoc.coll.frequencies", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.frequenciesAsMap(list LIST<ANY>)
        ("apoc.coll.frequenciesAsMap", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.indexOf(list LIST<ANY>, value ANY)
        ("apoc.coll.indexOf", vec![
            ("list", ApocType::List),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.insert(list LIST<ANY>, index INTEGER, value ANY)
        ("apoc.coll.insert", vec![
            ("list", ApocType::List),
            ("index", ApocType::Integer),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.insertAll(list LIST<ANY>, index INTEGER, values LIST<ANY>)
        ("apoc.coll.insertAll", vec![
            ("list", ApocType::List),
            ("index", ApocType::Integer),
            ("values", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.intersection(list1 LIST<ANY>, list2 LIST<ANY>)
        ("apoc.coll.intersection", vec![
            ("list1", ApocType::List),
            ("list2", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.isEqualCollection(list1 LIST<ANY>, list2 LIST<ANY>)
        ("apoc.coll.isEqualCollection", vec![
            ("list1", ApocType::List),
            ("list2", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.max(list LIST<NUMBER>)
        ("apoc.coll.max", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.min(list LIST<NUMBER>)
        ("apoc.coll.min", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.occurrences(list LIST<ANY>, value ANY)
        ("apoc.coll.occurrences", vec![
            ("list", ApocType::List),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.pairs(list LIST<ANY>)
        ("apoc.coll.pairs", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.pairsMin(list LIST<ANY>)
        ("apoc.coll.pairsMin", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.partition(list LIST<ANY>, batchSize INTEGER)
        ("apoc.coll.partition", vec![
            ("list", ApocType::List),
            ("batchSize", ApocType::Integer)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.randomItem(list LIST<ANY>)
        ("apoc.coll.randomItem", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.randomItems(list LIST<ANY>, count INTEGER)
        ("apoc.coll.randomItems", vec![
            ("list", ApocType::List),
            ("count", ApocType::Integer)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.remove(list LIST<ANY>, value ANY)
        ("apoc.coll.remove", vec![
            ("list", ApocType::List),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.removeAll(list LIST<ANY>, values LIST<ANY>)
        ("apoc.coll.removeAll", vec![
            ("list", ApocType::List),
            ("values", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.removeAtIndex(list LIST<ANY>, index INTEGER)
        ("apoc.coll.removeAtIndex", vec![
            ("list", ApocType::List),
            ("index", ApocType::Integer)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.removeAtIndexes(list LIST<ANY>, indexes LIST<INTEGER>)
        ("apoc.coll.removeAtIndexes", vec![
            ("list", ApocType::List),
            ("indexes", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.reverse(list LIST<ANY>)
        ("apoc.coll.reverse", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.set(list LIST<ANY>, index INTEGER, value ANY)
        ("apoc.coll.set", vec![
            ("list", ApocType::List),
            ("index", ApocType::Integer),
            ("value", ApocType::Any)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.shuffle(list LIST<ANY>)
        ("apoc.coll.shuffle", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.sort(list LIST<ANY>)
        ("apoc.coll.sort", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.sortMulti(list LIST<ANY>, keys LIST<STRING>)
        ("apoc.coll.sortMulti", vec![
            ("list", ApocType::List),
            ("keys", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.sortNodes(list LIST<NODE>, property STRING)
        ("apoc.coll.sortNodes", vec![
            ("list", ApocType::List),
            ("property", ApocType::String)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.sortText(list LIST<STRING>)
        ("apoc.coll.sortText", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.pairWithOffset(list LIST<ANY>, offset INTEGER)
        ("apoc.coll.pairWithOffset", vec![
            ("list", ApocType::List),
            ("offset", ApocType::Integer)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.stdev(list LIST<NUMBER>)
        ("apoc.coll.stdev", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.subtract(list1 LIST<ANY>, list2 LIST<ANY>)
        ("apoc.coll.subtract", vec![
            ("list1", ApocType::List),
            ("list2", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.sum(list LIST<NUMBER>)
        ("apoc.coll.sum", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.sumLongs(list LIST<INTEGER>)
        ("apoc.coll.sumLongs", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.toSet(list LIST<ANY>)
        ("apoc.coll.toSet", vec![
            ("list", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.union(list1 LIST<ANY>, list2 LIST<ANY>)
        ("apoc.coll.union", vec![
            ("list1", ApocType::List),
            ("list2", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.unionAll(list1 LIST<ANY>, list2 LIST<ANY>)
        ("apoc.coll.unionAll", vec![
            ("list1", ApocType::List),
            ("list2", ApocType::List)
        ], vec![("result", ApocType::Any)]),
        
        // apoc.coll.zip(list1 LIST<ANY>, list2 LIST<ANY>)
        ("apoc.coll.zip", vec![
            ("list1", ApocType::List),
            ("list2", ApocType::List)
        ], vec![("result", ApocType::Any)]),
    ]
});

// TODO: Implement collection procedure validation
// This will be implemented once we reference the APOC documentation
pub fn validate_coll_procedure(_name: &str, _args: &[PropertyValue]) -> Result<(), String> {
    todo!("Implement collection procedure validation - waiting for APOC documentation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coll_avg_signature() {
        let procedure = COLL_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.coll.avg")
            .expect("apoc.coll.avg should be defined");
        
        assert_eq!(procedure.1.len(), 1); // 1 parameter
        assert_eq!(procedure.1[0].0, "list");
        assert_eq!(procedure.1[0].1, ApocType::List);
    }

    #[test]
    fn test_coll_contains_signature() {
        let procedure = COLL_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.coll.contains")
            .expect("apoc.coll.contains should be defined");
        
        assert_eq!(procedure.1.len(), 2); // 2 parameters
        assert_eq!(procedure.1[0].0, "list");
        assert_eq!(procedure.1[0].1, ApocType::List);
        assert_eq!(procedure.1[1].0, "value");
        assert_eq!(procedure.1[1].1, ApocType::Any);
    }

    #[test]
    fn test_coll_insert_signature() {
        let procedure = COLL_PROCEDURES.iter()
            .find(|(name, _, _)| *name == "apoc.coll.insert")
            .expect("apoc.coll.insert should be defined");
        
        assert_eq!(procedure.1.len(), 3); // 3 parameters
        assert_eq!(procedure.1[0].0, "list");
        assert_eq!(procedure.1[0].1, ApocType::List);
        assert_eq!(procedure.1[1].0, "index");
        assert_eq!(procedure.1[1].1, ApocType::Integer);
        assert_eq!(procedure.1[2].0, "value");
        assert_eq!(procedure.1[2].1, ApocType::Any);
    }

    #[test]
    fn test_all_coll_procedures_have_signatures() {
        assert!(!COLL_PROCEDURES.is_empty(), "Should have at least one collection procedure");
        
        for (name, args, yields) in COLL_PROCEDURES.iter() {
            assert!(!name.is_empty(), "Procedure name should not be empty");
            assert!(!yields.is_empty(), "Procedure should have at least one yield field");
            assert_eq!(yields[0].0, "result", "First yield field should be 'result'");
            
            // All collection procedures should have at least one parameter (the list)
            assert!(!args.is_empty(), "Collection procedures should have at least one parameter");
            assert_eq!(args[0].0, "list", "First parameter should be 'list'");
            assert_eq!(args[0].1, ApocType::List, "First parameter should be List type");
        }
    }
}


