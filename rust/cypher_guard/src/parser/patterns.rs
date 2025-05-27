use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map, opt},
    multi::many1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::parser::ast::*;
use crate::parser::clauses::{property_map, relationship_type, where_clause};
use crate::parser::utils::{identifier, number_literal, string_literal};

#[cfg(test)]
use crate::parser::clauses::match_clause;

#[allow(dead_code)]
// TODO: Clean up unused functions or refactor to use them
pub fn property_value(input: &str) -> IResult<&str, PropertyValue> {
    alt((
        map(string_literal, PropertyValue::String),
        map(number_literal, PropertyValue::Number),
    ))(input)
}

#[allow(dead_code)]
// TODO: Clean up unused functions or refactor to use them
pub fn property(input: &str) -> IResult<&str, Property> {
    let (input, _) = multispace0(input)?;
    let (input, key) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = property_value(input)?;
    Ok((
        input,
        Property {
            key: key.to_string(),
            value,
        },
    ))
}

pub fn node_pattern(input: &str) -> IResult<&str, NodePattern> {
    println!("Parsing node pattern: {}", input);
    let (input, _) = char('(')(input)?;
    println!("After parsing '(': {}", input);
    let (input, variable) = opt(identifier)(input)?;
    println!(
        "After parsing variable: {:?}, remaining input: {}",
        variable, input
    );
    let (input, label) = opt(preceded(char(':'), identifier))(input)?;
    println!(
        "After parsing label: {:?}, remaining input: {}",
        label, input
    );
    let (input, _) = multispace0(input)?;
    let (input, properties) = opt(property_map)(input)?;
    println!(
        "After parsing properties: {:?}, remaining input: {}",
        properties, input
    );
    let (input, _) = char(')')(input)?;
    println!("After parsing ')': {}", input);
    let result = NodePattern {
        variable: variable.map(|s| s.to_string()),
        label: label.map(|s| s.to_string()),
        properties,
    };
    println!("Node pattern result: {:?}", result);
    Ok((input, result))
}

pub fn relationship_details(input: &str) -> IResult<&str, RelationshipDetails> {
    println!("Parsing relationship details: {}", input);
    // Parse relationship details (no direction)
    let (input, _) = char('[')(input)?;
    println!("After parsing '[': {}", input);
    let (input, variable) = opt(identifier)(input)?;
    println!(
        "After parsing variable: {:?}, remaining input: {}",
        variable, input
    );
    let (input, rel_type) = opt(relationship_type)(input)?;
    println!(
        "After parsing rel_type: {:?}, remaining input: {}",
        rel_type, input
    );
    let (input, _) = multispace0(input)?;
    let (input, properties) = opt(property_map)(input)?;
    println!(
        "After parsing properties: {:?}, remaining input: {}",
        properties, input
    );
    let (input, _) = char(']')(input)?;
    println!("After parsing ']': {}", input);
    // Parse length range if present
    let (input, length) = opt(length_range)(input)?;
    println!(
        "After parsing length: {:?}, remaining input: {}",
        length, input
    );
    // Parse optional WHERE clause
    let (input, where_clause) = opt(where_clause)(input)?;
    println!(
        "After parsing where_clause: {:?}, remaining input: {}",
        where_clause, input
    );
    let result = RelationshipDetails {
        variable: variable.map(|s| s.to_string()),
        direction: Direction::Undirected, // Will be set by pattern parser
        properties,
        rel_type,
        length,
        where_clause,
    };
    println!("Relationship details result: {:?}", result);
    Ok((input, result))
}

pub fn length_range(input: &str) -> IResult<&str, LengthRange> {
    let (input, _) = char('{')(input)?;
    let (input, min) = opt(digit1)(input)?;
    let (input, _) = char(',')(input)?;
    let (input, max) = opt(digit1)(input)?;
    let (input, _) = char('}')(input)?;
    Ok((
        input,
        LengthRange {
            min: min.map(|s| s.parse().unwrap()),
            max: max.map(|s| s.parse().unwrap()),
        },
    ))
}

