use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::parser::ast::{Quantifier, *};
use crate::parser::utils::{identifier, number_literal, string_literal};

// Shared property value parser
pub fn property_value(input: &str) -> IResult<&str, PropertyValue> {
    alt((
        map(string_literal, PropertyValue::String),
        map(number_literal, PropertyValue::Number),
        map(function_call, |(name, args)| PropertyValue::FunctionCall {
            name,
            args: args.into_iter().map(PropertyValue::String).collect(),
        }),
    ))(input)
}

// Parses a function call
pub fn function_call(input: &str) -> IResult<&str, (String, Vec<String>)> {
    let (input, function) = map(identifier, |s| s.to_string())(input)?;
    let (input, _) = char('(')(input)?;
    let (input, args) = separated_list0(
        tuple((multispace0, char(','), multispace0)),
        alt((
            map(char('*'), |_| "*".to_string()),
            map(identifier, |s| s.to_string()),
            map(string_literal, |s| s),
            map(number_literal, |n| n.to_string()),
        )),
    )(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, (function, args)))
}

// Shared property parser
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

// Shared property map parser
pub fn property_map(input: &str) -> IResult<&str, Vec<Property>> {
    let (input, _) = char('{')(input)?;
    let (input, properties) =
        separated_list0(tuple((multispace0, char(','), multispace0)), property)(input)?;
    let (input, _) = char('}')(input)?;
    Ok((input, properties))
}

// Shared relationship type parser
pub fn relationship_type(input: &str) -> IResult<&str, String> {
    let (input, _) = char(':')(input)?;
    let (input, first_type) = take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)?;
    let mut types = vec![first_type];
    let mut input = input;
    // Parse additional types separated by |
    while let Ok((next_input, _)) = char::<&str, nom::error::Error<&str>>('|')(input) {
        let (next_input, next_type) =
            take_while1(|c: char| c.is_alphanumeric() || c == '_')(next_input)?;
        types.push(next_type);
        input = next_input;
    }
    let rel_type = types.join("|");
    println!(
        "Parsed relationship type: {}, remaining input: {}",
        rel_type, input
    );
    Ok((input, rel_type))
}

// Variable length relationship parser
pub fn variable_length_relationship(input: &str) -> IResult<&str, (String, Quantifier, bool)> {
    println!(
        "DEBUG: Starting variable_length_relationship with input: {}",
        input
    );
    let (input, rel_type) = relationship_type(input)?;
    println!("DEBUG: Parsed relationship type: {}", rel_type);
    let (input, (quantifier, is_optional)) = quantifier(input)?;
    println!(
        "DEBUG: Parsed quantifier: {:?}, optional: {}",
        quantifier, is_optional
    );
    Ok((input, (rel_type, quantifier, is_optional)))
}

// Shared length range parser
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

// Shared relationship details parser
pub fn relationship_details(input: &str) -> IResult<&str, RelationshipDetails> {
    println!("DEBUG: Starting relationship_details with input: {}", input);
    let (input, _) = char('[')(input)?;
    println!("DEBUG: After opening bracket: {}", input);
    let (input, variable) = opt(identifier)(input)?;
    println!("DEBUG: Parsed variable: {:?}", variable);

    // Try to parse as variable length relationship first
    let (input, rel_type_quantifier_optional) =
        if let Ok((input, (rel_type, quantifier, is_optional))) =
            variable_length_relationship(input)
        {
            println!(
                "DEBUG: Parsed as variable length relationship: {:?}, {:?}, optional: {}",
                rel_type, quantifier, is_optional
            );
            (input, (Some(rel_type), Some(quantifier), is_optional))
        } else {
            // Fall back to regular relationship type
            let (input, rel_type) = opt(relationship_type)(input)?;
            println!("DEBUG: Parsed as regular relationship type: {:?}", rel_type);
            (input, (rel_type, None, false))
        };

    let (input, _) = multispace0(input)?;
    let (input, properties) = opt(property_map)(input)?;
    println!("DEBUG: Parsed properties: {:?}", properties);
    let (input, _) = char(']')(input)?;
    println!("DEBUG: After closing bracket: {}", input);

    Ok((
        input,
        RelationshipDetails {
            variable: variable.map(|s| s.to_string()),
            direction: Direction::Undirected, // Will be set by pattern parser
            properties,
            rel_type: rel_type_quantifier_optional.0,
            length: None,
            where_clause: None,
            quantifier: rel_type_quantifier_optional.1,
            is_optional: rel_type_quantifier_optional.2,
        },
    ))
}

