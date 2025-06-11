use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::opt,
    sequence::{tuple, terminated, preceded},
    IResult,
};
use nom::branch::alt;

use crate::parser::ast::*;
use crate::parser::components::{property_map, relationship_type};
use crate::parser::utils::identifier;
use crate::parser::components::*;

#[cfg(test)]
use crate::parser::clauses::match_clause;

pub fn node_pattern(input: &str) -> IResult<&str, NodePattern> {
    println!("[node_pattern] ENTER: input='{}'", input);
    match char::<&str, nom::error::Error<&str>>('(')(input) {
        Ok((input, _)) => {
            println!("[node_pattern] After parsing '(': input='{}'", input);
            let (input, variable) = opt(identifier)(input)?;
            println!("[node_pattern] After parsing variable: {:?}, input='{}'", variable, input);
            let (input, label) = opt(preceded(char(':'), identifier))(input)?;
            println!("[node_pattern] After parsing label: {:?}, input='{}'", label, input);
            let (input, _) = multispace0(input)?;
            let (input, properties) = opt(property_map)(input)?;
            println!("[node_pattern] After parsing properties: {:?}, input='{}'", properties, input);
            let (input, _) = char(')')(input)?;
            println!("[node_pattern] After parsing ')': input='{}'", input);
            let result = NodePattern {
                variable: variable.map(|s| s.to_string()),
                label: label.map(|s| s.to_string()),
                properties,
            };
            println!("[node_pattern] EXIT: {:?}", result);
            Ok((input, result))
        },
        Err(e) => {
            println!("[node_pattern] Failed to parse '(': {:?}", e);
            Err(e)
        }
    }
}

pub fn relationship_details(input: &str) -> IResult<&str, RelationshipDetails> {
    println!("Parsing relationship details: {}", input);
    let (input, _) = char('[')(input)?;
    println!("After parsing '[': {}", input);
    let (input, variable) = opt(identifier)(input)?;
    println!("After parsing variable: {:?}, remaining input: {}", variable, input);
    
    // Try to parse as variable length relationship first
    let (input, rel_type_quantifier_optional) = if let Ok((input, (rel_type, quantifier, is_optional))) = variable_length_relationship(input) {
        println!("Parsed as variable length relationship: {:?}, {:?}, optional: {}", rel_type, quantifier, is_optional);
        (input, (Some(rel_type), Some(quantifier), is_optional))
    } else {
        // Fall back to regular relationship type
        let (input, rel_type) = opt(relationship_type)(input)?;
        println!("After parsing rel_type: {:?}, remaining input: {}", rel_type, input);
        (input, (rel_type, None, false))
    };
    
    let (input, _) = multispace0(input)?;
    let (input, properties) = opt(property_map)(input)?;
    println!("After parsing properties: {:?}, remaining input: {}", properties, input);
    let (input, _) = char(']')(input)?;
    println!("After parsing ']': {}", input);
    let (input, length) = opt(length_range)(input)?;
    println!("After parsing length: {:?}, remaining input: {}", length, input);
    
    // Create relationship details with initial direction
    let details = RelationshipDetails {
        variable: variable.map(|s| s.to_string()),
        direction: Direction::Undirected, // Will be set by pattern parser
        properties,
        rel_type: rel_type_quantifier_optional.0,
        length,
        where_clause: None, // Will be set by pattern parser if needed
        quantifier: rel_type_quantifier_optional.1,
        is_optional: rel_type_quantifier_optional.2,
    };
    
    // For variable length relationships, ensure we can handle direction
    if details.quantifier.is_some() {
        println!("DEBUG: Variable length relationship detected in details");
    }
    
    Ok((input, details))
}

pub fn relationship_pattern(input: &str) -> IResult<&str, RelationshipPattern> {
    println!("DEBUG: Starting relationship_pattern with input: {}", input);
    let (input, _) = multispace0(input)?;
    println!("DEBUG: After whitespace: {}", input);

    // Parse the left side of the relationship (either '-' or '<-')
    let (input, left_dir) = alt((tag("<-"), tag("-")))(input)?;
    println!("DEBUG: Parsed left direction: {}", left_dir);

    // Parse relationship details
    let (input, mut details) = relationship_details(input)?;
    println!("DEBUG: Parsed relationship details: {:?}", details);

    // Parse the right side of the relationship (either '->' or '-')
    let (input, right_dir) = alt((tag("->"), tag("-")))(input)?;
    println!("DEBUG: Parsed right direction: {}", right_dir);

    // Set direction based on arrows
    details.direction = match (left_dir, right_dir) {
        ("-", "->") => Direction::Right,
        ("<-", "-") => Direction::Left,
        ("-", "-") => Direction::Undirected,
        _ => Direction::Undirected,
    };
    println!("DEBUG: Set direction to: {:?}", details.direction);

    if details.is_optional {
        return Ok((input, RelationshipPattern::OptionalRelationship(details)));
    }
    Ok((input, RelationshipPattern::Regular(details)))
}

