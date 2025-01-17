use ast::name::Name;

use super::Parser;
use crate::errors::{ParseError, ParseErrorKind};
use crate::tokens::{TokenKind, TokenValue};

impl<'src> Parser<'src> {
    /// Parse an identifier.
    ///
    /// For invalid identifiers, the 'id' field will be an empty string.
    pub(crate) fn parse_identifier(&mut self) -> Result<ast::Identifier, ParseError> {
        let start = self.node_start();

        if self.at(TokenKind::Name) {
            let TokenValue::Name(name) = self.bump_value(TokenKind::Name) else {
                unreachable!();
            };
            Ok(ast::Identifier {id: name, span: self.node_span(start)})
        } else {
            // self.add_error(
            //     ParseErrorKind::ExpectedIdentifier {
            //         found: self.current_token_kind(),
            //     },
            //     span,
            // );
            Err(ParseError {
                kind: ParseErrorKind::ExpectedIdentifier { found: self.current_token_kind() },
                span: self.node_span(start),
            })
        }
    }

    pub(crate) fn parse_signed_number(&mut self) -> Result<ast::SignedNumber, ParseError> {
        let start = self.node_start();
        let mut negation = 1;

        self.eat(TokenKind::Plus);
        if self.eat(TokenKind::Minus) {
            negation = -1;
        }
        match self.current_token_kind() {
            TokenKind::Int => {
                let value = self.bump_value(TokenKind::Int);
                if let TokenValue::Int(inner_value) = value {
                    return Ok(ast::SignedNumber::Integer(ast::IntLiteral {
                        span: self.node_span(start),
                        value: negation * inner_value
                    }));
                }
                return Err(ParseError {
                    span: self.node_span(start),
                    kind: ParseErrorKind::ExpectedValue
                });


            }
            TokenKind::Float => {
                let value = self.bump_value(TokenKind::Float);
                if let TokenValue::Float(inner_value) = value {
                    return Ok(ast::SignedNumber::Float(ast::FloatLiteral {
                        span: self.node_span(start),
                        value: negation as f64 * inner_value
                    }));
                }
                return Err(ParseError {
                    span: self.node_span(start),
                    kind: ParseErrorKind::ExpectedValue
                });
            }
            _ => {
                return Err(ParseError {
                    span: self.node_span(start),
                    kind: ParseErrorKind::ExpectedNumeric {
                        found: self.current_token_kind()
                    }
                })
            }
        }
    }
}
