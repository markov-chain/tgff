//! The package provides a parser for the
//! [TGFF](http://ziyang.eecs.umich.edu/~dickrp/tgff/) (Task Graphs For Free)
//! format, which is a format for storing task graphs and accompanying data
//! used in scheduling and allocation research.

#![feature(macro_rules)]

pub type ParserResult<T> = Result<T, ParserError>;

pub struct ParserError {
    line: uint,
    message: &'static str,
}

impl std::fmt::Show for ParserError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{} on line {}", self.message, self.line)
    }
}

macro_rules! raise(
    ($parser:expr, $message:expr) => (
        return Err(ParserError { line: $parser.line, message: $message });
    );
)

pub struct Parser<'a> {
    data: &'a str,
    cursor: std::iter::Peekable<(uint, char), std::str::CharOffsets<'a>>,
    line: uint,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a str) -> Parser<'a> {
        Parser {
            data: data,
            cursor: data.char_indices().peekable(),
            line: 1,
        }
    }

    pub fn process(&mut self) -> ParserResult<()> {
        return Ok(());

        loop {
            self.skip_whitespace();

            match self.peek() {
                Some(&(_, '\n')) => self.skip(),
                Some(&(_, '@')) => { self.process_at(); },
                _ => {},
            }
        }

        Ok(())
    }

    #[inline]
    fn peek(&mut self) -> Option<&(uint, char)> {
        self.cursor.peek()
    }

    #[inline]
    fn skip(&mut self) {
        self.next();
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(&(_, '\t')) |
                Some(&(_, ' ')) => self.skip(),
                _ => break,
            }
        }
    }

    fn process_at(&mut self) -> ParserResult<()> {
        self.skip(); // @

        let mut name = String::new();
        for (_, c) in self {
            match c {
                ' ' | '\t' => break,
                _ => name.push(c),
            }
        }

        if name.is_empty() {
            raise!(self, "the name of an @-statement should not be empty");
        }

        self.skip_whitespace();

        let mut number = String::new();
        for (_, c) in self {
            match c {
                ' ' | '\t' => break,
                _ => name.push(c),
            }
        }

        if number.is_empty() {
            raise!(self, "the number of an @-statement should not be empty");
        }

        Ok(())
    }
}

impl<'a> std::iter::Iterator<(uint, char)> for Parser<'a> {
    fn next(&mut self) -> Option<(uint, char)> {
        let pair = self.cursor.next();
        match pair {
            Some((_, '\n')) => self.line += 1,
            _ => {},
        }
        pair
    }
}

#[cfg(test)]
mod tests {
    macro_rules! assert_err(
        ($e: expr) => (
            if let Ok(_) = $e {
                assert!(false, "expected an error");
            }
        );
    )

    #[test]
    fn skip_whitespace() {
        let mut parser = super::Parser::new("  \t  abc");
        parser.skip_whitespace();
        assert_eq!(parser.next().unwrap(), (5, 'a'));
    }

    #[test]
    fn process_at() {
        let mut parser = super::Parser::new("@ ");
        assert_err!(parser.process_at());

        parser = super::Parser::new("@abc");
        assert_err!(parser.process_at());
    }
}
