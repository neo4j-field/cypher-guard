use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0},
    combinator::opt,
    sequence::{preceded, terminated, tuple},
    IResult,
};

use crate::parser::ast::{
    Direction, MatchElement, NodePattern, PatternElement, QuantifiedPathPattern,
    RelationshipDetails, RelationshipPattern,
};
use crate::parser::clauses::where_clause;
use crate::parser::components::*;
use crate::parser::components::{property_map, relationship_type};
use crate::parser::utils::identifier;

pub fn node_pattern(input: &str) -> IResult<&str, NodePattern> {
    match char::<&str, nom::error::Error<&str>>('(')(input) {
        Ok((input, _)) => {
            let (input, variable) = opt(identifier)(input)?;
            let (input, label) = opt(preceded(char(':'), identifier))(input)?;
            let (input, _) = multispace0(input)?;
            let (input, properties) = opt(property_map)(input)?;
            let (input, _) = char(')')(input)?;
            let result = NodePattern {
                variable: variable.map(|s| s.to_string()),
                label: label.map(|s| s.to_string()),
                properties,
            };
            Ok((input, result))
        }
        Err(e) => Err(e),
    }
}

pub fn relationship_details(input: &str) -> IResult<&str, RelationshipDetails> {
    let (input, _) = char('[')(input)?;
    println!("After parsing '[': {}", input);
    let (input, variable) = opt(identifier)(input)?;
    println!(
        "After parsing variable: {:?}, remaining input: {}",
        variable, input
    );

    // Try to parse as variable length relationship first
    let (input, rel_type_quantifier_optional) =
        if let Ok((input, (rel_type, quantifier, is_optional))) =
            variable_length_relationship(input)
        {
            println!(
                "Parsed as variable length relationship: {:?}, {:?}, optional: {}",
                rel_type, quantifier, is_optional
            );
            (input, (Some(rel_type), Some(quantifier), is_optional))
        } else {
            // Fall back to regular relationship type
            let (input, rel_type) = opt(relationship_type)(input)?;
            println!(
                "After parsing rel_type: {:?}, remaining input: {}",
                rel_type, input
            );
            (input, (rel_type, None, false))
        };

    let (input, _) = multispace0(input)?;
    let (input, properties) = opt(property_map)(input)?;
    println!(
        "After parsing properties: {:?}, remaining input: {}",
        properties, input
    );
    let (input, _) = char(']')(input)?;
    println!("After parsing ']': {}", input);
    let (input, length) = opt(length_range)(input)?;
    println!(
        "After parsing length: {:?}, remaining input: {}",
        length, input
    );

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

pub fn pattern_element_sequence(
    input: &str,
    allow_qpp: bool,
) -> IResult<&str, Vec<PatternElement>> {
    println!(
        "[pattern_element_sequence] >>> ENTER: input='{}', allow_qpp={}",
        input, allow_qpp
    );
    let mut elements = Vec::new();
    let mut current_input = input;
    let mut loop_count = 0;
    const MAX_LOOPS: usize = 100; // Prevent infinite loops

    loop {
        loop_count += 1;
        if loop_count > MAX_LOOPS {
            println!("[pattern_element_sequence] MAX_LOOPS exceeded, breaking");
            break;
        }

        println!(
            "[pattern_element_sequence] LOOP {}: input='{}'",
            loop_count, current_input
        );

        // Check if we've reached a clause boundary
        let trimmed_input = current_input.trim_start();
        if trimmed_input.is_empty()
            || trimmed_input.starts_with("WHERE")
            || trimmed_input.starts_with("RETURN")
            || trimmed_input.starts_with("ON MATCH")
            || trimmed_input.starts_with("ON CREATE")
            || trimmed_input.starts_with("MATCH")
            || trimmed_input.starts_with("WITH")
            || trimmed_input.starts_with("UNWIND")
            || trimmed_input.starts_with("CALL")
            || trimmed_input.starts_with("CREATE")
            || trimmed_input.starts_with("DELETE")
            || trimmed_input.starts_with("DETACH DELETE")
            || trimmed_input.starts_with("REMOVE")
            || trimmed_input.starts_with("SET")
            || trimmed_input.starts_with("MERGE")
            || trimmed_input.starts_with("LIMIT")
        {
            println!(
                "[pattern_element_sequence] Stopping at clause boundary: '{}'",
                current_input
            );
            break;
        }

        // QPP detection logic
        if allow_qpp && current_input.trim_start().starts_with('(') {
            // Find the matching closing parenthesis
            let mut depth = 1;
            let mut idx = 0;
            let trimmed_input = current_input.trim_start();
            for (i, c) in trimmed_input.char_indices().skip(1) {
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
                let after_paren = &trimmed_input[idx + 1..];
                let after_paren_trim = after_paren.trim_start();
                if after_paren_trim.starts_with('{')
                    || after_paren_trim.starts_with('+')
                    || after_paren_trim.starts_with('*')
                {
                    println!(
                        "[pattern_element_sequence] Detected QPP at input='{}'",
                        current_input
                    );
                    // Calculate the actual index in the original input
                    let whitespace_len = current_input.len() - trimmed_input.len();
                    let _actual_idx = whitespace_len + idx;
                    let qpp_input = &current_input[whitespace_len..];

                    match quantified_path_pattern(qpp_input) {
                        Ok((after, pattern)) => {
                            println!(
                                "[pattern_element_sequence] Parsed QPP: {:?}, after='{}'",
                                pattern, after
                            );
                            elements.push(pattern);
                            // Calculate the remaining input after the QPP
                            let after_qpp =
                                &current_input[whitespace_len + qpp_input.len() - after.len()..];
                            current_input = after_qpp.trim_start();
                            continue;
                        }
                        Err(e) => {
                            println!(
                                "[pattern_element_sequence] quantified_path_pattern failed: {:?}",
                                e
                            );
                            // If QPP parsing fails, fall back to regular parsing
                            // Don't break, just continue with normal parsing
                        }
                    }
                }
            }
        }
        // Try to parse a node first
        let input_before_parsing = current_input;
        match node_pattern(current_input) {
            Ok((rest, node)) => {
                println!(
                    "[pattern_element_sequence] Parsed node: {:?}, rest='{}'",
                    node, rest
                );
                elements.push(PatternElement::Node(node));
                current_input = rest;
            }
            Err(e) => {
                println!("[pattern_element_sequence] node_pattern failed: {:?}", e);
                // If we can't parse a node, try to parse a relationship
                match relationship_pattern(current_input) {
                    Ok((rest, rel)) => {
                        println!(
                            "[pattern_element_sequence] Parsed relationship: {:?}, rest='{}'",
                            rel, rest
                        );
                        elements.push(PatternElement::Relationship(rel));
                        current_input = rest;
                    }
                    Err(e) => {
                        println!(
                            "[pattern_element_sequence] relationship_pattern failed: {:?}",
                            e
                        );
                        break;
                    }
                }
            }
        }

        // Check if we made progress
        if current_input == input_before_parsing {
            println!("[pattern_element_sequence] No progress made, breaking");
            break;
        }
    }

    println!(
        "[pattern_element_sequence] <<< EXIT: elements={:?}, input='{}'",
        elements, current_input
    );
    Ok((current_input, elements))
}

pub fn match_element(input: &str) -> IResult<&str, MatchElement> {
    let (input, path_var) = opt(terminated(
        identifier,
        tuple((multispace0, char('='), multispace0)),
    ))(input)?;
    let (input, pattern) = pattern_element_sequence(input, true)?;
    println!(
        "[match_element] After pattern_element_sequence: pattern={:?}, input='{}'",
        pattern, input
    );
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

pub fn path_variable(input: &str) -> IResult<&str, String> {
    println!("[path_variable] ENTER: input='{}'", input);
    let (input, var) = terminated(identifier, tuple((multispace0, char('='), multispace0)))(input)?;
    println!("[path_variable] EXIT: var='{}', input='{}'", var, input);
    Ok((input, var.to_string()))
}

pub fn quantified_path_pattern(input: &str) -> IResult<&str, PatternElement> {
    println!("[quantified_path_pattern] >>> ENTER: input='{}'", input);
    let (input, _) = char('(')(input)?;
    println!(
        "[quantified_path_pattern] After opening parenthesis: input='{}'",
        input
    );

    // Parse optional path variable using the new parser
    let (input, path_var) = opt(path_variable)(input)?;
    println!(
        "[quantified_path_pattern] After path variable: {:?}, input='{}'",
        path_var, input
    );

    // Find the matching closing parenthesis for the QPP
    let mut depth = 1;
    let mut idx = 0;
    for (i, c) in input.char_indices() {
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
    if depth != 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }
    let inner_pattern_str = &input[..idx];
    let after_paren = &input[idx + 1..];
    println!(
        "[quantified_path_pattern] Extracted inner pattern substring: '{}'",
        inner_pattern_str
    );
    println!(
        "[quantified_path_pattern] After closing parenthesis: '{}'",
        after_paren
    );

    // Parse the inner pattern using the existing pattern_element_sequence function
    let (remaining_inner, mut inner_pattern) = pattern_element_sequence(inner_pattern_str, false)?;
    println!(
        "[quantified_path_pattern] Parsed inner pattern: {:?}",
        inner_pattern
    );
    println!(
        "[quantified_path_pattern] Remaining after inner pattern: '{}'",
        remaining_inner
    );

    // Strip quantifiers from relationships inside the QPP
    for element in &mut inner_pattern {
        if let PatternElement::Relationship(rel) = element {
            match rel {
                RelationshipPattern::Regular(details)
                | RelationshipPattern::OptionalRelationship(details) => {
                    details.quantifier = None;
                }
            }
        }
    }

    // Parse optional WHERE clause using the where_clause parser, from remaining_inner
    let (where_input, where_clause) = if let Ok((rest, clause)) = where_clause(remaining_inner) {
        println!(
            "[quantified_path_pattern] Successfully parsed WHERE clause: {:?}",
            clause
        );
        (rest, Some(clause))
    } else {
        (remaining_inner, None)
    };
    println!(
        "[quantified_path_pattern] After WHERE clause parsing: where_input='{}'",
        where_input
    );

    // Now, where_input should be empty, and after_paren is the input after the closing parenthesis
    let mut input = after_paren;
    // Skip any whitespace between ) and quantifier
    let (rest, _) = multispace0(input)?;
    input = rest;

    // Handle different quantifier formats: {min,max}, {min,}, {+}, {*}, +, *
    let (input, min, max) = if let Ok((rest, _)) = char::<&str, nom::error::Error<&str>>('+')(input)
    {
        // + means {1,} (one or more)
        (rest, 1, None)
    } else if let Ok((rest, _)) = char::<&str, nom::error::Error<&str>>('*')(input) {
        // * means {0,} (zero or more)
        (rest, 0, None)
    } else {
        // Try to parse {min,max} format
        let (input, _) = char('{')(input)?;
        println!(
            "[quantified_path_pattern] After opening brace: input='{}'",
            input
        );

        // Parse min value
        let (rest, min_str) = digit1::<&str, nom::error::Error<&str>>(input)?;
        let min = min_str.parse::<u32>().unwrap();

        // Check for separator
        let (rest, _) = alt((tag(".."), tag(",")))(rest)?;

        // Check if there's a max value
        if let Ok((rest, max_str)) = digit1::<&str, nom::error::Error<&str>>(rest) {
            let max = max_str.parse::<u32>().unwrap();
            (rest, min, Some(max))
        } else {
            // No max value means unlimited
            (rest, min, None)
        }
    };

    // If we parsed a brace format, we need to close it
    let (input, _) = if input.starts_with('}') {
        char('}')(input)?
    } else {
        (input, ' ')
    };

    println!(
        "[quantified_path_pattern] After quantifier parsing: input='{}'",
        input
    );

    // Create the quantified pattern
    let quantified_pattern = QuantifiedPathPattern {
        pattern: inner_pattern,
        min: Some(min),
        max,
        where_clause,
        path_variable: path_var,
    };
    println!(
        "[quantified_path_pattern] <<< EXIT: {:?}",
        quantified_pattern
    );
    Ok((
        input,
        PatternElement::QuantifiedPathPattern(quantified_pattern),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PropertyValue;
    use crate::WhereCondition;

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
    fn test_node_pattern_with_properties() {
        let input = "(a:Station { name: 'Denmark Hill' })";
        let (_, node) = node_pattern(input).unwrap();
        assert_eq!(node.variable, Some("a".to_string()));
        assert_eq!(node.label, Some("Station".to_string()));
        assert!(node.properties.is_some());
        if let Some(props) = node.properties {
            assert_eq!(props.len(), 1);
            assert_eq!(props[0].key, "name");
            assert_eq!(
                props[0].value,
                PropertyValue::String("Denmark Hill".to_string())
            );
        }
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
                if let WhereCondition::Comparison {
                    left,
                    operator,
                    right,
                } = &where_clause.conditions[0]
                {
                    assert_eq!(
                        left,
                        &crate::parser::ast::PropertyValue::Identifier("a.name".to_string())
                    );
                    assert_eq!(operator, "=");
                    assert_eq!(
                        right,
                        &crate::parser::ast::PropertyValue::String("Alice".to_string())
                    );
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

    #[test]
    fn test_quantified_path_pattern_with_anonymous_nodes() {
        let input = "((:Stop)-[:NEXT]->(:Stop)){1,3}";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.pattern.len(), 1);
        if let PatternElement::QuantifiedPathPattern(qpp) = &element.pattern[0] {
            assert_eq!(qpp.min, Some(1));
            assert_eq!(qpp.max, Some(3));
            assert_eq!(qpp.pattern.len(), 3); // Node, Relationship, Node
                                              // Verify that the nodes inside QPP are anonymous
            if let PatternElement::Node(node) = &qpp.pattern[0] {
                assert_eq!(node.variable, None);
                assert_eq!(node.label, Some("Stop".to_string()));
            } else {
                panic!("Expected node inside QuantifiedPathPattern");
            }
            if let PatternElement::Node(node) = &qpp.pattern[2] {
                assert_eq!(node.variable, None);
                assert_eq!(node.label, Some("Stop".to_string()));
            } else {
                panic!("Expected node inside QuantifiedPathPattern");
            }
        } else {
            panic!("Expected quantified path pattern");
        }
    }

    #[test]
    fn test_complex_pattern_with_qpp() {
        let input = "(a:Station { name: 'Denmark Hill' })<-[:CALLS_AT]-(d:Stop) ((:Stop)-[:NEXT]->(:Stop)){1,3} (final_stop:Stop)-[:CALLS_AT]->(:Station { name: 'Clapham Junction' })";
        let (_, pattern) = pattern_element_sequence(input, true).unwrap();
        println!("Parsed pattern: {:?}", pattern);
        // Should have multiple elements: Node, Relationship, Node, QPP, Node, Relationship, Node
        assert!(pattern.len() >= 5);

        // Check that we have a QPP in the pattern
        let has_qpp = pattern
            .iter()
            .any(|e| matches!(e, PatternElement::QuantifiedPathPattern(_)));
        assert!(has_qpp, "Pattern should contain a QuantifiedPathPattern");
    }

    #[test]
    fn test_property_map_debug() {
        use crate::parser::components::property_map;
        let input = "{ name: 'Denmark Hill' }";
        let result = property_map(input);
        println!("property_map result: {:?}", result);
        assert!(result.is_ok());
        let (rest, props) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, "name");
        assert_eq!(
            props[0].value,
            PropertyValue::String("Denmark Hill".to_string())
        );
    }

    #[test]
    fn test_property_debug() {
        use crate::parser::components::property;
        let input = "name: 'Denmark Hill'";
        let result = property(input);
        println!("property result: {:?}", result);
        assert!(result.is_ok());
        let (rest, prop) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(prop.key, "name");
        assert_eq!(
            prop.value,
            PropertyValue::String("Denmark Hill".to_string())
        );
    }

    #[test]
    fn test_property_map_simple() {
        use crate::parser::components::property_map;
        let input = "{name: 'Denmark Hill'}";
        let result = property_map(input);
        println!("property_map simple result: {:?}", result);
        assert!(result.is_ok());
        let (rest, props) = result.unwrap();
        assert_eq!(rest, "");
        assert_eq!(props.len(), 1);
        assert_eq!(props[0].key, "name");
        assert_eq!(
            props[0].value,
            PropertyValue::String("Denmark Hill".to_string())
        );
    }

    #[test]
    fn test_qpp_with_plus_quantifier() {
        let input = "((a)-[:LINK]-(b:Station) WHERE point.distance(a.location, ndl.location) > point.distance(b.location, ndl.location))+";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.pattern.len(), 1);
        if let PatternElement::QuantifiedPathPattern(qpp) = &element.pattern[0] {
            assert_eq!(qpp.min, Some(1));
            assert_eq!(qpp.max, None);
            assert!(qpp.where_clause.is_some());
            assert_eq!(qpp.pattern.len(), 3); // Node, Relationship, Node
        } else {
            panic!("Expected quantified path pattern");
        }
    }

    #[test]
    fn test_qpp_with_where_clause_multiline() {
        let input = "((a)-[:LINK]-(b:Station) WHERE point.distance(a.location, ndl.location) > point.distance(b.location, ndl.location))+";
        let (_, element) = match_element(input).unwrap();
        assert_eq!(element.pattern.len(), 1);
        if let PatternElement::QuantifiedPathPattern(qpp) = &element.pattern[0] {
            assert_eq!(qpp.min, Some(1));
            assert_eq!(qpp.max, None);
            assert!(qpp.where_clause.is_some());
            assert_eq!(qpp.pattern.len(), 3); // Node, Relationship, Node
        } else {
            panic!("Expected quantified path pattern");
        }
    }
}
