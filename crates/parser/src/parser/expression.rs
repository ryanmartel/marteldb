use ast::name::Name;

use super::Parser;
use crate::errors::{ParseError, ParseErrorKind};
use crate::tokens::{TokenKind, TokenValue};

impl<'src> Parser<'src> {
    /// Parse an identifier.
    ///
    /// For invalid identifiers, the 'id' field will be an empty string.
    pub fn parse_identifier(&mut self) -> Result<ast::Identifier, ParseError> {
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
}