#[allow(dead_code)]
// TODO: Clean up unused functions or refactor to use them
pub fn relationship_pattern(input: &str) -> IResult<&str, RelationshipPattern> {
    println!("Parsing relationship pattern: {}", input);
    let (input, _) = char('[')(input)?;
    let (input, variable) = opt(identifier)(input)?;
    let (input, rel_type) = opt(relationship_type)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, properties) = opt(property_map)(input)?;
    let (input, _) = char(']')(input)?;
    let (input, length) = opt(length_range)(input)?;
    let (input, where_clause) = opt(where_clause)(input)?;
    Ok((
        input,
        RelationshipPattern::Regular(RelationshipDetails {
            variable: variable.map(|s| s.to_string()),
            direction: Direction::Undirected,
            properties,
            rel_type,
            length,
            where_clause,
        }),
    ))
}

pub fn pattern_element_sequence(input: &str) -> IResult<&str, Vec<PatternElement>> {
    println!("Parsing pattern element sequence: {}", input);
    let (input, is_optional) = opt(tuple((tag("OPTIONAL"), multispace1)))(input)?;
    let (mut input, first_node) = node_pattern(input)?;
    let mut elements = vec![PatternElement::Node(first_node)];

    loop {
        let (rest, _) = multispace0(input)?;
        // Check for a dash or arrow (start of a relationship)
        let rel_start = {
            if let Ok((after, _)) = tag::<&str, &str, nom::error::Error<&str>>("<-")(rest) {
                Ok((after, "<-"))
            } else if let Ok((after, _)) = tag::<&str, &str, nom::error::Error<&str>>("-")(rest) {
                Ok((after, "-"))
            } else {
                Err(nom::Err::Error(nom::error::Error::new(
                    rest,
                    nom::error::ErrorKind::Tag,
                )))
            }
        };
        if let Ok((after_left, left)) = rel_start {
            // Parse relationship details
            let (after_details, details) = relationship_details(after_left)?;
            // Parse right dash/arrow
            let right_parse = {
                if let Ok((after, _)) =
                    tag::<&str, &str, nom::error::Error<&str>>("->")(after_details)
                {
                    Ok((after, "->"))
                } else if let Ok((after, _)) =
                    tag::<&str, &str, nom::error::Error<&str>>("-")(after_details)
                {
                    Ok((after, "-"))
                } else {
                    Err(nom::Err::Error(nom::error::Error::new(
                        after_details,
                        nom::error::ErrorKind::Tag,
                    )))
                }
            };
            let (after_right, right) = right_parse?;
            // Determine direction
            let direction = match (left, right) {
                ("-", "->") => Direction::Right,
                ("<-", "-") => Direction::Left,
                ("-", "-") => Direction::Undirected,
                ("<-", "->") => Direction::Undirected, // Technically invalid, treat as undirected
                _ => Direction::Undirected,
            };
            let mut details = details;
            details.direction = direction;
            let rel = if is_optional.is_some() {
                RelationshipPattern::OptionalRelationship(details)
            } else {
                RelationshipPattern::Regular(details)
            };
            elements.push(PatternElement::Relationship(rel));
            // Parse the next node
            let (after_node, node) = node_pattern(after_right)?;
            elements.push(PatternElement::Node(node));
            input = after_node;
        } else {
            break;
        }
    }
    println!("Pattern element sequence result: {:?}", elements);
    Ok((input, elements))
}

#[allow(dead_code)]
// TODO: Clean up unused functions or refactor to use them
pub fn quantified_path_pattern(input: &str) -> IResult<&str, MatchElement> {
    println!("Parsing quantified path pattern: {}", input);
    // Parse optional path variable
    let (input, path_var) = opt(preceded(
        tuple((multispace0, char('='), multispace0)),
        map(identifier, |s| s.to_string()),
    ))(input)?;
    println!(
        "After parsing path_var: {:?}, remaining input: {}",
        path_var, input
    );

    // Parse the pattern
    let (input, _) = char('(')(input)?;
    let (input, pattern) = pattern_element_sequence(input)?;
    let (input, _) = char(')')(input)?;
    println!(
        "After parsing pattern: {:?}, remaining input: {}",
        pattern, input
    );

    // Parse length range
    let (input, length) = opt(length_range)(input)?;
    println!(
        "After parsing length: {:?}, remaining input: {}",
        length, input
    );

    // Parse optional WHERE clause
    let (input, where_clause) = opt(where_clause)(input)?;
    println!(
        "After parsing where_clause: {:?}, remaining input: {}",
        where_clause, input
    );

    Ok((
        input,
        MatchElement::QuantifiedPathPattern(QuantifiedPathPattern {
            pattern,
            min: length.as_ref().and_then(|l| l.min),
            max: length.as_ref().and_then(|l| l.max),
            where_clause,
            path_variable: path_var,
        }),
    ))
}

