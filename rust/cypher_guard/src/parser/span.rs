#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spanned<T> {
    pub value: T,
    pub start: usize, // byte offset in the input
}

impl<T> Spanned<T> {
    pub fn new(value: T, start: usize) -> Self {
        Self { value, start }
    }
}

/// Convert a byte offset to line and column numbers
///
/// # Arguments
/// * `input` - The full input string
/// * `byte_offset` - The byte offset to convert
///
/// # Returns
/// A tuple of (line, column) where both are 1-indexed
pub fn offset_to_line_column(input: &str, byte_offset: usize) -> (usize, usize) {
    if byte_offset == 0 {
        return (1, 1);
    }
    if byte_offset >= input.len() {
        // If offset is beyond the input, return the last line/column
        let mut line = 1;
        let mut column = 1;
        for ch in input.chars() {
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        return (line, column);
    }

    let mut line = 1;
    let mut column = 1;
    let mut idx = 0;

    for ch in input.chars() {
        let ch_len = ch.len_utf8();
        let start_idx = idx;
        let end_idx = idx + ch_len;

        // Check if the offset is within this character's byte range
        if byte_offset >= start_idx && byte_offset < end_idx {
            if ch == '\n' && byte_offset == start_idx {
                return (line + 1, 1);
            } else {
                return (line, column);
            }
        }

        // Update for next iteration
        idx += ch_len;
        if ch == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    (line, column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_to_line_column_single_line() {
        let input = "MATCH (a:Person) RETURN a";
        assert_eq!(offset_to_line_column(input, 0), (1, 1)); // 'M'
        assert_eq!(offset_to_line_column(input, 5), (1, 6)); // 'H'
        assert_eq!(offset_to_line_column(input, 10), (1, 11)); // ' '
        assert_eq!(offset_to_line_column(input, 25), (1, 26)); // 'a'
    }

    #[test]
    fn test_offset_to_line_column_multiline() {
        let input = "MATCH (a:Person)\nWHERE a.age > 30\nRETURN a";
        println!("Input: {:?}", input);
        println!("Input length: {}", input.len());
        println!("Input bytes: {:?}", input.as_bytes());

        assert_eq!(offset_to_line_column(input, 0), (1, 1)); // 'M'
        assert_eq!(offset_to_line_column(input, 5), (1, 6)); // 'H'
        assert_eq!(offset_to_line_column(input, 10), (1, 11)); // ' '

        let result = offset_to_line_column(input, 15);
        println!("offset_to_line_column(input, 15) = {:?}", result);
        assert_eq!(result, (1, 16)); // ')'

        let result = offset_to_line_column(input, 16);
        println!("offset_to_line_column(input, 16) = {:?}", result);
        assert_eq!(result, (2, 1)); // 'W'

        assert_eq!(offset_to_line_column(input, 30), (2, 14)); // '>'
        assert_eq!(offset_to_line_column(input, 32), (2, 16)); // '0'
        assert_eq!(offset_to_line_column(input, 38), (3, 5)); // 'U'
    }

    #[test]
    fn test_offset_to_line_column_unicode() {
        let input = "MATCH (a:Person) -- 中文注释\nRETURN a";
        assert_eq!(offset_to_line_column(input, 0), (1, 1)); // 'M'
        assert_eq!(offset_to_line_column(input, 20), (1, 21)); // '中'
        assert_eq!(offset_to_line_column(input, 26), (1, 23)); // '注'
    }

    #[test]
    fn test_offset_to_line_column_beyond_input() {
        let input = "MATCH (a:Person)";
        assert_eq!(offset_to_line_column(input, 100), (1, 17)); // Last character
    }
}
