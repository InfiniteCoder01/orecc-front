/// A simple character reader that is useful for creating lexers
#[derive(Debug)]
pub struct TokenReader {
    source: std::rc::Rc<str>,

    /// A cursor. Just a byte index, useful for spanning and then [codespan_reporting]
    pub cursor: usize,
}

impl TokenReader {
    /// Create new [TokenReader]. Just requires a source as Rc<str>, can be provided by the [crate::Codebase]
    pub fn new(source: std::rc::Rc<str>) -> Self {
        Self {
            source,
            cursor: 0,
        }
    }

    /// Peek a char
    pub fn peek_char(&mut self) -> Option<char> {
        self.source.get(self.cursor..)?.chars().next()
    }

    /// Peek a char, return it and move cursor forward
    pub fn next_char(&mut self) -> Option<char> {
        let peek = self.peek_char();
        if let Some(peek) = peek {
            self.cursor += peek.len_utf8();
        }
        peek
    }

    /// [`Self::next_char()`] but advances if predicate
    pub fn next_char_if(&mut self, pred: impl FnOnce(char) -> bool) -> Option<char> {
        let peek = self.peek_char();
        if let Some(peek) = peek {
            if pred(peek) {
                self.cursor += peek.len_utf8();
                return Some(peek);
            }
        }
        None
    }

    /// Takes characters and adds then to the buffer while predicate
    pub fn next_token(&mut self, pred: impl Fn(char) -> bool, prefix: impl Into<String>) -> String {
        let mut buffer = prefix.into();
        while let Some(char) = self.next_char_if(&pred) {
            buffer.push(char);
        }
        buffer
    }
}
