use nom::{bytes::complete::take_while1, IResult};

pub fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
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

}