// Parse quantifiers like *, +, {n}, {n,m}, and allow ? after quantifier
pub fn quantifier(input: &str) -> IResult<&str, (Quantifier, bool)> {
    println!("DEBUG: Starting quantifier with input: {}", input);
    let mut input = input;
    let mut quant = None;
    // Try to parse *n..m
    if let Ok((rest, _)) = char::<&str, nom::error::Error<&str>>('*')(input) {
        input = rest;
        // Try *n..m
        if let Ok((rest, min)) = digit1::<&str, nom::error::Error<&str>>(input) {
            input = rest;
            if let Ok((rest, _)) = tag::<&str, &str, nom::error::Error<&str>>("..")(input) {
                input = rest;
                if let Ok((rest, max)) = digit1::<&str, nom::error::Error<&str>>(input) {
                    input = rest;
                    quant = Some(Quantifier {
                        min: Some(min.parse().unwrap()),
                        max: Some(max.parse().unwrap()),
                    });
                } else {
                    quant = Some(Quantifier {
                        min: Some(min.parse().unwrap()),
                        max: None,
                    });
                }
            } else {
                quant = Some(Quantifier {
                    min: Some(min.parse().unwrap()),
                    max: Some(min.parse().unwrap()),
                });
            }
        } else {
            // Just *
            quant = Some(Quantifier {
                min: Some(0),
                max: None,
            });
        }
    } else if let Ok((rest, _)) = char::<&str, nom::error::Error<&str>>('+')(input) {
        input = rest;
        quant = Some(Quantifier {
            min: Some(1),
            max: None,
        });
    }
    // If quantifier was parsed, check for ?
    if let Some(q) = quant {
        let (input, is_optional) =
            if let Ok((rest, _)) = char::<&str, nom::error::Error<&str>>('?')(input) {
                (rest, true)
            } else {
                (input, false)
            };
        println!(
            "DEBUG: Parsed quantifier: {:?}, optional: {}",
            q, is_optional
        );
        return Ok((input, (q, is_optional)));
    }
    println!("DEBUG: No quantifier found");
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Char,
    )))
}

