use std::ops::{Bound, RangeBounds};

pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use pretty_assertions::assert_eq;
    use super::StringUtils;

    #[test_case(0, 4, "abcd" ; "len with in the string length")]
    #[test_case(0, 999, "abcdefghijk" ; "len large than string length")]
    #[test_case(6, 1, "g" ; "start large than len")]
    #[test_case(6, 0, "" ; "len equal zero")]
    #[test_case(0, 0, "" ; "start and len equal")]
    fn substring_with_normal_charater(start: usize, end: usize, expected: &str) {
        // Arrange

        // Act
        let result = "abcdefghijk".substring(start, end);

        // Assert
        assert_eq!(result, expected);
    }

    #[test_case(0, 4, "abcd" ; "len with in the string length")]
    #[test_case(0, 999, "abcdefghijk" ; "len large than string length")]
    #[test_case(0, 0, "" ; "start and len equal")]
    fn slice_with_normal_charater(start: usize, end: usize, expected: &str) {
        // Arrange

        // Act
        let result = "abcdefghijk".slice(start..end);

        // Assert
        assert_eq!(result, expected);
    }
}