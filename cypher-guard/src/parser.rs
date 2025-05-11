use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum PatternElement {
    Node(NodePattern),
    Relationship(RelationshipPattern),
}

#[derive(Debug, PartialEq)]
pub struct NodePattern {
    pub variable: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct RelationshipPattern {
    pub variable: Option<String>,
    pub direction: Direction,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Undirected,
}

#[derive(Debug, PartialEq)]
pub struct MatchClause {
    pub elements: Vec<PatternElement>,
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

fn node_pattern(input: &str) -> IResult<&str, NodePattern> {
    let (input, _) = multispace0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, var) = opt_identifier(input)?;
    let (input, label) = opt(preceded(
        tuple((multispace0, char(':'))),
        identifier,
    ))(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, NodePattern { variable: var, label: label.map(|s| s.to_string()) }))
}

fn opt_identifier(input: &str) -> IResult<&str, Option<String>> {
    let (input, _) = multispace0(input)?;
    match identifier(input) {
        Ok((input, id)) => Ok((input, Some(id.to_string()))),
        Err(_) => Ok((input, None)),
    }
}

fn relationship_pattern(input: &str) -> IResult<&str, RelationshipPattern> {
    let (input, left) = opt(preceded(multispace0, alt((tag("<-"), tag("-")))))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, rel) = opt(delimited(
        char('['),
        opt_identifier,
        char(']'),
    ))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, right) = opt(alt((tag("->"), tag("-"))))(input)?;

    let direction = match (left, right) {
        (Some("<-"), _) => Direction::Left,
        (_, Some("->")) => Direction::Right,
        _ => Direction::Undirected,
    };
    Ok((input, RelationshipPattern { variable: rel.flatten(), direction }))
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

fn pattern_element_list(input: &str) -> IResult<&str, Vec<PatternElement>> {
    separated_list1(
        tuple((multispace0, char(','), multispace0)),
        pattern_element_sequence,
    )(input)
    .map(|(i, v)| (i, v.into_iter().flatten().collect()))
}

pub fn match_clause(input: &str) -> IResult<&str, MatchClause> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("MATCH")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, elements) = pattern_element_list(input)?;
    Ok((input, MatchClause { elements }))
}

fn return_clause(input: &str) -> IResult<&str, ReturnClause> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("RETURN")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = separated_list1(
        tuple((multispace0, char(','), multispace0)),
        identifier,
    )(input)?;
    Ok((input, ReturnClause { items: items.into_iter().map(|s| s.to_string()).collect() }))
}

pub fn query(input: &str) -> IResult<&str, Query> {
    let (input, match_clause) = match_clause(input)?;
    let (input, return_clause) = opt(return_clause)(input)?;
    Ok((input, Query { match_clause, return_clause }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_clause_single() {
        let input = "MATCH (n)";
        let res = match_clause(input);
        assert_eq!(
            res,
            Ok((
                "",
                MatchClause {
                    elements: vec![PatternElement::Node(NodePattern {
                        variable: Some("n".to_string())
                    })]
                }
            ))
        );
    }

    #[test]
    fn test_match_clause_multiple() {
        let input = "MATCH (n), (m)";
        let res = match_clause(input);
        assert_eq!(
            res,
            Ok((
                "",
                MatchClause {
                    elements: vec![
                        PatternElement::Node(NodePattern {
                            variable: Some("n".to_string())
                        }),
                        PatternElement::Node(NodePattern {
                            variable: Some("m".to_string())
                        })
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_match_clause_with_relationship() {
        let input = "MATCH (n)-[r]->(m)";
        let res = match_clause(input);
        assert_eq!(
            res,
            Ok((
                "",
                MatchClause {
                    elements: vec![
                        PatternElement::Node(NodePattern {
                            variable: Some("n".to_string())
                        }),
                        PatternElement::Relationship(RelationshipPattern {
                            variable: Some("r".to_string()),
                            direction: Direction::Right
                        }),
                        PatternElement::Node(NodePattern {
                            variable: Some("m".to_string())
                        })
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_match_clause_with_undirected_relationship() {
        let input = "MATCH (a)--(b)";
        let res = match_clause(input);
        assert_eq!(
            res,
            Ok((
                "",
                MatchClause {
                    elements: vec![
                        PatternElement::Node(NodePattern {
                            variable: Some("a".to_string())
                        }),
                        PatternElement::Relationship(RelationshipPattern {
                            variable: None,
                            direction: Direction::Undirected
                        }),
                        PatternElement::Node(NodePattern {
                            variable: Some("b".to_string())
                        })
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_match_clause_with_left_relationship() {
        let input = "MATCH (x)<-[rel]-(y)";
        let res = match_clause(input);
        assert_eq!(
            res,
            Ok((
                "",
                MatchClause {
                    elements: vec![
                        PatternElement::Node(NodePattern {
                            variable: Some("x".to_string())
                        }),
                        PatternElement::Relationship(RelationshipPattern {
                            variable: Some("rel".to_string()),
                            direction: Direction::Left
                        }),
                        PatternElement::Node(NodePattern {
                            variable: Some("y".to_string())
                        })
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_match_clause_with_node_label() {
        let input = "MATCH (n:Person)";
        let res = match_clause(input);
        assert_eq!(
            res,
            Ok((
                "",
                MatchClause {
                    elements: vec![PatternElement::Node(NodePattern {
                        variable: Some("n".to_string()),
                        label: Some("Person".to_string()),
                    })]
                }
            ))
        );
    }

    #[test]
    fn test_match_clause_with_multiple_node_labels() {
        let input = "MATCH (n:Person), (m:Animal)";
        let res = match_clause(input);
        assert_eq!(
            res,
            Ok((
                "",
                MatchClause {
                    elements: vec![
                        PatternElement::Node(NodePattern {
                            variable: Some("n".to_string()),
                            label: Some("Person".to_string()),
                        }),
                        PatternElement::Node(NodePattern {
                            variable: Some("m".to_string()),
                            label: Some("Animal".to_string()),
                        })
                    ]
                }
            ))
        );
    }

    #[test]
    fn test_query_with_return() {
        let input = "MATCH (n:Person) RETURN n";
        let res = query(input);
        assert_eq!(
            res,
            Ok((
                "",
                Query {
                    match_clause: MatchClause {
                        elements: vec![PatternElement::Node(NodePattern {
                            variable: Some("n".to_string()),
                            label: Some("Person".to_string()),
                        })]
                    },
                    return_clause: Some(ReturnClause { items: vec!["n".to_string()] }),
                }
            ))
        );
    }

    #[test]
    fn test_query_with_multiple_return_items() {
        let input = "MATCH (n:Person), (m:Animal) RETURN n, m";
        let res = query(input);
        assert_eq!(
            res,
            Ok((
                "",
                Query {
                    match_clause: MatchClause {
                        elements: vec![
                            PatternElement::Node(NodePattern {
                                variable: Some("n".to_string()),
                                label: Some("Person".to_string()),
                            }),
                            PatternElement::Node(NodePattern {
                                variable: Some("m".to_string()),
                                label: Some("Animal".to_string()),
                            })
                        ]
                    },
                    return_clause: Some(ReturnClause { items: vec!["n".to_string(), "m".to_string()] }),
                }
            ))
        );
    }
} 