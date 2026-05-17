/// Span represents a range in source code as byte offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    /// Start byte offset (inclusive)
    pub start: u32,
    /// End byte offset (exclusive)
    pub end: u32,
}

impl Span {
    /// Create a new span
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Create a zero-length span at the given position
    pub const fn at(pos: u32) -> Self {
        Self { start: pos, end: pos }
    }

    /// Get the length of the span in bytes
    pub const fn len(&self) -> u32 {
        self.end - self.start
    }

    /// Check if span is empty
    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Get the range as a tuple
    pub const fn range(&self) -> (u32, u32) {
        (self.start, self.end)
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(range: std::ops::Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            end: range.end as u32,
        }
    }
}

impl From<Span> for std::ops::Range<usize> {
    fn from(span: Span) -> Self {
        (span.start as usize)..(span.end as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_new() {
        let span = Span::new(0, 5);
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 5);
    }

    #[test]
    fn test_span_at() {
        let span = Span::at(10);
        assert_eq!(span.start, 10);
        assert_eq!(span.end, 10);
        assert!(span.is_empty());
    }

    #[test]
    fn test_span_len() {
        let span = Span::new(0, 10);
        assert_eq!(span.len(), 10);
    }

    #[test]
    fn test_span_is_empty() {
        assert!(Span::new(5, 5).is_empty());
        assert!(!Span::new(5, 10).is_empty());
    }

    #[test]
    fn test_span_range() {
        let span = Span::new(3, 7);
        assert_eq!(span.range(), (3, 7));
    }

    #[test]
    fn test_from_range() {
        let range = 2..8;
        let span = Span::from(range);
        assert_eq!(span.start, 2);
        assert_eq!(span.end, 8);
    }

    #[test]
    fn test_into_range() {
        let span = Span::new(2, 8);
        let range: std::ops::Range<usize> = span.into();
        assert_eq!(range, 2..8);
    }
}