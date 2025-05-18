use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map, opt, recognize},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::parser::ast::*;
use crate::parser::patterns::*;
use crate::parser::utils::identifier;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Clause {
    Match(MatchClause),
    OptionalMatch(MatchClause),
    Merge(MergeClause),
    Create(CreateClause),
    Return(ReturnClause),
    Query(Query),
}

// Parses a list of match elements separated by commas
pub fn match_element_list(input: &str) -> IResult<&str, Vec<MatchElement>> {
    println!("Parsing match element list: {}", input);
    let (input, first) = match_element(input)?;
    println!("First match element: {:?}", first);
    let (input, rest) = opt(preceded(
        tuple((multispace0, char(','), multispace0)),
        match_element,
    ))(input)?;
    println!("Rest of match elements: {:?}", rest);
    let mut elements = vec![first];
    if let Some(rest) = rest {
        elements.push(rest);
    }
    println!("Final match elements: {:?}", elements);
    Ok((input, elements))
}

// Parses the MATCH clause (e.g. MATCH (a)-[:KNOWS]->(b))
pub fn match_clause(input: &str) -> IResult<&str, MatchClause> {
    println!("Parsing match clause: {}", input);
    let (input, _) = multispace0(input)?;
    println!("After initial whitespace: {}", input);
    let (input, is_optional) = opt(tuple((tag("OPTIONAL"), multispace1)))(input)?;
    let (input, _) = tag("MATCH")(input)?;
    println!("After MATCH tag: {}", input);
    let (input, _) = multispace1(input)?;
    println!("After MATCH whitespace: {}", input);
    let (input, elements) = match_element_list(input)?;
    println!("Match clause elements: {:?}", elements);
    Ok((input, MatchClause { 
        elements,
        is_optional: is_optional.is_some(),
    }))
}

// Parses a return item: either an identifier or a dotted property access (e.g., a, a.name)
fn return_item(input: &str) -> IResult<&str, String> {
    let (input, first) = identifier(input)?;
    let (input, rest) = opt(preceded(char('.'), identifier))(input)?;
    if let Some(prop) = rest {
        Ok((input, format!("{}.{}", first, prop)))
    } else {
        Ok((input, first.to_string()))
    }
}

// Parses the RETURN clause (e.g. RETURN a, b, a.name)
pub fn return_clause(input: &str) -> IResult<&str, ReturnClause> {
    println!("Parsing return clause: {}", input); // Debug
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("RETURN")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) =
        separated_list1(tuple((multispace0, char(','), multispace0)), return_item)(input)?;
    println!("Return clause items: {:?}", items); // Debug
    Ok((
        input,
        ReturnClause {
            items,
        },
    ))
}

// Parses a numeric literal
fn numeric_literal(input: &str) -> IResult<&str, String> {
    let (input, num) = recognize(digit1)(input)?;
    Ok((input, num.to_string()))
}

// Parses a function call
fn function_call(input: &str) -> IResult<&str, (String, Vec<String>)> {
    let (input, function) = map(identifier, |s| s.to_string())(input)?;
    let (input, _) = char('(')(input)?;
    let (input, args) = separated_list1(
        tuple((multispace0, char(','), multispace0)),
        map(identifier, |s| s.to_string())
    )(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, (function, args)))
}

// Parses a path property access
fn path_property(input: &str) -> IResult<&str, (String, String)> {
    let (input, path_var) = map(identifier, |s| s.to_string())(input)?;
    let (input, _) = char('.')(input)?;
    let (input, property) = map(identifier, |s| s.to_string())(input)?;
    Ok((input, (path_var, property)))
}

// Parses a WHERE condition (e.g., a.age > 30, point.distance(a.location, b.location) > 10)
fn where_condition(input: &str) -> IResult<&str, WhereCondition> {
    alt((
        // Function call condition
        map(
            tuple((
                function_call,
                multispace0,
                alt((tag(">"), tag("<"), tag(">="), tag("<="), tag("="), tag("<>"))),
                multispace0,
                alt((numeric_literal, map(identifier, |s| s.to_string()))),
            )),
            |((function, args), _, operator, _, right)| WhereCondition::FunctionCall {
                function,
                arguments: args,
            },
        ),
        // Path property condition
        map(
            tuple((
                path_property,
                multispace0,
                alt((tag(">"), tag("<"), tag(">="), tag("<="), tag("="), tag("<>"))),
                multispace0,
                alt((numeric_literal, map(identifier, |s| s.to_string()))),
            )),
            |((path_var, property), _, operator, _, right)| WhereCondition::PathProperty {
                path_var,
                property,
            },
        ),
        // Regular property comparison
        map(
            tuple((
                map(identifier, |s| s.to_string()),
                multispace0,
                alt((tag(">"), tag("<"), tag(">="), tag("<="), tag("="), tag("<>"))),
                multispace0,
                alt((numeric_literal, map(identifier, |s| s.to_string()))),
            )),
            |(left, _, operator, _, right)| WhereCondition::Comparison {
                left,
                operator: operator.to_string(),
                right,
            },
        ),
    ))(input)
}

