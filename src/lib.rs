//! The package provides a parser for the
//! [TGFF](http://ziyang.eecs.umich.edu/~dickrp/tgff/) (Task Graphs For Free)
//! format, which is a format for storing task graphs and accompanying data
//! used in scheduling and allocation research.

#![feature(macro_rules, if_let)]

use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharOffsets;

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

static READ_CAPACITY: uint = 20;

pub struct Parser<'a> {
    input: &'a str,
    cursor: Peekable<(uint, char), CharOffsets<'a>>,
    line: uint,

    attributes: HashMap<String, String>,
}

macro_rules! raise(
    ($parser:expr, $message:expr) => (
        return Err(ParserError { line: $parser.line, message: $message });
    );
)

macro_rules! if_void_else(
    ($result:expr, $yes:expr, $no:expr) => (
        match $result {
            ' ' | '\t' | '\n' => { $yes; },
            _ => { $no; },
        }
    );
)

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            input: input,
            cursor: input.char_indices().peekable(),
            line: 1,

            attributes: HashMap::new(),
        }
    }

    pub fn process(&mut self) -> ParserResult<()> {
        return Ok(());

        loop {
            self.skip_void();

            match self.peek() {
                Some('@') => { self.process_at(); },
                _ => {},
            }
        }

        Ok(())
    }

    #[inline]
    fn peek(&mut self) -> Option<char> {
        match self.cursor.peek() {
            Some(&(_, c)) => Some(c),
            None => None,
        }
    }

    #[inline]
    fn skip(&mut self) {
        self.next();
    }

    fn skip_void(&mut self) {
        loop {
            match self.peek() {
                Some(c) => if_void_else!(c, self.skip(), break),
                _ => break,
            }
        }
    }

    fn process_at(&mut self) -> ParserResult<()> {
        self.skip(); // @

        let name = match self.read_name() {
            Some(name) => name,
            None => raise!(self, "the name of an @-statement should not be empty"),
        };

        self.skip_void();

        let number = match self.read_number() {
            Some(number) => number,
            None => raise!(self, "an @-statement should be followed by a positive integer"),
        };

        self.skip_void();

        if let Some('{') = self.peek() {
            self.process_block(name, number)
        } else {
            self.set_attribute(name, number)
        }
    }

    fn process_block(&mut self, name: String, number: String) -> ParserResult<()> {
        self.skip(); // {

        loop {
            self.skip_void();

            match self.peek() {
                Some('}') => {
                    self.skip();
                    return Ok(());
                },
                _ => break,
            }
        }

        raise!(self, "cannot find the end of a block");
    }

    fn read(&mut self, accept: |uint, char| -> bool) -> Option<String> {
        let mut result = std::string::String::with_capacity(READ_CAPACITY);
        let mut i = 0;

        loop {
            match self.peek() {
                Some(c) => {
                    if !accept(i, c) { break; }
                    self.skip();
                    result.push(c);
                    i += 1;
                },
                None => break,
            }
        }

        if i == 0 {
            None
        } else {
            Some(result)
        }
    }

    fn read_name(&mut self) -> Option<String> {
        self.read(|i, c| {
            match c {
                'A'...'Z' | 'a'...'z' if i == 0 => true,
                'A'...'Z' | 'a'...'z' | '_' | '0'...'9' if i > 0 => true,
                _ => false,
            }
        })
    }

    fn read_number(&mut self) -> Option<String> {
        self.read(|_, c| c >= '0' && c <= '9')
    }

    #[inline]
    fn set_attribute(&mut self, name: String, value: String) -> ParserResult<()> {
        self.attributes.insert(name, value);
        Ok(())
    }
}

impl<'a> std::iter::Iterator<char> for Parser<'a> {
    fn next(&mut self) -> Option<char> {
        match self.cursor.next() {
            Some((_, '\n')) => {
                self.line += 1;
                Some('\n')
            },
            Some((_, c)) => Some(c),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    macro_rules! assert_ok(
        ($result: expr) => (
            if let Err(err) = $result {
                assert!(false, "{}", err);
            }
        );
    )

    macro_rules! assert_err(
        ($result: expr) => (
            if let Ok(_) = $result {
                assert!(false, "expected an error");
            }
        );
    )

    #[test]
    fn skip_void() {
        let mut parser = Parser::new("  \t  abc");
        parser.skip_void();
        assert_eq!(parser.next().unwrap(), 'a');
    }

    #[test]
    fn process_at() {
        assert_ok!(Parser::new("@abc 12").process_at());
        assert_err!(Parser::new("@ ").process_at());
        assert_err!(Parser::new("@abc").process_at());
    }

    #[test]
    fn process_block() {
        assert_ok!(Parser::new("{}").process_block(String::from_str("name"),
                                                   String::from_str("number")));
    }

    #[test]
    fn read_name() {
        macro_rules! test(
            ($input:expr, $output:expr) => (
                assert_eq!(Parser::new($input).read_name().unwrap(),
                           String::from_str($output));
            );
        )

        test!("AZ xyz", "AZ");
        test!("az xyz", "az");
        test!("AZ_az_09 xyz", "AZ_az_09");
    }
}
