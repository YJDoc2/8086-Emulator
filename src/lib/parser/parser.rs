use super::{
    ir::IR,
    lexer::{Token, TokenType},
};
use std::{collections::HashMap, fmt::Display};

#[derive(Default)]
pub(super) struct ParserContext {
    pub(super) macros: HashMap<String, MacroDef>,
}

#[derive(Debug)]
pub(super) struct MacroDef {
    pub(super) name: Token,
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

    pub fn parse(&mut self) -> Result<Vec<IR>, Vec<ParsingError>> {
        let mut errs = Vec::new();
        let mut ir = Vec::new();
        while !self.is_at_end() {
            match self.handle_parse() {
                Ok(v) => {
                    ir.extend(v.into_iter());
                }
                Err(e) => {
                    errs.push(e);
                    self.sync();
                }
            }
        }
        if !errs.is_empty() {
            Err(errs)
        } else {
            Ok(ir)
        }
    }

    fn handle_parse(&mut self) -> Result<Vec<IR>, ParsingError> {
        match self.peek() {
            TokenType::Macro => {
                self.macro_def()?;
                Ok(Vec::new())
            }
            TokenType::EOF | TokenType::EOL => {
                self.current += 1;
                Ok(Vec::new())
            }
            TokenType::Identifier => self.macro_use(),
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
        let macro_name_token = self.consume_if(TokenType::Identifier)?;
        let macro_name = macro_name_token.value.string();
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

        let param_set: std::collections::HashSet<String> =
            params.iter().map(|p| p.value.string()).collect();

        for id in code.iter().filter(|t| t.typ == TokenType::Identifier) {
            if !param_set.contains(&id.value.string()) {
                return Err(ParsingError::new(
                    id.clone(),
                    format!(
                        "non-parameter identifier {} found in macro body",
                        id.value.string()
                    ),
                ));
            }
        }

        let m = MacroDef {
            name: macro_name_token,
            params: params.into_iter().map(|p| p.value.string()).collect(),
            code,
        };
        self.ctx.macros.insert(macro_name, m);
        Ok(())
    }

    fn macro_use(&mut self) -> Result<Vec<IR>, ParsingError> {
        // we know tha tit is a macro use, so can simply unwrap
        let macro_name_token = self.consume_if(TokenType::Identifier).unwrap();
        let macro_name = macro_name_token.value.string();

        self.consume_if(TokenType::LeftParen)?;
        // params
        let mut params = Vec::new();
        while !self.check(&TokenType::RightParen) {
            let mut tokens = Vec::new();
            while !self.check(&TokenType::Comma) && !self.check(&TokenType::RightParen) {
                tokens.push(self.consume());
            }
            let _ = self.consume_if(TokenType::Comma); // can possibly be left paran, so we ignore result
            params.push(tokens);
        }
        self.consume_if(TokenType::RightParen)?;

        let macro_def = match self.ctx.macros.get(&macro_name) {
            Some(m) => m,
            None => {
                return Err(ParsingError::new(
                    macro_name_token,
                    format!("no macro with name {} defined", macro_name),
                ));
            }
        };

        if params.len() != macro_def.params.len() {
            return Err(ParsingError::new(
                macro_name_token,
                format!(
                    "parameter count mismatch : expected {} as per definition, found {} instead",
                    macro_def.params.len(),
                    params.len()
                ),
            ));
        }

        let mut expanded_tokens: Vec<Token> = Vec::new();
        for token in macro_def.code.clone() {
            if token.typ == TokenType::Identifier {
                let id_name = token.value.string();
                if let Some(idx) = macro_def.params.iter().position(|p| **p == id_name) {
                    for t in &params[idx] {
                        expanded_tokens.push(t.clone());
                    }
                }
            } else {
                expanded_tokens.push(token);
            }
        }

        let mut parser = Self::new(expanded_tokens);
        match parser.parse() {
            Err(e) => {
                return Err(ParsingError::new(
                    macro_name_token,
                    format!("Error in expanding macro : {:?}", e),
                ));
            }
            Ok(v) => Ok(v),
        }
    }
}
