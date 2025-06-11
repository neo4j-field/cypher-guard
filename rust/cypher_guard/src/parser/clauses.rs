use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map, opt, recognize},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::parser::ast::*;
use crate::parser::components::*;
use crate::parser::patterns::*;
use crate::parser::utils::{identifier, string_literal};

#[derive(Debug, Clone)]
pub enum Clause {
    Match(MatchClause),
    OptionalMatch(MatchClause),
    Merge(MergeClause),
    Create(CreateClause),
    Return(ReturnClause),
    With(WithClause),
    Query(Query),
}

// Parses a list of match elements separated by commas
pub fn match_element_list(input: &str) -> IResult<&str, Vec<MatchElement>> {
    println!("Parsing match element list: {}", input);
    let (input, first) = match_element(input)?;
    println!("First match element: {:?}", first);
    let (input, rest) = opt(preceded(
        tuple((multispace0, char(','), multispace0)),
        match_element,
    ))(input)?;
    println!("Rest of match elements: {:?}", rest);
    let mut elements = vec![first];
    if let Some(rest) = rest {
        elements.push(rest);
    }
    println!("Final match elements: {:?}", elements);
    Ok((input, elements))
}

// Parses the MATCH clause (e.g. MATCH (a)-[:KNOWS]->(b))
pub fn match_clause(input: &str) -> IResult<&str, MatchClause> {
    println!("Parsing match clause: {}", input);
    let (input, _) = multispace0(input)?;
    println!("After initial whitespace: {}", input);
    let (input, is_optional) = opt(tuple((tag("OPTIONAL"), multispace1)))(input)?;
    let (input, _) = tag("MATCH")(input)?;
    println!("After MATCH tag: {}", input);
    let (input, _) = multispace1(input)?;
    println!("After MATCH whitespace: {}", input);
    let (input, elements) = match_element_list(input)?;
    println!("Match clause elements: {:?}", elements);
    Ok((
        input,
        MatchClause {
            elements,
            is_optional: is_optional.is_some(),
        },
    ))
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
    Ok((input, ReturnClause { items }))
}

// Parses a numeric literal
fn numeric_literal(input: &str) -> IResult<&str, String> {
    let (input, num) = recognize(digit1)(input)?;
    Ok((input, num.to_string()))
}

// Parses a path property access
fn path_property(input: &str) -> IResult<&str, (String, String)> {
    let (input, path_var) = map(identifier, |s| s.to_string())(input)?;
    let (input, _) = char('.')(input)?;
    let (input, property) = map(identifier, |s| s.to_string())(input)?;
    Ok((input, (path_var, property)))
}

// Parses a WHERE condition (e.g., a.age > 30, point.distance(a.location, b.location) > 10)
fn where_condition(input: &str) -> IResult<&str, WhereCondition> {
    alt((
        map(
            tuple((
                function_call,
                multispace0,
                alt((
                    tag(">"),
                    tag("<"),
                    tag(">="),
                    tag("<="),
                    tag("="),
                    tag("<>"),
                )),
                multispace0,
                alt((numeric_literal, map(identifier, |s| s.to_string()))),
            )),
            |((function, args), _, _operator, _, _right)| WhereCondition::FunctionCall {
                function,
                arguments: args,
            },
        ),
        // Path property condition
        map(
            tuple((
                path_property,
                multispace0,
                alt((
                    tag(">"),
                    tag("<"),
                    tag(">="),
                    tag("<="),
                    tag("="),
                    tag("<>"),
                )),
                multispace0,
                alt((numeric_literal, map(identifier, |s| s.to_string()))),
            )),
            |((path_var, property), _, _operator, _, _right)| WhereCondition::PathProperty {
                path_var,
                property,
            },
        ),
        // Regular property comparison
        map(
            tuple((
                map(identifier, |s| s.to_string()),
                multispace0,
                alt((
                    tag(">"),
                    tag("<"),
                    tag(">="),
                    tag("<="),
                    tag("="),
                    tag("<>"),
                )),
                multispace0,
                alt((numeric_literal, map(identifier, |s| s.to_string()))),
            )),
            |(left, _, operator, _, right)| WhereCondition::Comparison {
                left,
                operator: operator.to_string(),
                right,
            },
        ),
    ))(input)
}

