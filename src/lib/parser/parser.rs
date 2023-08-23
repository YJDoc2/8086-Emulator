use super::lexer::{Token, TokenType};
use std::fmt::Display;

type ParseResult = Result<(), Vec<ParsingError>>;

#[derive(Default)]
pub(super) struct ParserContext {
    pub(super) macros: Vec<MacroDef>,
}

#[derive(Debug)]
pub(super) struct MacroDef {
    pub(super) name: String,
    pub(super) params: Vec<String>,
    pub(super) code: Vec<Token>,
}

pub struct Parser {
    pub(super) ctx: ParserContext,
    source: Vec<Token>,
    src_length: usize,
    current: usize,
    start: usize,
}

pub struct ParsingError {
    token: Token,
    error: String,
}

// TODO improve these
impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "token {:?} {}", self.token, self.error)
    }
}

impl std::fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "token {:?} {}", self.token, self.error)
    }
}

impl ParsingError {
    pub fn new<T: Into<String>>(token: Token, err: T) -> Self {
        Self {
            token,
            error: err.into(),
        }
    }
}

impl Parser {
    pub fn new(source: Vec<Token>) -> Self {
        let src_length = source.len();
        Self {
            ctx: ParserContext::default(),
            source,
            src_length,
            current: 0,
            start: 0,
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut errs = Vec::new();
        while !self.is_at_end() {
            match self.handle_parse() {
                Ok(_) => {}
                Err(e) => {
                    errs.push(e);
                    self.sync();
                }
            }
        }
        if !errs.is_empty() {
            Err(errs)
        } else {
            Ok(())
        }
    }

    fn handle_parse(&mut self) -> Result<(), ParsingError> {
        match self.peek() {
            TokenType::Macro => self.macro_def(),
            TokenType::EOF | TokenType::EOL => {
                self.current += 1;
                Ok(())
            }
            other => Err(ParsingError::new(
                self.source[self.current].clone(),
                format!(
                    "unexpected token {:?}, expected one of : macro,eol,eof",
                    other
                ),
            )),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src_length
    }

    fn peek(&self) -> &TokenType {
        if !self.is_at_end() {
            &self.source[self.current].typ
        } else {
            &TokenType::EOF
        }
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek() == t
    }

    fn sync(&mut self) {
        // we can be almost always sure that peek will return OK with at worst EOF here
        while !self.is_at_end() && !matches!(self.peek(), TokenType::EOL | TokenType::EOF) {
            self.current += 1;
        }
        self.current += 1; // eol or eof token
    }

    fn consume(&mut self) -> Token {
        let ret = &self.source[self.current];
        self.current += 1;
        ret.clone()
    }

    // maybe optimize later by adding skip and skip if methods which don't clone and return tokens
    fn consume_if(&mut self, t: TokenType) -> Result<Token, ParsingError> {
        // there will always be at most 1 EOL because we coalesce them in lexer
        if *self.peek() == TokenType::EOL {
            self.current += 1;
        }
        if self.check(&t) {
            let ret = self.consume();
            return Ok(ret);
        } else {
            let crr = &self.source[self.current];
            return Err(ParsingError::new(
                crr.clone(),
                format!("expected {:?}, but found {:?} instead", t, crr.typ),
            ));
        }
    }

    fn macro_def(&mut self) -> Result<(), ParsingError> {
        self.consume_if(TokenType::Macro)?;
        let macro_name = self.consume_if(TokenType::Identifier)?;
        self.consume_if(TokenType::LeftParen)?;

        // identifiers
        let mut params = Vec::new();

        if *self.peek() != TokenType::RightParen && *self.peek() != TokenType::EOF {
            let id = self.consume_if(TokenType::Identifier)?;
            params.push(id);
        }

        while *self.peek() != TokenType::RightParen && *self.peek() != TokenType::EOF {
            self.consume_if(TokenType::Comma)?;
            let id = self.consume_if(TokenType::Identifier)?;
            params.push(id);
        }
        self.consume_if(TokenType::RightParen)?;

        self.consume_if(TokenType::MacroStart)?;

        // macro body
        let mut code = Vec::new();
        while *self.peek() != TokenType::MacroEnd && *self.peek() != TokenType::EOF {
            let t = self.consume();
            code.push(t);
        }
        self.consume_if(TokenType::MacroEnd)?;
        let m = MacroDef {
            name: macro_name.value.string(),
            params: params.into_iter().map(|p| p.value.string()).collect(),
            code,
        };
        println!("{:?}", m);
        self.ctx.macros.push(m);
        Ok(())
    }
}
