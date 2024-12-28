use ast::name::Name;

use super::Parser;
use crate::tokens::{TokenKind, TokenValue};
use crate::errors::ParseErrorKind;

impl<'src> Parser<'src> {

    /// Parse an identifier.
    ///
    /// For invalid identifiers, the 'id' field will be an empty string.
    pub fn parse_identifier(&mut self) -> ast::Identifier {
        let span = self.current_token_span();

        if self.at(TokenKind::Name) {
            let TokenValue::Name(name) = self.bump_value(TokenKind::Name) else {
                unreachable!();
            };
            return ast::Identifier {id: name, span};
        } else {
            self.add_error(
                ParseErrorKind::ExpectedIdentifier { found: self.current_token_kind() },
                span
            );
            
            ast::Identifier {
                id: Name::empty(),
                span,
            }
        }
    }

}