// Parses the WHERE clause (e.g. WHERE a.age > 30)
pub fn where_clause(input: &str) -> IResult<&str, WhereClause> {
    println!("Parsing where clause: {}", input);
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("WHERE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, conditions) = separated_list1(
        tuple((multispace0, tag("AND"), multispace0)),
        where_condition
    )(input)?;
    println!("Where conditions: {:?}", conditions);
    Ok((input, WhereClause { conditions }))
}

// Parses a SET clause (e.g. SET a.name = 'Alice')
fn set_clause(input: &str) -> IResult<&str, SetClause> {
    let (input, _) = multispace0(input)?;
    let (input, variable) = identifier(input)?;
    let (input, _) = char('.')(input)?;
    let (input, property) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = alt((
        map(string_literal, PropertyValue::String),
        map(numeric_literal, |s| PropertyValue::Number(s.parse().unwrap())),
    ))(input)?;
    println!("Parsed set clause: variable={}, property={}, value={:?}", variable, property, value);
    Ok((
        input,
        SetClause {
            variable: variable.to_string(),
            property: property.to_string(),
            value,
        },
    ))
}

// Parses ON CREATE clause (e.g. ON CREATE SET a.name = 'Alice')
fn on_create_clause(input: &str) -> IResult<&str, OnCreateClause> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("ON CREATE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("SET")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, set_clauses) = separated_list1(
        tuple((multispace0, char(','), multispace0)),
        set_clause
    )(input)?;
    println!("Parsed set clauses: {:?}", set_clauses);
    Ok((input, OnCreateClause { set_clauses }))
}

// Parses ON MATCH clause (e.g. ON MATCH SET a.name = 'Alice')
fn on_match_clause(input: &str) -> IResult<&str, OnMatchClause> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("ON MATCH")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("SET")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, set_clauses) = separated_list1(
        tuple((multispace0, char(','), multispace0)),
        set_clause
    )(input)?;
    Ok((input, OnMatchClause { set_clauses }))
}

// Parses the MERGE clause (e.g. MERGE (a:Person {name: 'Alice'}) ON CREATE SET a.created = true)
pub fn merge_clause(input: &str) -> IResult<&str, MergeClause> {
    println!("Parsing merge clause: {}", input);
    let (input, _) = tag("MERGE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, elements) = match_element_list(input)?;
    let (input, _) = multispace0(input)?;
    let (input, on_create) = opt(on_create_clause)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, on_match) = opt(on_match_clause)(input)?;
    Ok((
        input,
        MergeClause {
            elements,
            on_create,
            on_match,
        },
    ))
}