// Parses the WHERE clause (e.g. WHERE a.age > 30)
pub fn where_clause(input: &str) -> IResult<&str, WhereClause> {
    println!("Parsing where clause: {}", input);
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("WHERE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, conditions) = separated_list1(
        tuple((multispace0, tag("AND"), multispace0)),
        where_condition,
    )(input)?;
    println!("Where conditions: {:?}", conditions);
    Ok((input, WhereClause { conditions }))
}

// Parses a SET clause (e.g. SET a.name = 'Alice')
fn set_clause(input: &str) -> IResult<&str, SetClause> {
    println!("DEBUG: Parsing set clause: {}", input);
    let (input, _) = multispace0(input)?;
    let (input, variable) = identifier(input)?;
    println!("DEBUG: Parsed variable: {}", variable);
    let (input, _) = char('.')(input)?;
    let (input, property) = identifier(input)?;
    println!("DEBUG: Parsed property: {}", property);
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = property_value(input)?;
    println!("DEBUG: Parsed value: {:?}", value);
    Ok((
        input,
        SetClause {
            variable: variable.to_string(),
            property: property.to_string(),
            value,
        },
    ))
}

// Parses ON CREATE clause (e.g. ON CREATE SET a.name = 'Alice')
fn on_create_clause(input: &str) -> IResult<&str, OnCreateClause> {
    println!("DEBUG: Parsing ON CREATE clause: {}", input);
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("ON CREATE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("SET")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, set_clauses) =
        separated_list1(tuple((multispace0, char(','), multispace0)), set_clause)(input)?;
    println!("DEBUG: Parsed ON CREATE set clauses: {:?}", set_clauses);
    Ok((input, OnCreateClause { set_clauses }))
}

// Parses ON MATCH clause (e.g. ON MATCH SET a.name = 'Alice')
fn on_match_clause(input: &str) -> IResult<&str, OnMatchClause> {
    println!("DEBUG: Parsing ON MATCH clause: {}", input);
    let (input, _) = multispace0(input)?;
    println!("DEBUG: After initial whitespace: {}", input);
    let (input, _) = tag("ON MATCH")(input)?;
    println!("DEBUG: After ON MATCH tag: {}", input);
    let (input, _) = multispace1(input)?;
    println!("DEBUG: After ON MATCH whitespace: {}", input);
    let (input, _) = tag("SET")(input)?;
    println!("DEBUG: After SET tag: {}", input);
    let (input, _) = multispace1(input)?;
    println!("DEBUG: After SET whitespace: {}", input);
    let (input, set_clauses) = match separated_list1(tuple((multispace0, char(','), multispace0)), set_clause)(input) {
        Ok(res) => {
            println!("DEBUG: Successfully parsed set_clauses");
            res
        },
        Err(e) => {
            println!("DEBUG: Failed to parse set_clauses: {:?}", e);
            return Err(e);
        }
    };
    println!("DEBUG: Parsed ON MATCH set clauses: {:?}", set_clauses);
    Ok((input, OnMatchClause { set_clauses }))
}

// Parses the MERGE clause (e.g. MERGE (a:Person {name: 'Alice'}) ON CREATE SET a.created = timestamp())
pub fn merge_clause(input: &str) -> IResult<&str, MergeClause> {
    println!("DEBUG: Parsing merge clause: {}", input);
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("MERGE")(input)?;
    let (input, _) = multispace1(input)?;
    println!("DEBUG: After MERGE tag, remaining input: {}", input);
    let (mut input, elements) = match_element_list(input)?;
    println!("DEBUG: After parsing elements, remaining input: {}", input);
    let mut found_on_create = None;
    let mut found_on_match = None;
    
    // Try up to two times (since there can be at most one ON CREATE and one ON MATCH)
    for i in 0..2 {
        println!("DEBUG: Attempt {} to parse ON clauses", i + 1);
        // Consume any whitespace before trying to parse ON clauses
        let (rest, _) = multispace0(input)?;
        input = rest;
        println!("DEBUG: After consuming whitespace, remaining input: {}", input);
        
        if found_on_create.is_none() {
            println!("DEBUG: Trying to parse ON CREATE clause");
            match on_create_clause(input) {
                Ok((rest, clause)) => {
                    println!("DEBUG: Successfully parsed ON CREATE clause");
                    found_on_create = Some(clause);
                    input = rest;
                    continue;
                },
                Err(e) => {
                    println!("DEBUG: Failed to parse ON CREATE clause: {:?}", e);
                }
            }
        }
        if found_on_match.is_none() {
            println!("DEBUG: Trying to parse ON MATCH clause");
            match on_match_clause(input) {
                Ok((rest, clause)) => {
                    println!("DEBUG: Successfully parsed ON MATCH clause");
                    found_on_match = Some(clause);
                    input = rest;
                    continue;
                },
                Err(e) => {
                    println!("DEBUG: Failed to parse ON MATCH clause: {:?}", e);
                }
            }
        }
        println!("DEBUG: No more ON clauses found");
        break;
    }
    println!("DEBUG: Final merge clause state - on_create: {:?}, on_match: {:?}", found_on_create, found_on_match);
    Ok((
        input,
        MergeClause {
            elements,
            on_create: found_on_create,
            on_match: found_on_match,
        },
    ))
}

// Parses the CREATE clause (e.g. CREATE (a:Person {name: 'Alice'})-[r:KNOWS]->(b:Person {name: 'Bob'}))
pub fn create_clause(input: &str) -> IResult<&str, CreateClause> {
    println!("Parsing create clause: {}", input);
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("CREATE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, elements) = match_element_list(input)?;
    Ok((input, CreateClause { elements }))
}

// Parses a WITH item (e.g. a, a.name AS name, count(*) AS count)
fn with_item(input: &str) -> IResult<&str, WithItem> {
    println!("DEBUG: Parsing with_item from input: '{}'", input);
    let (input, expr) = alt((
        map(tag("*"), |_| {
            println!("DEBUG: Matched wildcard");
            WithExpression::Wildcard
        }),
        map(
            tuple((
                identifier,
                preceded(tag("."), identifier),
            )),
            |(var, prop)| {
                println!("DEBUG: Matched property access: {}.{}", var, prop);
                WithExpression::PropertyAccess {
                    variable: var.to_string(),
                    property: prop.to_string(),
                }
            },
        ),
        map(
            function_call,
            |(name, args)| {
                println!("DEBUG: Matched function call: {}({:?})", name, args);
                WithExpression::FunctionCall {
                    name,
                    args: args.into_iter().map(|arg| WithExpression::Identifier(arg)).collect(),
                }
            },
        ),
        map(identifier, |s| {
            println!("DEBUG: Matched identifier: {}", s);
            WithExpression::Identifier(s.to_string())
        }),
    ))(input)?;
    println!("DEBUG: After parsing expression, remaining input: '{}'", input);
    let (input, alias) = opt(preceded(
        tuple((multispace0, tag("AS"), multispace0)),
        identifier,
    ))(input)?;
    println!("DEBUG: After parsing alias, remaining input: '{}'", input);
    println!("DEBUG: Final with_item: expression={:?}, alias={:?}", expr, alias);
    Ok((input, WithItem { expression: expr, alias: alias.map(|s| s.to_string()) }))
}

// Parses the WITH clause (e.g. WITH a, count(*) AS count)
pub fn with_clause(input: &str) -> IResult<&str, WithClause> {
    println!("DEBUG: Parsing with_clause from input: '{}'", input);
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("WITH")(input)?;
    let (input, _) = multispace1(input)?;
    println!("DEBUG: After WITH tag, remaining input: '{}'", input);
    let (input, items) =
        separated_list1(tuple((multispace0, char(','), multispace0)), with_item)(input)?;
    println!("DEBUG: Parsed with items: {:?}", items);
    Ok((input, WithClause { items }))
}

// Parses a clause (MATCH, RETURN, etc.)
pub fn clause(input: &str) -> IResult<&str, Clause> {
    println!("Parsing clause: {}", input);
    let (input, _) = multispace0(input)?;
    alt((
        map(match_clause, Clause::Match),
        map(return_clause, Clause::Return),
        map(merge_clause, Clause::Merge),
        map(create_clause, Clause::Create),
        map(with_clause, Clause::With),
    ))(input)
}

// Parses a complete query (e.g. MATCH (a)-[:KNOWS]->(b) RETURN a, b)
pub fn parse_query(input: &str) -> IResult<&str, Query> {
    println!("Parsing query: {}", input);
    let (input, _) = multispace0(input)?;
    let (input, clauses) = many1(preceded(multispace0, clause))(input)?;
    println!("Parsed clauses: {:?}", clauses);
    let mut query = Query {
        match_clause: None,
        merge_clause: None,
        create_clause: None,
        with_clause: None,
        where_clause: None,
        return_clause: None,
    };
    for clause in clauses {
        match clause {
            Clause::Match(match_clause) => query.match_clause = Some(match_clause),
            Clause::Merge(merge_clause) => query.merge_clause = Some(merge_clause),
            Clause::Create(create_clause) => query.create_clause = Some(create_clause),
            Clause::With(with_clause) => query.with_clause = Some(with_clause),
            Clause::Return(return_clause) => query.return_clause = Some(return_clause),
            _ => (),
        }
    }
    Ok((input, query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional_match_clause() {
        let input = "OPTIONAL MATCH (a)-[:KNOWS]->(b)";
        let (_, clause) = match_clause(input).unwrap();
        assert!(clause.is_optional);
        assert_eq!(clause.elements.len(), 1);
    }

    #[test]
    fn test_regular_match_clause() {
        let input = "MATCH (a)-[:KNOWS]->(b)";
        let (_, clause) = match_clause(input).unwrap();
        assert!(!clause.is_optional);
        assert_eq!(clause.elements.len(), 1);
    }

    #[test]
    fn test_merge_clause() {
        let input = "MERGE (a:Person {name: 'Alice'})";
        let (_, clause) = merge_clause(input).unwrap();
        assert_eq!(clause.elements.len(), 1);
        assert!(clause.on_create.is_none());
        assert!(clause.on_match.is_none());
    }

    #[test]
    fn test_create_clause() {
        let input = "CREATE (a:Person {name: 'Alice'})-[r:KNOWS]->(b:Person {name: 'Bob'})";
        let (_, clause) = create_clause(input).unwrap();
        assert_eq!(clause.elements.len(), 1);
    }

    #[test]
    fn test_merge_with_on_match() {
        let input = "MERGE (a:Person {name: 'Alice'}) ON MATCH SET a.lastSeen = timestamp()";
        let (_, clause) = merge_clause(input).unwrap();
        assert_eq!(clause.elements.len(), 1);
        assert!(clause.on_create.is_none());
        assert!(clause.on_match.is_some());
    }

    #[test]
    fn test_create_with_relationship() {
        let input = "CREATE (a:Person {name: 'Alice'})-[r:KNOWS {since: 2020}]->(b:Person {name: 'Bob'})";
        let (_, clause) = create_clause(input).unwrap();
        assert_eq!(clause.elements.len(), 1);
        if let PatternElement::Relationship(rel) = &clause.elements[0].pattern[1] {
            assert_eq!(rel.direction(), Direction::Right);
            assert_eq!(rel.rel_type(), Some("KNOWS"));
            assert!(rel.properties().is_some());
        } else {
            panic!("Expected relationship");
        }
    }

    #[test]
    fn test_with_clause_simple() {
        let input = "WITH a, b";
        let (_, clause) = with_clause(input).unwrap();
        assert_eq!(clause.items.len(), 2);
    }

    #[test]
    fn test_with_clause_alias() {
        let input = "WITH a.name AS name";
        let (_, clause) = with_clause(input).unwrap();
        assert_eq!(clause.items.len(), 1);
    }

    #[test]
    fn test_with_clause_wildcard() {
        let input = "WITH *";
        let (_, clause) = with_clause(input).unwrap();
        assert_eq!(clause.items.len(), 1);
    }

    #[test]
    fn test_with_clause_multiple() {
        let input = "WITH a, count(*) AS count, b.name AS name";
        let (_, clause) = with_clause(input).unwrap();
        assert_eq!(clause.items.len(), 3);
    }
}
