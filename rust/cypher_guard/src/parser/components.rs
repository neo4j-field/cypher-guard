use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace0},
    combinator::{map, opt},
    multi::separated_list1,
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
    let (input, args) = nom::multi::separated_list0(
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
        separated_list1(tuple((multispace0, char(','), multispace0)), property)(input)?;
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