pub fn pattern_element_sequence<'a>(input: &'a str, allow_qpp: bool) -> IResult<&'a str, Vec<PatternElement>> {
    println!("[pattern_element_sequence] >>> ENTER: input='{}', allow_qpp={}", input, allow_qpp);
    let mut elements = Vec::new();
    let mut input = input;

    loop {
        println!("[pattern_element_sequence] LOOP: input='{}'", input);
        if allow_qpp {
            // Only try QPP if input starts with '(' and after the matching ')' is a '{'
            if let Some(rest) = input.strip_prefix('(') {
                // Find the matching closing parenthesis
                let mut depth = 1;
                let mut idx = 0;
                for (i, c) in rest.char_indices() {
                    if c == '(' {
                        depth += 1;
                    } else if c == ')' {
                        depth -= 1;
                        if depth == 0 {
                            idx = i;
                            break;
                        }
                    }
                }
                if depth == 0 {
                    let after_paren = &rest[idx + 1..];
                    let after_paren_trim = after_paren.trim_start();
                    if after_paren_trim.starts_with('{') {
                        // This is a QPP
                        match quantified_path_pattern(input) {
                            Ok((after, pattern)) => {
                                println!("[pattern_element_sequence] Parsed QPP: {:?}, after='{}'", pattern, after);
                                elements.push(pattern);
                                input = after;
                                continue;
                            },
                            Err(e) => {
                                println!("[pattern_element_sequence] quantified_path_pattern failed: {:?}", e);
                                break;
                            }
                        }
                    }
                }
            }
        }
        // Always try to parse a node first
        match node_pattern(input) {
            Ok((rest, node)) => {
                println!("[pattern_element_sequence] Parsed node: {:?}, rest='{}'", node, rest);
                elements.push(PatternElement::Node(node));
                input = rest;
            },
            Err(e) => {
                println!("[pattern_element_sequence] node_pattern failed: {:?}", e);
                break;
            }
        }
        // Now try to parse a relationship pattern and another node
        let rel_and_node = {
            let rel_result = relationship_pattern(input);
            if let Ok((rest, rel)) = rel_result {
                let node_result = node_pattern(rest);
                if let Ok((after_node, node)) = node_result {
                    Some((after_node, rel, node))
                } else {
                    None
                }
            } else {
                None
            }
        };
        if let Some((after_node, rel, node)) = rel_and_node {
            println!("[pattern_element_sequence] Parsed relationship and node: {:?}, {:?}, after_node='{}'", rel, node, after_node);
            elements.push(PatternElement::Relationship(rel));
            elements.push(PatternElement::Node(node));
            input = after_node;
        } else {
            break;
        }
    }
    println!("[pattern_element_sequence] <<< EXIT: elements={:?}, input='{}'", elements, input);
    Ok((input, elements))
}

pub fn match_element(input: &str) -> IResult<&str, MatchElement> {
    println!("[match_element] ENTER: input='{}'", input);
    let (input, path_var) = opt(terminated(identifier, tuple((multispace0, char('='), multispace0))))(input)?;
    println!("[match_element] After path variable: {:?}, input='{}'", path_var, input);
    let (input, pattern) = pattern_element_sequence(input, true)?;
    println!("[match_element] After pattern_element_sequence: pattern={:?}, input='{}'", pattern, input);
    Ok((
        input,
        MatchElement {
            path_var: path_var.map(|s| s.to_string()),
            pattern,
        },
    ))
}

pub fn pattern(input: &str) -> IResult<&str, Vec<PatternElement>> {
    pattern_element_sequence(input, true)
}

