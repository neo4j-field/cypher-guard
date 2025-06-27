use nom::{branch::alt, bytes::complete::take_while1, character::complete::char, IResult};

pub fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

#[allow(dead_code)]
// TODO: Clean up unused functions or refactor to use them
pub fn string_literal(input: &str) -> IResult<&str, String> {
    let (input, quote) = alt((char('\''), char('"')))(input)?;
    let (input, s) = take_while1(|c| c != quote)(input)?;
    let (input, _) = char(quote)(input)?;
    Ok((input, s.to_string()))
}

#[allow(dead_code)]
// TODO: Clean up unused functions or refactor to use them
pub fn number_literal(input: &str) -> IResult<&str, i64> {
    let (input, n) = take_while1(|c: char| c.is_ascii_digit())(input)?;
    Ok((input, n.parse().unwrap()))
}

#[allow(dead_code)]
// TODO: Clean up unused functions or refactor to use them
pub fn opt_identifier(input: &str) -> IResult<&str, Option<String>> {
    let (input, _) = nom::character::complete::multispace0(input)?;
    match identifier(input) {
        Ok((input, id)) => Ok((input, Some(id.to_string()))),
        Err(_) => Ok((input, None)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_basic() {
        let result = identifier("abc123");
        assert!(result.is_ok());
        let (rest, id) = result.unwrap();
        assert_eq!(id, "abc123");
        assert_eq!(rest, "");
    }

    #[test]
    fn test_identifier_with_underscore() {
        let result = identifier("foo_bar");
        assert!(result.is_ok());
        let (rest, id) = result.unwrap();
        assert_eq!(id, "foo_bar");
        assert_eq!(rest, "");
    }

    #[test]
    fn test_identifier_stops_on_non_alnum() {
        let result = identifier("foo-bar");
        assert!(result.is_ok());
        let (rest, id) = result.unwrap();
        assert_eq!(id, "foo");
        assert_eq!(rest, "-bar");
    }

    #[test]
    fn test_identifier_empty() {
        let result = identifier("");
        assert!(result.is_err());
    }

    #[test]
    fn test_string_literal_single_quotes() {
        let result = string_literal("'hello world'");
        assert!(result.is_ok());
        let (rest, s) = result.unwrap();
        assert_eq!(s, "hello world");
        assert_eq!(rest, "");
    }

    #[test]
    fn test_string_literal_double_quotes() {
        let result = string_literal("\"foo\"");
        assert!(result.is_ok());
        let (rest, s) = result.unwrap();
        assert_eq!(s, "foo");
        assert_eq!(rest, "");
    }

    #[test]
    fn test_string_literal_stops_on_quote() {
        let result = string_literal("'foo bar'baz");
        assert!(result.is_ok());
        let (rest, s) = result.unwrap();
        assert_eq!(s, "foo bar");
        assert_eq!(rest, "baz");
    }

    #[test]
    fn test_string_literal_empty() {
        let result = string_literal("''");
        assert!(result.is_err());
    }

    #[test]
    fn test_number_literal_basic() {
        let result = number_literal("12345");
        assert!(result.is_ok());
        let (rest, n) = result.unwrap();
        assert_eq!(n, 12345);
        assert_eq!(rest, "");
    }

    #[test]
    fn test_number_literal_stops_on_non_digit() {
        let result = number_literal("42abc");
        assert!(result.is_ok());
        let (rest, n) = result.unwrap();
        assert_eq!(n, 42);
        assert_eq!(rest, "abc");
    }

    #[test]
    fn test_number_literal_empty() {
        let result = number_literal("");
        assert!(result.is_err());
    }

    #[test]
    fn test_opt_identifier_present() {
        let result = opt_identifier("foo");
        assert!(result.is_ok());
        let (rest, id) = result.unwrap();
        assert_eq!(id, Some("foo".to_string()));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_opt_identifier_with_whitespace() {
        let result = opt_identifier("   bar");
        assert!(result.is_ok());
        let (rest, id) = result.unwrap();
        assert_eq!(id, Some("bar".to_string()));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_opt_identifier_absent() {
        let result = opt_identifier("   ");
        assert!(result.is_ok());
        let (rest, id) = result.unwrap();
        assert_eq!(id, None);
        assert_eq!(rest, "");
    }
}
