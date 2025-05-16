use nom::{
    bytes::complete::take_while1,
    character::complete::char,
    IResult,
};

pub fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

pub fn string_literal(input: &str) -> IResult<&str, String> {
    let (input, _) = char('"')(input)?;
    let (input, s) = take_while1(|c| c != '"')(input)?;
    let (input, _) = char('"')(input)?;
    Ok((input, s.to_string()))
}

pub fn number_literal(input: &str) -> IResult<&str, i64> {
    let (input, n) = take_while1(|c: char| c.is_ascii_digit())(input)?;
    Ok((input, n.parse().unwrap()))
}

pub fn opt_identifier(input: &str) -> IResult<&str, Option<String>> {
    let (input, _) = nom::character::complete::multispace0(input)?;
    match identifier(input) {
        Ok((input, id)) => Ok((input, Some(id.to_string()))),
        Err(_) => Ok((input, None)),
    }
}