pub fn quantified_path_pattern(input: &str) -> IResult<&str, PatternElement> {
    println!("[quantified_path_pattern] >>> ENTER: input='{}'", input);
    let (input, _) = char('(')(input)?;
    println!("[quantified_path_pattern] After opening parenthesis: input='{}'", input);
    
    // Parse the inner pattern directly using pattern_element_sequence
    let (input, mut inner_pattern) = pattern_element_sequence(input, false)?;
    println!("[quantified_path_pattern] Parsed pattern_element_sequence: {:?}, input='{}'", inner_pattern, input);
    
    // Strip quantifiers from relationships inside the QPP
    for element in &mut inner_pattern {
        if let PatternElement::Relationship(rel) = element {
            match rel {
                RelationshipPattern::Regular(details) | RelationshipPattern::OptionalRelationship(details) => {
                    details.quantifier = None;
                }
            }
        }
    }
    
    // Extract direction from the pattern if it's a relationship
    let mut direction = Direction::Undirected;
    for element in &inner_pattern {
        if let PatternElement::Relationship(rel) = element {
            direction = match rel {
                RelationshipPattern::Regular(details) => details.direction.clone(),
                RelationshipPattern::OptionalRelationship(details) => details.direction.clone(),
            };
            break;
        }
    }
    
    println!("[quantified_path_pattern] Extracted direction: {:?}", direction);
    
    let (input, _) = char(')')(input)?;
    println!("[quantified_path_pattern] After closing parenthesis: input='{}'", input);
    let (input, _) = char('{')(input)?;
    println!("[quantified_path_pattern] After opening brace: input='{}'", input);
    let (input, min) = digit1(input)?;
    println!("[quantified_path_pattern] Parsed min: {}, input='{}'", min, input);
    let (input, _) = alt((tag(".."), tag(",")))(input)?;
    println!("[quantified_path_pattern] After separator: input='{}'", input);
    let (input, max) = digit1(input)?;
    println!("[quantified_path_pattern] Parsed max: {}, input='{}'", max, input);
    let (input, _) = char('}')(input)?;
    println!("[quantified_path_pattern] After closing brace: input='{}'", input);
    let min = min.parse::<u32>().unwrap();
    let max = max.parse::<u32>().unwrap();
    
    // Create the quantified pattern, preserving relationship directions
    let quantified_pattern = QuantifiedPathPattern {
        pattern: inner_pattern,
        min: Some(min),
        max: Some(max),
        where_clause: None,
        path_variable: None,
    };
    println!("[quantified_path_pattern] <<< EXIT: {:?}", quantified_pattern);
    Ok((input, PatternElement::QuantifiedPathPattern(quantified_pattern)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_length_relationship() {
        let input = "p = (a)-[r:KNOWS*1..3]->(b)";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.path_var, Some("p".to_string()));
        // The pattern should have 3 elements: Node, Relationship, Node
        assert_eq!(element.pattern.len(), 3);
        if let PatternElement::Relationship(rel) = &element.pattern[1] {
            assert_eq!(rel.rel_type(), Some("KNOWS"));
            assert!(rel.quantifier().is_some());
            if let Some(quant) = rel.quantifier() {
                assert_eq!(quant.min, Some(1));
                assert_eq!(quant.max, Some(3));
            }
        } else {
            panic!("Expected relationship inside pattern");
        }
    }

    #[test]
    fn test_variable_length_relationship_with_star() {
        let input = "p = (a)-[r:KNOWS*]->(b)";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.path_var, Some("p".to_string()));
        assert_eq!(element.pattern.len(), 3);
        if let PatternElement::Relationship(rel) = &element.pattern[1] {
            assert_eq!(rel.rel_type(), Some("KNOWS"));
            assert!(rel.quantifier().is_some());
            if let Some(quant) = rel.quantifier() {
                assert_eq!(quant.min, Some(0));
                assert_eq!(quant.max, None);
            }
        } else {
            panic!("Expected relationship inside pattern");
        }
    }

    #[test]
    fn test_variable_length_relationship_with_plus() {
        let input = "p = (a)-[r:KNOWS+]->(b)";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.path_var, Some("p".to_string()));
        assert_eq!(element.pattern.len(), 3);
        if let PatternElement::Relationship(rel) = &element.pattern[1] {
            assert_eq!(rel.rel_type(), Some("KNOWS"));
            assert!(rel.quantifier().is_some());
            if let Some(quant) = rel.quantifier() {
                assert_eq!(quant.min, Some(1));
                assert_eq!(quant.max, None);
            }
        } else {
            panic!("Expected relationship inside pattern");
        }
    }

    #[test]
    fn test_variable_length_relationship_with_exact_count() {
        let input = "p = (a)-[r:KNOWS*3]->(b)";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.path_var, Some("p".to_string()));
        assert_eq!(element.pattern.len(), 3);
        if let PatternElement::Relationship(rel) = &element.pattern[1] {
            assert_eq!(rel.rel_type(), Some("KNOWS"));
            assert!(rel.quantifier().is_some());
            if let Some(quant) = rel.quantifier() {
                assert_eq!(quant.min, Some(3));
                assert_eq!(quant.max, Some(3));
            }
        } else {
            panic!("Expected relationship inside pattern");
        }
    }

    #[test]
    fn test_variable_length_relationship_with_properties() {
        let input = "p = (a)-[r:KNOWS*1..3 {since: 2020}]->(b)";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.pattern.len(), 3);
        if let PatternElement::Relationship(rel) = &element.pattern[1] {
            assert!(rel.properties().is_some());
            if let Some(props) = rel.properties() {
                assert_eq!(props.len(), 1);
                assert_eq!(props[0].key, "since");
                assert_eq!(props[0].value, PropertyValue::Number(2020));
            }
        } else {
            panic!("Expected relationship with properties");
        }
    }

    #[test]
    fn test_variable_length_relationship_with_optional() {
        let input = "p = (a)-[r:KNOWS*1..3?]->(b)";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.pattern.len(), 3);
        if let PatternElement::Relationship(rel) = &element.pattern[1] {
            assert!(matches!(rel, RelationshipPattern::OptionalRelationship(_)));
        } else {
            panic!("Expected optional relationship");
        }
    }

    #[test]
    fn test_variable_length_relationship_with_multiple_types() {
        let input = "p = (a)-[r:KNOWS|FOLLOWS*1..3]->(b)";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.pattern.len(), 3);
        if let PatternElement::Relationship(rel) = &element.pattern[1] {
            assert_eq!(rel.rel_type(), Some("KNOWS|FOLLOWS"));
            assert!(rel.quantifier().is_some());
        } else {
            panic!("Expected relationship with multiple types");
        }
    }

    #[test]
    fn test_quantified_path_pattern() {
        let input = "p = ((a)-[r:KNOWS]->(b)){1,3}";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.path_var, Some("p".to_string()));
        assert_eq!(element.pattern.len(), 1);
        if let PatternElement::QuantifiedPathPattern(qpp) = &element.pattern[0] {
            assert_eq!(qpp.min, Some(1));
            assert_eq!(qpp.max, Some(3));
            assert_eq!(qpp.pattern.len(), 3); // Node, Relationship, Node
            // Verify that the relationship inside QPP doesn't have a quantifier
            if let PatternElement::Relationship(rel) = &qpp.pattern[1] {
                assert_eq!(rel.rel_type(), Some("KNOWS"));
                assert!(rel.quantifier().is_none());
            } else {
                panic!("Expected relationship inside QuantifiedPathPattern");
            }
        } else {
            panic!("Expected quantified path pattern");
        }
    }

    #[test]
    fn test_quantified_path_pattern_with_complex_pattern() {
        let input = "p = ((a)-[r1:KNOWS]->(b)-[r2:FOLLOWS]->(c)){1,3}";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.path_var, Some("p".to_string()));
        assert_eq!(element.pattern.len(), 1);
        if let PatternElement::QuantifiedPathPattern(qpp) = &element.pattern[0] {
            assert_eq!(qpp.min, Some(1));
            assert_eq!(qpp.max, Some(3));
            assert_eq!(qpp.pattern.len(), 5); // Node, Relationship, Node, Relationship, Node
            // Verify that relationships inside QPP don't have quantifiers
            if let PatternElement::Relationship(rel) = &qpp.pattern[1] {
                assert_eq!(rel.rel_type(), Some("KNOWS"));
                assert!(rel.quantifier().is_none());
            } else {
                panic!("Expected first relationship inside QuantifiedPathPattern");
            }
            if let PatternElement::Relationship(rel) = &qpp.pattern[3] {
                assert_eq!(rel.rel_type(), Some("FOLLOWS"));
                assert!(rel.quantifier().is_none());
            } else {
                panic!("Expected second relationship inside QuantifiedPathPattern");
            }
        } else {
            panic!("Expected quantified path pattern");
        }
    }

    #[test]
    fn test_optional_relationship() {
        let input = "(a)-[r:KNOWS]->(b)";
        let (_, pattern) = pattern_element_sequence(input, true).unwrap();
        assert_eq!(pattern.len(), 3);
        if let PatternElement::Relationship(rel) = &pattern[1] {
            assert_eq!(rel.direction(), Direction::Right);
        } else {
            panic!("Expected relationship");
        }
    }

    #[test]
    fn test_regular_relationship() {
        let input = "(a)-[r:KNOWS]-(b)";
        let (_, pattern) = pattern_element_sequence(input, true).unwrap();
        assert_eq!(pattern.len(), 3);
        if let PatternElement::Relationship(rel) = &pattern[1] {
            assert_eq!(rel.direction(), Direction::Undirected);
        } else {
            panic!("Expected relationship");
        }
    }

    #[test]
    fn test_direction_assignment() {
        let input = "(a)<-[r:KNOWS]-(b)";
        let (_, pattern) = pattern_element_sequence(input, true).unwrap();
        assert_eq!(pattern.len(), 3);
        if let PatternElement::Relationship(rel) = &pattern[1] {
            assert_eq!(rel.direction(), Direction::Left);
        } else {
            panic!("Expected relationship");
        }
    }

    #[test]
    fn test_node_pattern() {
        let input = "(a:Person {name: 'Alice'})";
        let (_, node) = node_pattern(input).unwrap();
        assert_eq!(node.variable, Some("a".to_string()));
        assert_eq!(node.label, Some("Person".to_string()));
        assert!(node.properties.is_some());
    }

    #[test]
    fn test_relationship_pattern() {
        let input = "[r:KNOWS {since: 2020}]";
        let (_, rel) = relationship_details(input).unwrap();
        assert_eq!(rel.variable, Some("r".to_string()));
        assert_eq!(rel.rel_type, Some("KNOWS".to_string()));
        assert!(rel.properties.is_some());
    }

    #[test]
    fn test_quantified_path_pattern_with_where_clause() {
        let input = "p = ((a)-[r:KNOWS]->(b) WHERE a.name = 'Alice'){1,3}";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.pattern.len(), 1);
        if let PatternElement::QuantifiedPathPattern(qpp) = &element.pattern[0] {
            assert!(qpp.where_clause.is_some());
            if let Some(where_clause) = &qpp.where_clause {
                assert_eq!(where_clause.conditions.len(), 1);
                if let WhereCondition::Comparison { left, operator, right } = &where_clause.conditions[0] {
                    assert_eq!(left, "a.name");
                    assert_eq!(operator, "=");
                    assert_eq!(right, "'Alice'");
                } else {
                    panic!("Expected comparison condition");
                }
            }
        } else {
            panic!("Expected quantified path pattern with where clause");
        }
    }

    #[test]
    fn test_quantified_path_pattern_with_path_variable() {
        let input = "p = (path = (a)-[r:KNOWS]->(b)){1,3}";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.pattern.len(), 1);
        if let PatternElement::QuantifiedPathPattern(qpp) = &element.pattern[0] {
            assert_eq!(qpp.path_variable, Some("path".to_string()));
        } else {
            panic!("Expected quantified path pattern with path variable");
        }
    }

    #[test]
    fn test_quantified_path_pattern_with_direction() {
        let input = "p = ((a)-[r:KNOWS]->(b)){1,3}";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.path_var, Some("p".to_string()));
        assert_eq!(element.pattern.len(), 1);
        if let PatternElement::QuantifiedPathPattern(qpp) = &element.pattern[0] {
            assert_eq!(qpp.min, Some(1));
            assert_eq!(qpp.max, Some(3));
            assert_eq!(qpp.pattern.len(), 3); // Node, Relationship, Node
            // Verify that the relationship inside QPP has the correct direction
            if let PatternElement::Relationship(rel) = &qpp.pattern[1] {
                assert_eq!(rel.rel_type(), Some("KNOWS"));
                assert_eq!(rel.direction(), Direction::Right);
                assert!(rel.quantifier().is_none());
            } else {
                panic!("Expected relationship inside QuantifiedPathPattern");
            }
        } else {
            panic!("Expected quantified path pattern");
        }
    }

    #[test]
    fn test_quantified_path_pattern_with_undirected_relationship() {
        let input = "p = ((a)-[r:KNOWS]-(b)){1,3}";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.path_var, Some("p".to_string()));
        assert_eq!(element.pattern.len(), 1);
        if let PatternElement::QuantifiedPathPattern(qpp) = &element.pattern[0] {
            assert_eq!(qpp.min, Some(1));
            assert_eq!(qpp.max, Some(3));
            assert_eq!(qpp.pattern.len(), 3); // Node, Relationship, Node
            // Verify that the relationship inside QPP has the correct direction
            if let PatternElement::Relationship(rel) = &qpp.pattern[1] {
                assert_eq!(rel.rel_type(), Some("KNOWS"));
                assert_eq!(rel.direction(), Direction::Undirected);
                assert!(rel.quantifier().is_none());
            } else {
                panic!("Expected relationship inside QuantifiedPathPattern");
            }
        } else {
            panic!("Expected quantified path pattern");
        }
    }
}
