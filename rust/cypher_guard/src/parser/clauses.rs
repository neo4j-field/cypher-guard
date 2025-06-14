use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::{map, opt, recognize},
    multi::{many1, separated_list0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

use crate::parser::ast;
use crate::parser::ast::{
    CreateClause, MatchClause, MatchElement, MergeClause, OnCreateClause, OnMatchClause,
    PropertyValue, Query, ReturnClause, SetClause, WithClause, WithExpression, WithItem,
};
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

// Parses a property access pattern (e.g., a.name)
fn property_access(input: &str) -> IResult<&str, String> {
    println!("[property_access] >>> ENTER: input='{}'", input);
    let (input, var) = identifier(input)?;
    println!("[property_access] Parsed variable: {}", var);
    let (input, _) = char('.')(input)?;
    println!("[property_access] After dot: input='{}'", input);
    let (input, prop) = identifier(input)?;
    println!("[property_access] Parsed property: {}", prop);
    let result = format!("{}.{}", var, prop);
    println!("[property_access] <<< EXIT: {}", result);
    Ok((input, result))
}

// Parses a function call (e.g., length(a.name), substring(a.name, 0, 5))
fn function_call(input: &str) -> IResult<&str, (String, Vec<String>)> {
    println!("[function_call] >>> ENTER: input='{}'", input);
    let (input, _) = multispace0(input)?;
    let (input, function) = map(identifier, |s| s.to_string())(input)?;
    println!("[function_call] Parsed function name: {}", function);
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, _) = multispace0(input)?;

    // Parse arguments
    let (input, args) = separated_list0(
        tuple((multispace0, tag(","), multispace0)),
        alt((
            // Try to parse nested function calls
            map(function_call, |(func, args)| {
                format!("{}({})", func, args.join(", "))
            }),
            // Try to parse property access
            map(property_access, |s| s),
            // Try to parse string literals
            map(string_literal, |s| format!("'{}'", s)),
            // Try to parse numeric literals
            map(numeric_literal, |n| n.to_string()),
            // Try to parse boolean literals
            map(tag_no_case("true"), |_| "true".to_string()),
            map(tag_no_case("false"), |_| "false".to_string()),
            // Try to parse NULL
            map(tag_no_case("NULL"), |_| "NULL".to_string()),
            // Try to parse identifiers
            map(identifier, |s| s.to_string()),
        )),
    )(input)?;

    let (input, _) = multispace0(input)?;
    let (input, _) = tag(")")(input)?;
    println!("[function_call] <<< EXIT: {}({:?})", function, args);
    Ok((input, (function, args)))
}

