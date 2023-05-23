use std::str::Chars;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum Kind {
    Section(String)
}

pub struct Lexer<'a> {
    pub input: &'a str,
    pub chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.chars(),
        }
    }

    pub fn read_next_token(&mut self) -> Token {
        let start = self.offset();
        let kind = self.read_next_kind();
        let end = self.offset();
        Token { kind, start, end }
    }

    fn read_next_kind(&mut self) -> Kind {
        //I want include the section value in the token
        match self.chars.next() {
            Some('[') => self.read_next_section(),
            _ => panic!("Invalid token"),
        }
    }

    fn read_next_section(&mut self) -> Kind {
        let mut section = String::new();
        loop {
            match self.chars.next() {
                Some(']') => break,
                Some(c) => section.push(c),
                None => panic!("Invalid token"),
            }
        }
        Kind::Section(section)
    }

    fn offset(&self) -> usize {
        self.input.len() - self.chars.as_str().len()
    }
}


#[cfg(test)]
mod test {
    use super::{Kind, Lexer};

    //I want to write a test to assert the lexer is able to identify section on an Epanet file
    #[test]
    fn lexer_read_section() {
        let input = "[TITLE]";
        let mut lexer = Lexer::new(input);
        let token = lexer.read_next_token();
        assert_eq!(token.kind, Kind::Section("TITLE".to_string()));
        assert_eq!(token.start, 0);
        assert_eq!(token.end, 7);
    }

}

