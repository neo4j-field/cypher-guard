use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list1,
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
    Return(ReturnClause),
    Query(Query),
}

// Parses a list of match elements separated by commas
pub fn match_element_list(input: &str) -> IResult<&str, Vec<MatchElement>> {
    println!("Parsing match element list: {}", input); // Debug
    let (input, first) = match_element(input)?;
    println!("First match element: {:?}", first); // Debug
    let (input, rest) = opt(preceded(
        tuple((multispace0, char(','), multispace0)),
        match_element,
    ))(input)?;
    println!("Rest of match elements: {:?}", rest); // Debug
    let mut elements = vec![first];
    if let Some(element) = rest {
        elements.push(element);
    }
    println!("Final match elements: {:?}", elements); // Debug
    Ok((input, elements))
}

// Parses the MATCH clause (e.g. MATCH (a)-[:KNOWS]->(b))
pub fn match_clause(input: &str) -> IResult<&str, MatchClause> {
    println!("Parsing match clause: {}", input); // Debug
    match multispace0(input) {
        Ok((input, _)) => {
            println!("After initial whitespace: {}", input); // Debug
            match tag("MATCH")(input) {
                Ok((input, _)) => {
                    println!("After MATCH tag: {}", input); // Debug
                    match multispace1(input) {
                        Ok((input, _)) => {
                            println!("After MATCH whitespace: {}", input); // Debug
                            match match_element_list(input) {
                                Ok((input, elements)) => {
                                    println!("Match clause elements: {:?}", elements); // Debug
                                    Ok((input, MatchClause { elements }))
                                }
                                Err(e) => {
                                    println!("Match element list parse error: {:?}", e); // Debug
                                    Err(e)
                                }
                            }
                        }
                        Err(e) => {
                            println!("Whitespace parse error: {:?}", e); // Debug
                            Err(e)
                        }
                    }
                }
                Err(e) => {
                    println!("MATCH tag parse error: {:?}", e); // Debug
                    Err(e)
                }
            }
        }
        Err(e) => {
            println!("Initial whitespace parse error: {:?}", e); // Debug
            Err(e)
        }
    }
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

// Parses an entire Cypher query consisting of a MATCH and optional RETURN clause
pub fn parse_query(input: &str) -> IResult<&str, Query> {
    println!("Parsing query: {}", input); // Debug
    match match_clause(input) {
        Ok((input, match_clause)) => {
            println!("Match clause parsed: {:?}", match_clause); // Debug
            match opt(return_clause)(input) {
                Ok((input, return_clause)) => {
                    println!("Return clause parsed: {:?}", return_clause); // Debug
                    Ok((
                        input,
                        Query {
                            match_clause,
                            return_clause,
                        },
                    ))
                }
                Err(e) => {
                    println!("Return clause parse error: {:?}", e); // Debug
                    Err(e)
                }
            }
        }
        Err(e) => {
            println!("Match clause parse error: {:?}", e); // Debug
            Err(e)
        }
    }
}

// Update the parser to handle OPTIONAL MATCH
#[allow(dead_code)]
pub fn clause(input: &str) -> IResult<&str, Clause> {
    alt((
        map(
            tuple((tag("OPTIONAL MATCH"), multispace1, match_element_list)),
            |(_, _, elements)| Clause::OptionalMatch(MatchClause { elements }),
        ),
        map(
            tuple((tag("MATCH"), multispace1, match_element_list)),
            |(_, _, elements)| Clause::Match(MatchClause { elements }),
        ),
        // ... existing clause parsers ...
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional_match_clause() {
        let input = "OPTIONAL MATCH (a)-[:KNOWS]->(b)";
        let result = clause(input);
        assert!(result.is_ok());
        let (_, clause) = result.unwrap();
        match clause {
            Clause::OptionalMatch(mc) => {
                assert_eq!(mc.elements.len(), 1);
            }
            _ => panic!("Expected OptionalMatch"),
        }
    }

    #[test]
    fn test_regular_match_clause() {
        let input = "MATCH (a)-[:KNOWS]->(b)";
        let result = clause(input);
        assert!(result.is_ok());
        let (_, clause) = result.unwrap();
        match clause {
            Clause::Match(mc) => {
                assert_eq!(mc.elements.len(), 1);
            }
            _ => panic!("Expected Match"),
        }
    }
}
