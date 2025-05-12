use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum PatternElement {
    Node(NodePattern),
    Relationship(RelationshipPattern),
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub key: String,
    pub value: PropertyValue,
}

#[derive(Debug, PartialEq)]
pub enum PropertyValue {
    String(String),
    Number(i64),
}

#[derive(Debug, PartialEq)]
pub struct NodePattern {
    pub variable: Option<String>,
    pub label: Option<String>,
    pub properties: Option<Vec<Property>>,
}

#[derive(Debug, PartialEq)]
pub struct LengthRange {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub struct RelationshipPattern {
    pub variable: Option<String>,
    pub direction: Direction,
    pub properties: Option<Vec<Property>>,
    pub rel_type: Option<String>,
    pub length: Option<LengthRange>,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Undirected,
}

#[derive(Debug, PartialEq)]
pub enum MatchElement {
    Pattern(Vec<PatternElement>),
    QuantifiedPathPattern(QuantifiedPathPattern),
}

#[derive(Debug, PartialEq)]
pub struct QuantifiedPathPattern {
    pub pattern: Vec<PatternElement>,
    pub min: Option<u32>,
    pub max: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub struct MatchClause {
    pub elements: Vec<MatchElement>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnClause {
    pub items: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct Query {
    pub match_clause: MatchClause,
    pub return_clause: Option<ReturnClause>,
}

fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

fn string_literal(input: &str) -> IResult<&str, String> {
    let (input, _) = char('"')(input)?;
    let (input, s) = take_while1(|c| c != '"')(input)?;
    let (input, _) = char('"')(input)?;
    Ok((input, s.to_string()))
}

fn number_literal(input: &str) -> IResult<&str, i64> {
    let (input, n) = take_while1(|c: char| c.is_ascii_digit())(input)?;
    Ok((input, n.parse().unwrap()))
}

fn property_value(input: &str) -> IResult<&str, PropertyValue> {
    alt((
        map(string_literal, PropertyValue::String),
        map(number_literal, PropertyValue::Number),
    ))(input)
}

fn property(input: &str) -> IResult<&str, Property> {
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

fn property_map(input: &str) -> IResult<&str, Vec<Property>> {
    delimited(
        tuple((multispace0, char('{'))),
        separated_list1(tuple((multispace0, char(','), multispace0)), property),
        tuple((multispace0, char('}'))),
    )(input)
}

fn node_pattern(input: &str) -> IResult<&str, NodePattern> {
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, var) = opt_identifier(input)?;
    let (input, label) = opt(preceded(tuple((multispace0, char(':'))), identifier))(input)?;
    let (input, properties) = opt(preceded(multispace0, property_map))(input)?;
    let (input, _) = char(')')(input)?;
    Ok((
        input,
        NodePattern {
            variable: var,
            label: label.map(|s| s.to_string()),
            properties,
        },
    ))
}

fn opt_identifier(input: &str) -> IResult<&str, Option<String>> {
    let (input, _) = multispace0(input)?;
    match identifier(input) {
        Ok((input, id)) => Ok((input, Some(id.to_string()))),
        Err(_) => Ok((input, None)),
    }
}

fn length_range(input: &str) -> IResult<&str, LengthRange> {
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

fn relationship_pattern(input: &str) -> IResult<&str, RelationshipPattern> {
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
        // support for -[:TYPE*1..3]-> (no variable)
        opt(length_range)(input)?
    } else {
        (input, None)
    };
    let (input, _) = multispace0(input)?;
    let (input, right) = opt(alt((tag("->"), tag("-"))))(input)?;

    let (variable, rel_type, properties, rel_length) = rel
        .unwrap_or((None, None, None, None));
    let direction = match (left, right) {
        (Some("<-"), _) => Direction::Left,
        (_, Some("->")) => Direction::Right,
        _ => Direction::Undirected,
    };
    Ok((
        input,
        RelationshipPattern {
            variable,
            direction,
            properties,
            rel_type: rel_type.map(|s| s.to_string()),
            length: rel_length.or(length),
        },
    ))
}

fn pattern_element_sequence(input: &str) -> IResult<&str, Vec<PatternElement>> {
    let (input, first_node) = node_pattern(input)?;
    let mut elements = vec![PatternElement::Node(first_node)];
    let mut input = input;
    loop {
        let res = relationship_pattern(input);
        if let Ok((input2, rel)) = res {
            let (input3, node) = node_pattern(input2)?;
            elements.push(PatternElement::Relationship(rel));
            elements.push(PatternElement::Node(node));
            input = input3;
        } else {
            break;
        }
    }
    Ok((input, elements))
}

fn quantified_path_pattern(input: &str) -> IResult<&str, MatchElement> {
    let (input, pattern) = delimited(
        tuple((multispace0, char('('))),
        pattern_element_sequence,
        tuple((char(')'), multispace0)),
    )(input)?;
    let (input, quant) = delimited(
        char('{'),
        tuple((
            opt(map(number_literal, |n| n as u32)),
            opt(preceded(
                tuple((char(','), multispace0)),
                map(number_literal, |n| n as u32),
            )),
        )),
        char('}'),
    )(input)?;
    let (min, max) = quant;
    Ok((
        input,
        MatchElement::QuantifiedPathPattern(QuantifiedPathPattern { pattern, min, max }),
    ))
}

fn match_element(input: &str) -> IResult<&str, MatchElement> {
    // Try quantified path pattern first
    if let Ok((input2, qpp)) = quantified_path_pattern(input) {
        return Ok((input2, qpp));
    }
    // Otherwise, parse a normal pattern sequence
    let (input, pattern) = pattern_element_sequence(input)?;
    Ok((input, MatchElement::Pattern(pattern)))
}

fn match_element_list(input: &str) -> IResult<&str, Vec<MatchElement>> {
    separated_list1(tuple((multispace0, char(','), multispace0)), match_element)(input)
}

pub fn match_clause(input: &str) -> IResult<&str, MatchClause> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("MATCH")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, elements) = match_element_list(input)?;
    Ok((input, MatchClause { elements }))
}

fn return_clause(input: &str) -> IResult<&str, ReturnClause> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("RETURN")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) =
        separated_list1(tuple((multispace0, char(','), multispace0)), identifier)(input)?;
    Ok((
        input,
        ReturnClause {
            items: items.into_iter().map(|s| s.to_string()).collect(),
        },
    ))
}

pub fn query(input: &str) -> IResult<&str, Query> {
    let (input, match_clause) = match_clause(input)?;
    let (input, return_clause) = opt(return_clause)(input)?;
    Ok((
        input,
        Query {
            match_clause,
            return_clause,
        },
    ))
}
