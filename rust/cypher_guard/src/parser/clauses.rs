use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::parser::ast::*;
use crate::parser::patterns::*;
use crate::parser::utils::identifier;

#[derive(Debug, Clone)]
pub enum Clause {
    Match(MatchClause),
    OptionalMatch(MatchClause),
    Return(ReturnClause),
    Query(Query),
}

// Parses a list of match elements, e.g. (a)-[:KNOWS]->(b), (b)-[:LIVES_IN]->(c)
pub fn match_element_list(input: &str) -> IResult<&str, Vec<MatchElement>> {
    separated_list1(tuple((multispace0, char(','), multispace0)), match_element)(input)
}

// Parses the MATCH clause (e.g. MATCH (a)-[:KNOWS]->(b))
pub fn match_clause(input: &str) -> IResult<&str, MatchClause> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("MATCH")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, elements) = match_element_list(input)?;
    Ok((input, MatchClause { elements }))
}

// Parses the RETURN clause (e.g. RETURN a, b)
pub fn return_clause(input: &str) -> IResult<&str, ReturnClause> {
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

// Parses an entire Cypher query consisting of a MATCH and optional RETURN clause
pub fn parse_query(input: &str) -> IResult<&str, Query> {
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

// Update the parser to handle OPTIONAL MATCH
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
