use super::asm;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::{fmt::Display, iter::FromIterator};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumberKind {
    Decimal,
    Hexadecimal,
    Binary,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NumberType {
    U8,
    U16,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Asm(String),
    Label(String),
    Identifier(String), // for macro params
    // Data directives
    DB,
    DW,
    Set,
    Print,
    // next three are specific to print
    Mem,
    Reg,
    Flags,

    Size,
    Macro,
    MacroStart,
    MacroEnd,
    RightBracket,
    LeftBracket,
    Comma,
    String(String),
    Number {
        value: u16,
        kind: NumberKind,
        typ: NumberType,
    },
    EOL,
    EOF,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub offset: usize,
    pub line: usize,
    pub typ: TokenType,
}

pub struct LexingError {
    line: usize,
    offset: usize,
    error: String,
}

impl Display for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}:{} {}", self.line, self.offset, self.error)
    }
}

impl std::fmt::Debug for LexingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}:{} {}", self.line, self.offset, self.error)
    }
}

impl LexingError {
    pub fn new<T: Into<String>>(line: usize, offset: usize, err: T) -> Self {
        Self {
            line,
            offset,
            error: err.into(),
        }
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut hm = HashMap::new();
        hm.insert("print", TokenType::Print);
        hm.insert("mem", TokenType::Mem);
        hm.insert("reg", TokenType::Reg);
        hm.insert("flags", TokenType::Flags);
        hm.insert("set", TokenType::Set);
        hm.insert("db", TokenType::DB);
        hm.insert("dw", TokenType::DW);
        hm.insert("macro", TokenType::Macro);
        hm
    };
}

pub struct Lexer {
    source: Vec<char>,
    src_length: usize,
    line: usize,
    line_pos: usize,
    current: usize,
    start: usize,
    tokens: Vec<Token>,
    errors: Vec<LexingError>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let src: Vec<char> = input.chars().collect();
        let src_length = src.len();
        Self {
            source: src,
            src_length,
            line: 1,
            line_pos: 0,
            current: 0,
            start: 0,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Vec<LexingError>> {
        while !self.is_at_end() {
            self.scan_token();
            self.start = self.current;
        }
        self.add_token(TokenType::EOF);
        if !self.errors.is_empty() {
            return Err(self.errors);
        } else {
            return Ok(self.tokens);
        }
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.current >= self.src_length
    }

    #[inline]
    fn get_offset(&self) -> usize {
        // index starts at 0, but we want offset starting at 1,  hence +1
        self.start - self.line_pos + 1
    }

    fn error<T: Into<String>>(&mut self, err: T) {
        self.errors
            .push(LexingError::new(self.line, self.get_offset(), err));
    }

    fn advance(&mut self) -> char {
        let ret = self.source[self.current];
        self.current += 1;
        ret
    }

    fn advance_line_data(&mut self) {
        self.line += 1;
        self.line_pos = self.current;
    }

    fn consume_if_matches(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != c {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
        }
    }

    fn consume_string(&mut self) {
        let mut temp: Vec<char> = Vec::new();
        let start_line = self.line;
        let start_offset = self.get_offset();
        while !self.is_at_end() && self.peek() != Some('"') {
            if self.peek() == Some('\n') {
                self.advance_line_data();
            }
            temp.push(self.advance());
        }
        if self.is_at_end() {
            self.error(format!(
                "unterminated string starting at line {start_line}:{start_offset}"
            ));
            return;
        }
        self.advance(); // for ending "
        let str: String = String::from_iter(temp.into_iter());
        self.add_token(TokenType::String(str));
    }

    fn consume_number(&mut self, start_char: char) {
        let kind;
        let mut temp: Vec<char> = vec![start_char]; // even if it is hex, starting 0 doesn't matter
        let digit_validator: Box<dyn Fn(char) -> bool>;
        if start_char == '0' && self.peek() == Some('x') {
            kind = NumberKind::Hexadecimal;
            digit_validator = Box::new(|c| matches!(c,'0'..='9'|'a'..='f'|'A'..='F'));
            self.advance(); // skip the x
        } else if start_char == '0' && self.peek() == Some('b') {
            kind = NumberKind::Binary;
            digit_validator = Box::new(|c| matches!(c, '0' | '1'));
            self.advance(); // skip the b
        } else {
            kind = NumberKind::Decimal;
            digit_validator = Box::new(|c| matches!(c, '0'..='9'));
        }
        while self.peek() != None {
            if !digit_validator(self.peek().unwrap()) {
                // next character is not a digit, so the number is over
                break;
            } else {
                temp.push(self.advance());
            }
        }
        let str = String::from_iter(temp.into_iter());
        let radix;
        match kind {
            NumberKind::Decimal => radix = 10,
            NumberKind::Hexadecimal => radix = 16,
            NumberKind::Binary => radix = 2,
        }

        match u16::from_str_radix(&str, radix) {
            Ok(val) => {
                let typ = if val <= u8::MAX as u16 {
                    NumberType::U8
                } else {
                    NumberType::U16
                };
                self.add_token(TokenType::Number {
                    value: val,
                    kind,
                    typ,
                })
            }
            Err(e) => {
                self.error(format!(
                    "error in parsing number '{str}' : {e}Note that only 0->65535 can be used"
                ));
            }
        }
    }

    fn consume_and_return_identifier(&mut self, start_char: char) -> String {
        let mut temp = vec![start_char];
        while self.peek() != None {
            let next = self.peek().unwrap();
            if matches!(next,'_'|'a'..='z'|'A'..='Z'|'0'..='9') {
                temp.push(next);
                self.advance();
            } else {
                break;
            }
        }
        String::from_iter(temp.into_iter())
    }

    fn add_token(&mut self, typ: TokenType) {
        self.tokens.push(Token {
            offset: self.get_offset(),
            line: self.line,
            typ,
        })
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            ',' => self.add_token(TokenType::Comma),
            '\n' => {
                self.add_token(TokenType::EOL);
                self.advance_line_data();
            }
            '-' => {
                if self.consume_if_matches('>') {
                    self.add_token(TokenType::MacroStart);
                } else {
                    self.error(format!("unexpected '-'"));
                }
            }
            '<' => {
                if self.consume_if_matches('-') {
                    self.add_token(TokenType::MacroEnd);
                } else {
                    self.error(format!("unexpected '<'"));
                }
            }
            ';' => {
                while !self.is_at_end() && self.peek() != Some('\n') {
                    self.advance();
                }
            }
            '"' => self.consume_string(),
            ' ' | '\r' | '\t' => { /* Ignore spaces */ }
            '0'..='9' => {
                self.consume_number(c);
            }
            '_' | 'a'..='z' | 'A'..='Z' => {
                let token = self.consume_and_return_identifier(c);
                if self.peek() == Some(':') {
                    self.add_token(TokenType::Label(token));
                    self.advance();
                } else if asm::INSTRUCTIONS.contains(token.to_ascii_lowercase().as_str()) {
                    self.add_token(TokenType::Asm(token));
                } else if KEYWORDS.contains_key(token.to_ascii_lowercase().as_str()) {
                    let typ = KEYWORDS
                        .get(token.to_ascii_lowercase().as_str())
                        .unwrap()
                        .clone();
                    self.add_token(typ);
                } else {
                    self.add_token(TokenType::Identifier(token));
                }
            }
            other => {
                self.error(format!("unexpected character '{other}'"));
            }
        }
    }
}
