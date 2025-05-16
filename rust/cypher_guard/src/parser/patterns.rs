use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::parser::ast::*;
use crate::parser::utils::{identifier, string_literal, number_literal, opt_identifier};

pub fn property_value(input: &str) -> IResult<&str, PropertyValue> {
    alt((
        map(string_literal, PropertyValue::String),
        map(number_literal, PropertyValue::Number),
    ))(input)
}

pub fn property(input: &str) -> IResult<&str, Property> {
    let (input, _) = multispace0(input)?;
    let (input, key) = identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = property_value(input)?;
    Ok((input, Property { key: key.to_string(), value }))
}

pub fn property_map(input: &str) -> IResult<&str, Vec<Property>> {
    delimited(
        tuple((multispace0, char('{'))),
        separated_list1(tuple((multispace0, char(','), multispace0)), property),
        tuple((multispace0, char('}'))),
    )(input)
}

pub fn node_pattern(input: &str) -> IResult<&str, NodePattern> {
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, var) = opt_identifier(input)?;
    let (input, label) = opt(preceded(tuple((multispace0, char(':'))), identifier))(input)?;
    let (input, properties) = opt(preceded(multispace0, property_map))(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, NodePattern {
        variable: var,
        label: label.map(|s| s.to_string()),
        properties,
    }))
}

pub fn length_range(input: &str) -> IResult<&str, LengthRange> {
    let (input, _) = char('*')(input)?;
    let (input, min) = opt(map(number_literal, |n| n as u32))(input)?;
    let (input, max) = if let Ok((input, _)) = char::<&str, nom::error::Error<&str>>('.')(input) {
        let (input, _) = char('.')(input)?;
        let (input, max) = opt(map(number_literal, |n| n as u32))(input)?;
        (input, max)
    } else {
        (input, None)
    };
    Ok((input, LengthRange { min, max }))
}

pub fn relationship_type(input: &str) -> IResult<&str, String> {
    let (input, _) = char(':')(input)?;
    let (input, rel_type) = identifier(input)?;
    Ok((input, rel_type.to_string()))
}

pub fn relationship_details(input: &str) -> IResult<&str, RelationshipDetails> {
    let (input, left) = opt(preceded(multispace0, alt((tag("<-"), tag("-")))))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, rel) = opt(delimited(
        char('['),
        tuple((
            opt_identifier,
            opt(preceded(tuple((multispace0, char(':'))), identifier)),
            opt(preceded(multispace0, property_map)),
            opt(length_range),
        )),
        char(']'),
    ))(input)?;
    let (input, length) = if rel.is_none() {
        opt(length_range)(input)?
    } else {
        (input, None)
    };
    let (input, _) = multispace0(input)?;
    let (input, right) = opt(alt((tag("->"), tag("-"))))(input)?;

    let (variable, rel_type, properties, rel_length) = rel.unwrap_or((None, None, None, None));
    let direction = match (left, right) {
        (Some("<-"), _) => Direction::Left,
        (_, Some("->")) => Direction::Right,
        _ => Direction::Undirected,
    };
    Ok((input, RelationshipDetails {
        variable,
        direction,
        properties,
        rel_type: rel_type.map(|s| s.to_string()),
        length: rel_length.or(length),
    }))
}

pub fn relationship(input: &str) -> IResult<&str, RelationshipPattern> {
    let (input, _) = char('-')(input)?;
    let (input, rel_type) = opt(relationship_type)(input)?;
    let (input, direction) = alt((
        map(tag("->"), |_| Direction::Right),
        map(tag("<-"), |_| Direction::Left),
        map(tag("-"), |_| Direction::Right),
    ))(input)?;
    let details = RelationshipDetails {
        rel_type,
        direction,
        length: None,
        properties: None,
        variable: None,
    };
    Ok((input, RelationshipPattern::Regular(details)))
}

pub fn pattern_element_sequence(input: &str) -> IResult<&str, Vec<PatternElement>> {
    let (input, is_optional) = opt(tuple((tag("OPTIONAL"), multispace1)))(input)?;
    let (mut input, first_node) = node_pattern(input)?;
    let mut elements = vec![PatternElement::Node(first_node)];
    loop {
        // Skip whitespace before attempting to parse another relationship
        let (rest, _) = multispace0(input)?;
        if rest.is_empty() {
            break;
        }
        // Try to parse a relationship segment: -[ ... ]-> or -[ ... ]-
        let rel_res = relationship_details(rest);
        if let Ok((input2, details)) = rel_res {
            let (input3, node) = node_pattern(input2)?;
            let rel = if is_optional.is_some() {
                RelationshipPattern::OptionalRelationship(details)
            } else {
                RelationshipPattern::Regular(details)
            };
            elements.push(PatternElement::Relationship(rel));
            elements.push(PatternElement::Node(node));
            input = input3;
        } else {
            break;
        }
    }
    Ok((input, elements))
}

pub fn quantified_path_pattern(input: &str) -> IResult<&str, MatchElement> {
    let (input, pattern) = delimited(
        tuple((multispace0, char('('))),
        pattern_element_sequence,
        tuple((char(')'), multispace0)),
    )(input)?;
    let (input, quant) = delimited(
        char('{'),
        tuple((
            opt(map(number_literal, |n| n as u32)),
            opt(preceded(tuple((char(','), multispace0)), map(number_literal, |n| n as u32))),
        )),
        char('}'),
    )(input)?;
    let (min, max) = quant;
    Ok((input, MatchElement::QuantifiedPathPattern(QuantifiedPathPattern { pattern, min, max })))
}

pub fn match_element(input: &str) -> IResult<&str, MatchElement> {
    if let Ok((input2, qpp)) = quantified_path_pattern(input) {
        return Ok((input2, qpp));
    }
    let (input, pattern) = pattern_element_sequence(input)?;
    Ok((input, MatchElement::Pattern(pattern)))
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
            },
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
            },
            _ => panic!("Expected Regular relationship"),
        }
    }
}