// Parses the CREATE clause (e.g. CREATE (a:Person {name: 'Alice'}))
pub fn create_clause(input: &str) -> IResult<&str, CreateClause> {
    let (input, _) = tag("CREATE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, elements) = match_element_list(input)?;
    Ok((input, CreateClause { elements }))
}

// Parses a single match element (pattern or quantified)
fn match_element(input: &str) -> IResult<&str, MatchElement> {
    println!("Parsing match element: {}", input);
    alt((
        map(pattern_element_sequence, MatchElement::Pattern),
        map(quantified_path_pattern, MatchElement::QuantifiedPathPattern),
    ))(input)
}

// Parses a quantified path pattern with optional WHERE clause and path variable
fn quantified_path_pattern(input: &str) -> IResult<&str, QuantifiedPathPattern> {
    println!("Parsing quantified path pattern: {}", input);
    
    // Parse optional path variable
    let (input, path_var) = opt(preceded(
        tuple((multispace0, char('='), multispace0)),
        map(identifier, |s| s.to_string())
    ))(input)?;
    
    // Parse the pattern
    let (input, pattern) = pattern(input)?;
    
    // Parse length range
    let (input, length) = opt(length_range)(input)?;
    
    // Parse optional WHERE clause
    let (input, where_clause) = opt(where_clause)(input)?;
    
    Ok((
        input,
        QuantifiedPathPattern {
            pattern,
            min: length.as_ref().and_then(|l| l.min),
            max: length.as_ref().and_then(|l| l.max),
            where_clause,
            path_variable: path_var,
        },
    ))
}

// Parses a relationship direction
pub fn relationship_direction(input: &str) -> IResult<&str, Direction> {
    println!("Parsing relationship direction: {}", input);
    let result = alt((
        map(tag("<-"), |_| Direction::Left),
        map(tag("->"), |_| Direction::Right),
        map(tag("-"), |_| Direction::Undirected),
    ))(input);
    println!("Relationship direction result: {:?}", result);
    result
}

// Parses a relationship type
pub fn relationship_type(input: &str) -> IResult<&str, String> {
    let (input, _) = char(':')(input)?;
    let (input, rel_type) = map(identifier, |s| s.to_string())(input)?;
    Ok((input, rel_type))
}

// Parses a property map
pub fn property_map(input: &str) -> IResult<&str, Vec<Property>> {
    let (input, _) = char('{')(input)?;
    let (input, props) = separated_list1(
        tuple((multispace0, char(','), multispace0)),
        property
    )(input)?;
    let (input, _) = char('}')(input)?;
    Ok((input, props))
}

// Parses a single property
fn property(input: &str) -> IResult<&str, Property> {
    let (input, key) = map(identifier, |s| s.to_string())(input)?;
    let (input, _) = tuple((multispace0, char(':'), multispace0))(input)?;
    let (input, value) = property_value(input)?;
    Ok((input, Property { key, value }))
}

// Parses a property value
fn property_value(input: &str) -> IResult<&str, PropertyValue> {
    alt((
        map(string_literal, PropertyValue::String),
        map(numeric_literal, |s| PropertyValue::Number(s.parse().unwrap())),
    ))(input)
}

// Parses a string literal
fn string_literal(input: &str) -> IResult<&str, String> {
    let (input, _) = char('\'')(input)?;
    let (input, content) = take_while1(|c| c != '\'')(input)?;
    let (input, _) = char('\'')(input)?;
    Ok((input, content.to_string()))
}

// Parses a relationship with optional WHERE clause
fn relationship_details(input: &str) -> IResult<&str, RelationshipDetails> {
    println!("Parsing relationship details: {}", input);
    // Parse relationship details (no direction)
    let (input, _) = char('[')(input)?;
    let (input, variable) = opt(identifier)(input)?;
    let (input, rel_type) = opt(relationship_type)(input)?;
    let (input, properties) = opt(property_map)(input)?;
    let (input, _) = char(']')(input)?;
    // Parse length range if present
    let (input, length) = opt(length_range)(input)?;
    // Parse optional WHERE clause
    let (input, where_clause) = opt(where_clause)(input)?;
    Ok((
        input,
        RelationshipDetails {
            variable: variable.map(|s| s.to_string()),
            direction: Direction::Undirected, // Will be set by pattern parser
            properties,
            rel_type,
            length,
            where_clause,
        },
    ))
}

// Update the parser to handle OPTIONAL MATCH
#[allow(dead_code)]
pub fn clause(input: &str) -> IResult<&str, Clause> {
    alt((
        map(
            tuple((tag("OPTIONAL MATCH"), multispace1, match_element_list)),
            |(_, _, elements)| Clause::OptionalMatch(MatchClause { 
                elements,
                is_optional: true,
            }),
        ),
        map(
            tuple((tag("MATCH"), multispace1, match_element_list)),
            |(_, _, elements)| Clause::Match(MatchClause { 
                elements,
                is_optional: false,
            }),
        ),
        map(merge_clause, Clause::Merge),
        map(create_clause, Clause::Create),
        // ... existing clause parsers ...
    ))(input)
}

// Update the query parser to handle MERGE and CREATE
pub fn parse_query(input: &str) -> IResult<&str, Query> {
    println!("Parsing query: {}", input);
    let (input, match_clause) = opt(match_clause)(input)?;
    let (input, merge_clause) = opt(merge_clause)(input)?;
    let (input, create_clause) = opt(create_clause)(input)?;
    let (input, where_clause) = opt(where_clause)(input)?;
    let (input, return_clause) = opt(return_clause)(input)?;
    Ok((
        input,
        Query {
            match_clause,
            merge_clause,
            create_clause,
            where_clause,
            return_clause,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional_match_clause() {
        let input = "OPTIONAL MATCH (a)-[:KNOWS]->(b)";
        let result = match_clause(input);
        assert!(result.is_ok());
        let (_, match_clause) = result.unwrap();
        assert!(match_clause.is_optional);
        assert_eq!(match_clause.elements.len(), 1);
    }

    #[test]
    fn test_regular_match_clause() {
        let input = "MATCH (a)-[:KNOWS]->(b)";
        let result = match_clause(input);
        assert!(result.is_ok());
        let (_, match_clause) = result.unwrap();
        assert!(!match_clause.is_optional);
        assert_eq!(match_clause.elements.len(), 1);
    }

    #[test]
    fn test_merge_clause() {
        let input = "MERGE (a:Person {name: 'Alice'}) ON CREATE SET a.created = true";
        let result = merge_clause(input);
        assert!(result.is_ok());
        let (_, merge) = result.unwrap();
        assert_eq!(merge.elements.len(), 1);
        assert!(merge.on_create.is_some());
        assert!(merge.on_match.is_none());
        let on_create = merge.on_create.unwrap();
        assert_eq!(on_create.set_clauses.len(), 1);
        let set = &on_create.set_clauses[0];
        assert_eq!(set.variable, "a");
        assert_eq!(set.property, "created");
        assert_eq!(set.value, PropertyValue::String("true".to_string()));
    }

    #[test]
    fn test_create_clause() {
        let input = "CREATE (a:Person {name: 'Alice'})";
        let result = create_clause(input);
        assert!(result.is_ok());
        let (_, create) = result.unwrap();
        assert_eq!(create.elements.len(), 1);
        match &create.elements[0] {
            MatchElement::Pattern(pattern) => {
                assert_eq!(pattern.len(), 1);
                match &pattern[0] {
                    PatternElement::Node(node) => {
                        assert_eq!(node.variable, Some("a".to_string()));
                        assert_eq!(node.label, Some("Person".to_string()));
                        assert!(node.properties.is_some());
                        let props = node.properties.as_ref().unwrap();
                        assert_eq!(props.len(), 1);
                        assert_eq!(props[0].key, "name");
                        assert_eq!(props[0].value, PropertyValue::String("Alice".to_string()));
                    }
                    _ => panic!("Expected Node"),
                }
            }
            _ => panic!("Expected Pattern"),
        }
    }

    #[test]
    fn test_merge_with_on_match() {
        let input = "MERGE (a:Person {name: 'Alice'}) ON MATCH SET a.lastSeen = '2024-03-20'";
        let result = merge_clause(input);
        assert!(result.is_ok());
        let (_, merge) = result.unwrap();
        assert_eq!(merge.elements.len(), 1);
        assert!(merge.on_create.is_none());
        assert!(merge.on_match.is_some());
        let on_match = merge.on_match.unwrap();
        assert_eq!(on_match.set_clauses.len(), 1);
        let set = &on_match.set_clauses[0];
        assert_eq!(set.variable, "a");
        assert_eq!(set.property, "lastSeen");
        assert_eq!(set.value, PropertyValue::String("2024-03-20".to_string()));
    }

    #[test]
    fn test_create_with_relationship() {
        let input = "CREATE (a:Person {name: 'Alice'})-[:KNOWS]->(b:Person {name: 'Bob'})";
        let result = create_clause(input);
        assert!(result.is_ok());
        let (_, create) = result.unwrap();
        assert_eq!(create.elements.len(), 1);
        match &create.elements[0] {
            MatchElement::Pattern(pattern) => {
                assert_eq!(pattern.len(), 3);
                // Check first node
                match &pattern[0] {
                    PatternElement::Node(node) => {
                        assert_eq!(node.variable, Some("a".to_string()));
                        assert_eq!(node.label, Some("Person".to_string()));
                    }
                    _ => panic!("Expected Node"),
                }
                // Check relationship
                match &pattern[1] {
                    PatternElement::Relationship(RelationshipPattern::Regular(details)) => {
                        assert_eq!(details.rel_type, Some("KNOWS".to_string()));
                        assert_eq!(details.direction, Direction::Right);
                    }
                    _ => panic!("Expected Regular relationship"),
                }
                // Check second node
                match &pattern[2] {
                    PatternElement::Node(node) => {
                        assert_eq!(node.variable, Some("b".to_string()));
                        assert_eq!(node.label, Some("Person".to_string()));
                    }
                    _ => panic!("Expected Node"),
                }
            }
            _ => panic!("Expected Pattern"),
        }
    }
}