#[allow(dead_code)]
// TODO: Clean up unused functions or refactor to use them
pub fn match_element(input: &str) -> IResult<&str, MatchElement> {
    println!("Parsing match element: {}", input);
    if let Ok((input2, qpp)) = quantified_path_pattern(input) {
        println!("Found quantified path pattern: {:?}", qpp);
        return Ok((input2, qpp));
    }
    let (input, pattern) = pattern_element_sequence(input)?;
    println!("Found regular pattern: {:?}", pattern);
    Ok((input, MatchElement::Pattern(pattern)))
}

pub fn pattern(input: &str) -> IResult<&str, Vec<PatternElement>> {
    let (input, elements) = many1(alt((
        map(node_pattern, PatternElement::Node),
        map(relationship_details, |details| {
            PatternElement::Relationship(RelationshipPattern::Regular(details))
        }),
    )))(input)?;
    println!("Found regular pattern: {:?}", elements);
    Ok((input, elements))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional_relationship() {
        let input = "OPTIONAL (a)-[:KNOWS]->(b)";
        let result = pattern_element_sequence(input);
        if let Err(e) = &result {
            println!("Parser error: {:?}", e);
        }
        assert!(result.is_ok());
        let (_, pattern) = result.unwrap();
        assert_eq!(pattern.len(), 3);
        match &pattern[1] {
            PatternElement::Relationship(RelationshipPattern::OptionalRelationship(details)) => {
                assert_eq!(details.rel_type, Some("KNOWS".to_string()));
                assert_eq!(details.direction, Direction::Right);
            }
            _ => panic!("Expected OptionalRelationship"),
        }
    }

    #[test]
    fn test_regular_relationship() {
        let input = "(a)-[:KNOWS]->(b)";
        let result = pattern_element_sequence(input);
        if let Err(e) = &result {
            println!("Parser error: {:?}", e);
        }
        assert!(result.is_ok());
        let (_, pattern) = result.unwrap();
        assert_eq!(pattern.len(), 3);
        match &pattern[1] {
            PatternElement::Relationship(RelationshipPattern::Regular(details)) => {
                assert_eq!(details.rel_type, Some("KNOWS".to_string()));
                assert_eq!(details.direction, Direction::Right);
            }
            _ => panic!("Expected Regular relationship"),
        }
    }

    #[test]
    fn test_direction_assignment() {
        let input = "MATCH (a)-[:KNOWS]->(b)";
        let (_, clause) = match_clause(input).unwrap();
        let MatchElement::Pattern(pattern) = &clause.elements[0] else {
            panic!()
        };
        let PatternElement::Relationship(RelationshipPattern::Regular(details)) = &pattern[1]
        else {
            panic!()
        };
        assert_eq!(details.direction, Direction::Right);
    }

    #[test]
    fn test_node_pattern() {
        let input = "(a:Person {name: 'Alice'})";
        let result = node_pattern(input);
        assert!(result.is_ok());
        let (_, node) = result.unwrap();
        assert_eq!(node.variable, Some("a".to_string()));
        assert_eq!(node.label, Some("Person".to_string()));
    }

    #[test]
    fn test_relationship_pattern() {
        let input = "[r:KNOWS {since: 2020}]";
        let result = relationship_pattern(input);
        assert!(result.is_ok());
        let (_, rel) = result.unwrap();
        match rel {
            RelationshipPattern::Regular(details) => {
                assert_eq!(details.variable, Some("r".to_string()));
                assert_eq!(details.rel_type, Some("KNOWS".to_string()));
            }
            _ => panic!("Expected Regular relationship pattern"),
        }
    }

    #[test]
    fn test_quantified_path_pattern() {
        let input = "((a)-[:KNOWS]->(b)){1,3}";
        let result = quantified_path_pattern(input);
        assert!(result.is_ok());
        let (_, match_elem) = result.unwrap();
        match match_elem {
            MatchElement::QuantifiedPathPattern(qpp) => {
                assert_eq!(qpp.min, Some(1));
                assert_eq!(qpp.max, Some(3));
            }
            _ => panic!("Expected QuantifiedPathPattern"),
        }
    }
}