// Parses a WHERE condition (e.g., a.age > 30, a.name = 'Alice', a.name IS NULL)
fn where_condition(input: &str) -> IResult<&str, ast::WhereCondition> {
    println!("[where_condition] >>> ENTER: input='{}'", input);
    let (input, _) = multispace0(input)?;

    // Try to parse NOT
    if let Ok((rest, _)) = tag::<&str, &str, nom::error::Error<&str>>("NOT")(input) {
        let (rest, _) = multispace1(rest)?;
        let (rest, condition) = where_condition(rest)?;
        return Ok((rest, ast::WhereCondition::Not(Box::new(condition))));
    }

    // Try to parse parenthesized condition
    if let Ok((rest, _)) = tag::<&str, &str, nom::error::Error<&str>>("(")(input) {
        let (rest, condition) = where_condition(rest)?;
        let (rest, _) = tag::<&str, &str, nom::error::Error<&str>>(")")(rest)?;
        return Ok((
            rest,
            ast::WhereCondition::Parenthesized(Box::new(condition)),
        ));
    }

    // Try to parse as a function call
    if let Ok((rest, (function, args))) = function_call(input) {
        println!(
            "[where_condition] Parsed function call: {}({:?})",
            function, args
        );
        return Ok((
            rest,
            ast::WhereCondition::FunctionCall {
                function,
                arguments: args,
            },
        ));
    }

    // Try to parse as a comparison first
    let comparison_result = (|| {
        let (input, left) = alt((
            map(property_access, |s| s),
            map(identifier, |s| s.to_string()),
        ))(input)?;
        println!("[where_condition] Parsed left side: {}", left);
        let (input, _) = multispace0(input)?;
        let (input, operator) = alt((
            tag("="),
            tag("<>"),
            tag("<"),
            tag(">"),
            tag("<="),
            tag(">="),
            tag("IS NULL"),
            tag("IS NOT NULL"),
        ))(input)?;
        println!("[where_condition] Parsed operator: {}", operator);
        let (input, _) = multispace0(input)?;
        let (input, right) = alt((
            map(string_literal, |s| format!("'{}'", s)),
            map(numeric_literal, |n| n),
            map(identifier, |s| s.to_string()),
        ))(input)?;
        println!("[where_condition] Parsed right side: {}", right);
        Ok((
            input,
            ast::WhereCondition::Comparison {
                left,
                operator: operator.to_string(),
                right,
            },
        ))
    })();

    if let Ok(result) = comparison_result {
        return Ok(result);
    }

    // If comparison parsing failed, try to parse as a path property
    if let Ok((rest, (path_var, property))) = path_property(input) {
        println!(
            "[where_condition] Parsed path property: {}.{}",
            path_var, property
        );
        return Ok((
            rest,
            ast::WhereCondition::PathProperty { path_var, property },
        ));
    }

    // If all parsing attempts failed, return the error from the comparison attempt
    comparison_result
}

// Parses the WHERE clause (e.g. WHERE a.age > 30 AND b.name = 'Alice')
pub fn where_clause(input: &str) -> IResult<&str, ast::WhereClause> {
    println!("[where_clause] >>> ENTER: input='{}'", input);
    let (input, _) = multispace0(input)?;
    println!("[where_clause] After initial whitespace: input='{}'", input);
    let (input, _) = tag("WHERE")(input)?;
    println!("[where_clause] After WHERE tag: input='{}'", input);
    let (input, _) = multispace1(input)?;
    println!("[where_clause] After WHERE whitespace: input='{}'", input);

    // Parse conditions separated by AND or OR
    let (input, conditions) = separated_list1(
        tuple((multispace0, alt((tag("AND"), tag("OR"))), multispace0)),
        where_condition,
    )(input)?;
    println!("[where_clause] Parsed conditions: {:?}", conditions);

    let clause = ast::WhereClause { conditions };
    println!("[where_clause] <<< EXIT: {:?}", clause);
    Ok((input, clause))
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
    let (input, set_clauses) =
        match separated_list1(tuple((multispace0, char(','), multispace0)), set_clause)(input) {
            Ok(res) => {
                println!("DEBUG: Successfully parsed set_clauses");
                res
            }
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
        println!(
            "DEBUG: After consuming whitespace, remaining input: {}",
            input
        );

        if found_on_create.is_none() {
            println!("DEBUG: Trying to parse ON CREATE clause");
            match on_create_clause(input) {
                Ok((rest, clause)) => {
                    println!("DEBUG: Successfully parsed ON CREATE clause");
                    found_on_create = Some(clause);
                    input = rest;
                    continue;
                }
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
                }
                Err(e) => {
                    println!("DEBUG: Failed to parse ON MATCH clause: {:?}", e);
                }
            }
        }
        println!("DEBUG: No more ON clauses found");
        break;
    }
    println!(
        "DEBUG: Final merge clause state - on_create: {:?}, on_match: {:?}",
        found_on_create, found_on_match
    );
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