pub fn relationship_pattern(input: &str) -> IResult<&str, RelationshipPattern> {
    println!("DEBUG: Starting relationship_pattern with input: {}", input);
    let (input, _) = multispace0(input)?;
    println!("DEBUG: After whitespace: {}", input);

    // Parse the left side of the relationship
    let (input, left) = alt((tag("<-"), tag("-")))(input)?;
    println!("DEBUG: Parsed left side: {}", left);

    // Parse relationship details
    let (input, mut details) = relationship_details(input)?;
    println!("DEBUG: Parsed relationship details: {:?}", details);

    // Parse the right side of the relationship
    let (input, right) = alt((tag("->"), tag("-")))(input)?;
    println!("DEBUG: Parsed right side: {}", right);

    // Set direction based on arrows, ensuring it's set even for variable length relationships
    details.direction = match (left, right) {
        ("-", "->") => Direction::Right,
        ("<-", "-") => Direction::Left,
        ("-", "-") => Direction::Undirected,
        _ => Direction::Undirected,
    };
    println!("DEBUG: Set direction to: {:?}", details.direction);

    // For variable length relationships, ensure direction is properly set
    if details.quantifier.is_some() {
        println!(
            "DEBUG: Variable length relationship with direction: {:?}",
            details.direction
        );
    }

    if details.is_optional {
        return Ok((input, RelationshipPattern::OptionalRelationship(details)));
    }
    Ok((input, RelationshipPattern::Regular(details)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Direction, PropertyValue, RelationshipPattern};

    // ===== PROPERTY VALUE TESTS =====
    #[test]
    fn test_property_value_string() {
        let result = property_value("'hello world'");
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        assert!(matches!(value, PropertyValue::String(s) if s == "hello world"));
    }

    #[test]
    fn test_property_value_number() {
        let result = property_value("42");
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        assert!(matches!(value, PropertyValue::Number(n) if n == 42));
    }

    #[test]
    fn test_property_value_function_call() {
        let result = property_value("timestamp()");
        assert!(result.is_ok());
        let (_, value) = result.unwrap();
        assert!(
            matches!(value, PropertyValue::FunctionCall { name, args } if name == "timestamp" && args.is_empty())
        );
    }

    #[test]
    fn test_property_value_invalid() {
        let result = property_value("invalid");
        assert!(result.is_err());
    }

    // ===== FUNCTION CALL TESTS =====
    #[test]
    fn test_function_call_simple() {
        let result = function_call("length(name)");
        assert!(result.is_ok());
        let (_, (func, args)) = result.unwrap();
        assert_eq!(func, "length");
        assert_eq!(args, vec!["name"]);
    }

    #[test]
    fn test_function_call_multiple_args() {
        let result = function_call("substring(name, 0, 5)");
        assert!(result.is_ok());
        let (_, (func, args)) = result.unwrap();
        assert_eq!(func, "substring");
        assert_eq!(args, vec!["name", "0", "5"]);
    }

    #[test]
    fn test_function_call_with_string_literal() {
        let result = function_call("coalesce(name, 'Unknown')");
        assert!(result.is_ok());
        let (_, (func, args)) = result.unwrap();
        assert_eq!(func, "coalesce");
        assert_eq!(args, vec!["name", "Unknown"]); // string_literal strips the quotes
    }

    #[test]
    fn test_function_call_with_wildcard() {
        let result = function_call("count(*)");
        assert!(result.is_ok());
        let (_, (func, args)) = result.unwrap();
        assert_eq!(func, "count");
        assert_eq!(args, vec!["*"]);
    }

    #[test]
    fn test_function_call_invalid() {
        let result = function_call("invalid");
        assert!(result.is_err());
    }

    // ===== PROPERTY TESTS =====
    #[test]
    fn test_property_simple() {
        let result = property("name: 'Alice'");
        assert!(result.is_ok());
        let (_, prop) = result.unwrap();
        assert_eq!(prop.key, "name");
        assert!(matches!(prop.value, PropertyValue::String(s) if s == "Alice"));
    }

    #[test]
    fn test_property_number() {
        let result = property("age: 30");
        assert!(result.is_ok());
        let (_, prop) = result.unwrap();
        assert_eq!(prop.key, "age");
        assert!(matches!(prop.value, PropertyValue::Number(n) if n == 30));
    }

    #[test]
    fn test_property_with_whitespace() {
        let result = property("  name  :  'Alice'  ");
        assert!(result.is_ok());
        let (_, prop) = result.unwrap();
        assert_eq!(prop.key, "name");
        assert!(matches!(prop.value, PropertyValue::String(s) if s == "Alice"));
    }

    #[test]
    fn test_property_invalid() {
        let result = property("invalid");
        assert!(result.is_err());
    }

    // ===== PROPERTY MAP TESTS =====
    #[test]
    fn test_property_map_empty() {
        let result = property_map("{}");
        assert!(result.is_ok());
        let (_, props) = result.unwrap();
        assert!(props.is_empty());
    }

    #[test]
    fn test_property_map_single() {
        let result = property_map("{name: 'Alice'}");
        assert!(result.is_ok());
        let (_, props) = result.unwrap();
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, "name");
        assert!(matches!(&props[0].value, PropertyValue::String(s) if s == "Alice"));
    }

    #[test]
    fn test_property_map_multiple() {
        let result = property_map("{name: 'Alice', age: 30}");
        assert!(result.is_ok());
        let (_, props) = result.unwrap();
        assert_eq!(props.len(), 2);

        let name_prop = props.iter().find(|p| p.key == "name").unwrap();
        assert!(matches!(&name_prop.value, PropertyValue::String(s) if s == "Alice"));

        let age_prop = props.iter().find(|p| p.key == "age").unwrap();
        assert!(matches!(&age_prop.value, PropertyValue::Number(n) if n == &30));
    }

    #[test]
    fn test_property_map_invalid() {
        let result = property_map("invalid");
        assert!(result.is_err());
    }

    // ===== RELATIONSHIP TYPE TESTS =====
    #[test]
    fn test_relationship_type_simple() {
        let result = relationship_type(":KNOWS");
        assert!(result.is_ok());
        let (_, rel_type) = result.unwrap();
        assert_eq!(rel_type, "KNOWS");
    }

    #[test]
    fn test_relationship_type_with_underscore() {
        let result = relationship_type(":ACTED_IN");
        assert!(result.is_ok());
        let (_, rel_type) = result.unwrap();
        assert_eq!(rel_type, "ACTED_IN");
    }

    #[test]
    fn test_relationship_type_multiple() {
        let result = relationship_type(":KNOWS|FRIENDS|COLLEAGUES");
        assert!(result.is_ok());
        let (_, rel_type) = result.unwrap();
        assert_eq!(rel_type, "KNOWS|FRIENDS|COLLEAGUES");
    }

    #[test]
    fn test_relationship_type_invalid() {
        let result = relationship_type("KNOWS"); // Missing colon
        assert!(result.is_err());
    }

    #[test]
    fn test_relationship_type_empty() {
        let result = relationship_type(":");
        assert!(result.is_err());
    }

    // ===== VARIABLE LENGTH RELATIONSHIP TESTS =====
    #[test]
    fn test_variable_length_relationship_star() {
        let result = variable_length_relationship(":KNOWS*");
        assert!(result.is_ok());
        let (_, (rel_type, quantifier, is_optional)) = result.unwrap();
        assert_eq!(rel_type, "KNOWS");
        assert_eq!(quantifier.min, Some(0));
        assert_eq!(quantifier.max, None);
        assert!(!is_optional);
    }

    #[test]
    fn test_variable_length_relationship_plus() {
        let result = variable_length_relationship(":KNOWS+");
        assert!(result.is_ok());
        let (_, (rel_type, quantifier, is_optional)) = result.unwrap();
        assert_eq!(rel_type, "KNOWS");
        assert_eq!(quantifier.min, Some(1));
        assert_eq!(quantifier.max, None);
        assert!(!is_optional);
    }

    #[test]
    fn test_variable_length_relationship_exact() {
        let result = variable_length_relationship(":KNOWS*5");
        assert!(result.is_ok());
        let (_, (rel_type, quantifier, is_optional)) = result.unwrap();
        assert_eq!(rel_type, "KNOWS");
        assert_eq!(quantifier.min, Some(5));
        assert_eq!(quantifier.max, Some(5));
        assert!(!is_optional);
    }

    #[test]
    fn test_variable_length_relationship_range() {
        let result = variable_length_relationship(":KNOWS*1..5");
        assert!(result.is_ok());
        let (_, (rel_type, quantifier, is_optional)) = result.unwrap();
        assert_eq!(rel_type, "KNOWS");
        assert_eq!(quantifier.min, Some(1));
        assert_eq!(quantifier.max, Some(5));
        assert!(!is_optional);
    }

    #[test]
    fn test_variable_length_relationship_optional() {
        let result = variable_length_relationship(":KNOWS*?");
        assert!(result.is_ok());
        let (_, (rel_type, quantifier, is_optional)) = result.unwrap();
        assert_eq!(rel_type, "KNOWS");
        assert_eq!(quantifier.min, Some(0));
        assert_eq!(quantifier.max, None);
        assert!(is_optional);
    }

    // ===== LENGTH RANGE TESTS =====
    #[test]
    fn test_length_range_empty() {
        let result = length_range("{,}");
        assert!(result.is_ok());
        let (_, range) = result.unwrap();
        assert_eq!(range.min, None);
        assert_eq!(range.max, None);
    }

    #[test]
    fn test_length_range_min_only() {
        let result = length_range("{5,}");
        assert!(result.is_ok());
        let (_, range) = result.unwrap();
        assert_eq!(range.min, Some(5));
        assert_eq!(range.max, None);
    }

    #[test]
    fn test_length_range_max_only() {
        let result = length_range("{,10}");
        assert!(result.is_ok());
        let (_, range) = result.unwrap();
        assert_eq!(range.min, None);
        assert_eq!(range.max, Some(10));
    }

    #[test]
    fn test_length_range_both() {
        let result = length_range("{1,5}");
        assert!(result.is_ok());
        let (_, range) = result.unwrap();
        assert_eq!(range.min, Some(1));
        assert_eq!(range.max, Some(5));
    }

    #[test]
    fn test_length_range_invalid() {
        let result = length_range("invalid");
        assert!(result.is_err());
    }

    // ===== RELATIONSHIP DETAILS TESTS =====
    #[test]
    fn test_relationship_details_simple() {
        let result = relationship_details("[r:KNOWS]");
        assert!(result.is_ok());
        let (_, details) = result.unwrap();
        assert_eq!(details.variable, Some("r".to_string()));
        assert_eq!(details.rel_type, Some("KNOWS".to_string()));
        assert!(details.properties.is_none());
        assert!(details.quantifier.is_none());
        assert!(!details.is_optional);
    }

    #[test]
    fn test_relationship_details_with_properties() {
        let result = relationship_details("[r:KNOWS {since: '2020'}]");
        assert!(result.is_ok());
        let (_, details) = result.unwrap();
        assert_eq!(details.variable, Some("r".to_string()));
        assert_eq!(details.rel_type, Some("KNOWS".to_string()));
        assert!(details.properties.is_some());
        assert_eq!(details.properties.as_ref().unwrap().len(), 1);
        assert_eq!(details.properties.as_ref().unwrap()[0].key, "since");
    }

    #[test]
    fn test_relationship_details_variable_length() {
        let result = relationship_details("[r:KNOWS*1..5]");
        assert!(result.is_ok());
        let (_, details) = result.unwrap();
        assert_eq!(details.variable, Some("r".to_string()));
        assert_eq!(details.rel_type, Some("KNOWS".to_string()));
        assert!(details.quantifier.is_some());
        let quant = details.quantifier.as_ref().unwrap();
        assert_eq!(quant.min, Some(1));
        assert_eq!(quant.max, Some(5));
    }

    #[test]
    fn test_relationship_details_no_variable() {
        let result = relationship_details("[:KNOWS]");
        assert!(result.is_ok());
        let (_, details) = result.unwrap();
        assert_eq!(details.variable, None);
        assert_eq!(details.rel_type, Some("KNOWS".to_string()));
    }

    #[test]
    fn test_relationship_details_no_type() {
        let result = relationship_details("[r]");
        assert!(result.is_ok());
        let (_, details) = result.unwrap();
        assert_eq!(details.variable, Some("r".to_string()));
        assert_eq!(details.rel_type, None);
    }

    #[test]
    fn test_relationship_details_invalid() {
        let result = relationship_details("invalid");
        assert!(result.is_err());
    }

    // ===== QUANTIFIER TESTS =====
    #[test]
    fn test_quantifier_star() {
        let result = quantifier("*");
        assert!(result.is_ok());
        let (_, (quant, is_optional)) = result.unwrap();
        assert_eq!(quant.min, Some(0));
        assert_eq!(quant.max, None);
        assert!(!is_optional);
    }

    #[test]
    fn test_quantifier_plus() {
        let result = quantifier("+");
        assert!(result.is_ok());
        let (_, (quant, is_optional)) = result.unwrap();
        assert_eq!(quant.min, Some(1));
        assert_eq!(quant.max, None);
        assert!(!is_optional);
    }

    #[test]
    fn test_quantifier_exact() {
        let result = quantifier("*5");
        assert!(result.is_ok());
        let (_, (quant, is_optional)) = result.unwrap();
        assert_eq!(quant.min, Some(5));
        assert_eq!(quant.max, Some(5));
        assert!(!is_optional);
    }

    #[test]
    fn test_quantifier_range() {
        let result = quantifier("*1..5");
        assert!(result.is_ok());
        let (_, (quant, is_optional)) = result.unwrap();
        assert_eq!(quant.min, Some(1));
        assert_eq!(quant.max, Some(5));
        assert!(!is_optional);
    }

    #[test]
    fn test_quantifier_optional() {
        let result = quantifier("*?");
        assert!(result.is_ok());
        let (_, (quant, is_optional)) = result.unwrap();
        assert_eq!(quant.min, Some(0));
        assert_eq!(quant.max, None);
        assert!(is_optional);
    }

    #[test]
    fn test_quantifier_plus_optional() {
        let result = quantifier("+?");
        assert!(result.is_ok());
        let (_, (quant, is_optional)) = result.unwrap();
        assert_eq!(quant.min, Some(1));
        assert_eq!(quant.max, None);
        assert!(is_optional);
    }

    #[test]
    fn test_quantifier_invalid() {
        let result = quantifier("invalid");
        assert!(result.is_err());
    }

    // ===== RELATIONSHIP PATTERN TESTS =====
    #[test]
    fn test_relationship_pattern_right_directed() {
        let result = relationship_pattern("-[r:KNOWS]->");
        assert!(result.is_ok());
        let (_, pattern) = result.unwrap();
        match pattern {
            RelationshipPattern::Regular(details) => {
                assert_eq!(details.direction, Direction::Right);
                assert_eq!(details.variable, Some("r".to_string()));
                assert_eq!(details.rel_type, Some("KNOWS".to_string()));
            }
            _ => panic!("Expected regular relationship pattern"),
        }
    }

    #[test]
    fn test_relationship_pattern_left_directed() {
        let result = relationship_pattern("<-[r:KNOWS]-");
        assert!(result.is_ok());
        let (_, pattern) = result.unwrap();
        match pattern {
            RelationshipPattern::Regular(details) => {
                assert_eq!(details.direction, Direction::Left);
                assert_eq!(details.variable, Some("r".to_string()));
                assert_eq!(details.rel_type, Some("KNOWS".to_string()));
            }
            _ => panic!("Expected regular relationship pattern"),
        }
    }

    #[test]
    fn test_relationship_pattern_undirected() {
        let result = relationship_pattern("-[r:KNOWS]-");
        assert!(result.is_ok());
        let (_, pattern) = result.unwrap();
        match pattern {
            RelationshipPattern::Regular(details) => {
                assert_eq!(details.direction, Direction::Undirected);
                assert_eq!(details.variable, Some("r".to_string()));
                assert_eq!(details.rel_type, Some("KNOWS".to_string()));
            }
            _ => panic!("Expected regular relationship pattern"),
        }
    }

    #[test]
    fn test_relationship_pattern_with_properties() {
        let result = relationship_pattern("-[r:KNOWS {since: '2020'}]->");
        assert!(result.is_ok());
        let (_, pattern) = result.unwrap();
        match pattern {
            RelationshipPattern::Regular(details) => {
                assert_eq!(details.direction, Direction::Right);
                assert!(details.properties.is_some());
                assert_eq!(details.properties.as_ref().unwrap().len(), 1);
            }
            _ => panic!("Expected regular relationship pattern"),
        }
    }

    #[test]
    fn test_relationship_pattern_invalid() {
        let result = relationship_pattern("invalid");
        assert!(result.is_err());
    }
}
