use crate::errors::ParseError;
use crate::tokens::Token;
use ast::Stmts;
use parser::Parser;
pub mod parser;

mod errors;
mod lexer;
mod token_source;
mod tokens;

pub fn parse_stmts(source: &str) -> Result<Parsed, ParseError> {
    Parser::new(source).parse().into_result()
}

/// Represents the parsed source code.
#[derive(Debug, PartialEq, Clone)]
pub struct Parsed {
    pub stmts: Stmts,
    tokens: Tokens,
    errors: Vec<ParseError>,
}

impl Parsed {
    /// Returns all the tokens for the parsed output.
    pub fn tokens(&self) -> &Tokens {
        &self.tokens
    }

    /// Returns a list of syntax errors found during parsing.
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Consumes the [`Parsed`] output and returns a list of syntax errors found during parsing.
    pub fn into_errors(self) -> Vec<ParseError> {
        self.errors
    }

    /// Returns `true` if the parsed source code is valid i.e., it has no syntax errors.
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    fn into_result(self) -> Result<Parsed, ParseError> {
        if self.is_valid() {
            Ok(self)
        } else {
            Err(self.into_errors().into_iter().next().unwrap())
        }
    }
}

/// Tokens represents a vector of lexed [`Token`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tokens {
    raw: Vec<Token>,
}

impl Tokens {
    pub fn new(raw: Vec<Token>) -> Self {
        Tokens { raw }
    }
}