// Parses a WITH item (e.g., a, a.name, count(*))
fn with_item(input: &str) -> IResult<&str, WithItem> {
    println!("[with_item] >>> ENTER: input='{}'", input);
    let (input, _) = multispace0(input)?;
    let (input, expr) = alt((
        map(char('*'), |_| WithExpression::Wildcard),
        map(property_access, |s| {
            let parts: Vec<&str> = s.split('.').collect();
            WithExpression::PropertyAccess {
                variable: parts[0].to_string(),
                property: parts[1].to_string(),
            }
        }),
        map(function_call, |(name, args)| WithExpression::FunctionCall {
            name,
            args: args.into_iter().map(WithExpression::Identifier).collect(),
        }),
        map(identifier, |s| WithExpression::Identifier(s.to_string())),
    ))(input)?;
    let (input, alias) = opt(preceded(
        tuple((multispace0, tag("AS"), multispace1)),
        identifier,
    ))(input)?;
    let result = WithItem {
        expression: expr,
        alias: alias.map(|s| s.to_string()),
    };
    println!(
        "[with_item] <<< EXIT: result={:?}, remaining input='{}'",
        result, input
    );
    Ok((input, result))
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

// Parses a property value (e.g., 42, 'hello', true, [1, 2, 3], {name: 'Alice'})
fn property_value(input: &str) -> IResult<&str, PropertyValue> {
    println!("[property_value] >>> ENTER: input='{}'", input);
    let (input, _) = multispace0(input)?;

    // Try to parse as a list/array
    if let Ok((rest, _)) = char::<_, nom::error::Error<&str>>('[')(input) {
        let (rest, items) = separated_list0(
            tuple((multispace0, char(','), multispace0)),
            alt((
                map(string_literal, PropertyValue::String),
                map(identifier, |s| PropertyValue::String(s.to_string())),
                map(numeric_literal, |n| {
                    PropertyValue::Number(n.parse().unwrap())
                }),
                map(tag_no_case("true"), |_| PropertyValue::Boolean(true)),
                map(tag_no_case("false"), |_| PropertyValue::Boolean(false)),
                map(tag_no_case("NULL"), |_| PropertyValue::Null),
            )),
        )(rest)?;
        let (rest, _) = char(']')(rest)?;
        println!("[property_value] <<< EXIT: List({:?})", items);
        return Ok((rest, PropertyValue::List(items)));
    }

    // Try to parse as a map/object
    if let Ok((rest, _)) = char::<_, nom::error::Error<&str>>('{')(input) {
        let (rest, pairs) = separated_list0(
            tuple((multispace0, char(','), multispace0)),
            tuple((
                identifier,
                tuple((multispace0, char(':'), multispace0)),
                alt((
                    map(string_literal, PropertyValue::String),
                    map(identifier, |s| PropertyValue::String(s.to_string())),
                    map(numeric_literal, |n| {
                        PropertyValue::Number(n.parse().unwrap())
                    }),
                    map(tag_no_case("true"), |_| PropertyValue::Boolean(true)),
                    map(tag_no_case("false"), |_| PropertyValue::Boolean(false)),
                    map(tag_no_case("NULL"), |_| PropertyValue::Null),
                )),
            )),
        )(rest)?;
        let (rest, _) = char('}')(rest)?;
        let map: std::collections::HashMap<String, PropertyValue> = pairs
            .into_iter()
            .map(|(k, _, v)| (k.to_string(), v))
            .collect();
        println!("[property_value] <<< EXIT: Map({:?})", map);
        return Ok((rest, PropertyValue::Map(map)));
    }

    // Try to parse as a primitive value
    let (input, value) = alt((
        map(string_literal, PropertyValue::String),
        map(identifier, |s| PropertyValue::String(s.to_string())),
        map(numeric_literal, |n| {
            PropertyValue::Number(n.parse().unwrap())
        }),
        map(tag_no_case("true"), |_| PropertyValue::Boolean(true)),
        map(tag_no_case("false"), |_| PropertyValue::Boolean(false)),
        map(tag_no_case("NULL"), |_| PropertyValue::Null),
    ))(input)?;
    println!("[property_value] <<< EXIT: {:?}", value);
    Ok((input, value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Direction, PatternElement};

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
        let input =
            "CREATE (a:Person {name: 'Alice'})-[r:KNOWS {since: 2020}]->(b:Person {name: 'Bob'})";
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
        let input = "WITH a, b.name AS name";
        let (_, clause) = with_clause(input).unwrap();
        assert_eq!(clause.items.len(), 2);
    }
}
