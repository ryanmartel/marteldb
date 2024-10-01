use std::f64;
use std::str::FromStr;

use ast::name::Name;
use source_index::{location::Location, span::Span};

use crate::{errors::{LexicalError, LexicalErrorKind}, tokens::{TokenKind, TokenValue}};

use self::cursor::Cursor;


mod cursor;

pub struct Lexer<'src> {

    source: &'src str,

    // Current index into the source string.
    cursor: Cursor<'src>,

    // current token being lexed
    current_kind: TokenKind,

    // current token value
    current_value: TokenValue,

    current_span: Span,

    // errors encountered
    errors: Vec<LexicalError>,
}

impl<'src> Lexer<'src> {

    pub(crate) fn new(source: &'src str) -> Self {
        Lexer {
            source,
            cursor: Cursor::new(source),
            current_kind: TokenKind::Unknown,
            current_value: TokenValue::None,
            current_span: Span::new(Location::new(0), Location::new(0)),
            errors: Vec::new()
        }
    }


    pub fn next_token(&mut self) -> TokenKind {
        self.cursor.start_token();
        self.current_value = TokenValue::None;
        self.current_kind = self.lex_token();

        self.current_kind
    }

    pub fn current_span(&self) -> Span {
        self.current_span
    }

    // Skips whitespace, checks that the char is in the ascii set,
    // and returns EndOfFile at end of stream
    fn lex_token(&mut self) -> TokenKind {
        self.skip_whitespace();
        // may have skipped whitespace, so move the cursor start
        self.cursor.start_token();

        if let Some(c) = self.cursor.bump() {
            // only support ascii text
            if c.is_ascii() {
                self.lex_ascii(c)
            } else {
                // TokenKind::Unknown
                self.push_error(LexicalError::new(
                        LexicalErrorKind::InvalidToken, self.token_range()))
            }
        } else {
            // reached end of file
            TokenKind::EndOfFIle
        }
        
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.cursor.first() {
                ' ' | '\n' | '\t' => self.cursor.bump(),
                _ => break,
            };
        }
    }

    // Push error onto error list and return TokenKind::Unknown
    fn push_error(&mut self, error: LexicalError) -> TokenKind {
        self.errors.push(error);
        TokenKind::Unknown
    }

    fn lex_ascii(&mut self, c: char) -> TokenKind {
        let token = match c {
            c if is_identifier_start(c) => self.lex_identifier_or_keyword(c),
            '0'..='9' => self.lex_number(c),
            '\'' => self.lex_string(),
            '-' => {
                if self.cursor.eat_char('-') {
                    return self.lex_comment()
                }
                TokenKind::Minus
            }
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            ';' => TokenKind::Semicolon,
            _ => self.push_error(LexicalError::new(
                    LexicalErrorKind::InvalidToken, self.token_range()
            ))
        };
        token
    }

    fn lex_identifier_or_keyword(&mut self, c: char) -> TokenKind {
        self.cursor.eat_while(|c| is_identifier_rest(c));
        // Handle table qualified Identifier. Let parser figure this out.
        if self.cursor.eat_char('.') {
            // table.*
            if self.cursor.eat_char('*') {
            } else {
                // table.column
                self.cursor.eat_if(|c| is_identifier_start(c));
                self.cursor.eat_while(|c| is_identifier_rest(c));
            }
        }
        let text = self.token_text();
        match text.to_uppercase().as_str() {
            "SELECT" => TokenKind::Select,
            _ => {
                self.current_value = TokenValue::Name(Name::new(text.to_string()));
                TokenKind::Name

            }
        }
    }

    fn lex_number(&mut self, c: char) -> TokenKind {
        let mut owned = String::new();
        let mut is_float = false;

        // float with leading 0 (0.xxxx)
        if matches!(c, '0') {
            is_float = true;
            owned.push(c);
            if !self.cursor.eat_char('.') {
                return self.push_error(LexicalError::new(
                        LexicalErrorKind::InvalidNumber, self.token_range()
                ));
            }
            owned.push('.');
            while let Some(digit) = self.cursor.eat_if(|c| is_digit(c)) {
                owned.push(digit);
            }
        } else {
            owned.push(c);
            while let Some(digit) = self.cursor.eat_if(|c| is_digit(c)) {
                owned.push(digit);
            }
            // check for float point
            if self.cursor.eat_char('.') {
                is_float = true;
                owned.push('.');
                // add digits after point
                while let Some(digit) = self.cursor.eat_if(|c| is_digit(c)) {
                    owned.push(digit);
                }
            }
        }
        if is_float {
            let Ok(val) = f64::from_str(&owned) else {
                return self.push_error(LexicalError::new(
                        LexicalErrorKind::InvalidFloat, self.token_range()
                ));
            };
            self.current_value = TokenValue::Float(val);
            return TokenKind::Float;
        }
        let Ok(val) = i32::from_str(&owned) else {
            return self.push_error(LexicalError::new(
                    LexicalErrorKind::InvalidInt, self.token_range()
            ));
        };
        self.current_value = TokenValue::Int(val);
        TokenKind::Int
    }

    fn lex_comment(&mut self) -> TokenKind {
        self.cursor.eat_while(|c| !matches!(c, '\n' | '\r'));
        TokenKind::Comment
    } 

    fn lex_string(&mut self) -> TokenKind {
        let mut num_backslashes = 0;
        loop {
            match self.cursor.bump() {
                Some(c) => {
                    match c {
                        '\\' => num_backslashes += 1,
                        '\'' => {
                            if num_backslashes%2 == 0 {
                                break;
                            }
                            num_backslashes = 0
                        }
                        _ => num_backslashes = 0,
                        
                    }
                }
                None => return self.push_error(LexicalError::new(
                        LexicalErrorKind::UnterminatedString, self.token_range()
                ))
            }
        }
        let text = self.token_text();
        self.current_value = TokenValue::String(text.to_string());
        TokenKind::String
    }

    fn token_text(&self) -> &'src str {
        &self.source[self.token_range()]
    }

    fn token_range(&self) -> Span {
        let end = self.offset();
        let len = self.cursor.token_length();

        Span::new(end - len, len)
    }

    fn offset(&self) -> Location {
        Location::new(self.source.len()) - self.cursor.text_len()
    }

    // Create a checkpoint of the lexer
    pub fn checkpoint(&self) -> LexerCheckpoint {
        LexerCheckpoint {
            value: self.current_value.clone(),
            current_kind: self.current_kind,
            current_span: self.current_span,
            cursor_offset: self.offset(),
            errors_position: self.errors.len()
        }
    }

    // Restore lexer to given checkpoint.
    pub fn rewind(&mut self, checkpoint: LexerCheckpoint) {
        let LexerCheckpoint {
            value,
            current_kind,
            current_span,
            cursor_offset,
            errors_position,
        } = checkpoint;
        
        let mut cursor = Cursor::new(self.source);
        cursor.seek_forward(*cursor_offset);

        self.current_value = value;
        self.current_kind = current_kind;
        self.current_span = current_span;
        self.cursor = cursor;
        self.errors.truncate(errors_position);
    }
}

pub struct LexerCheckpoint {
    value: TokenValue,
    current_kind: TokenKind,
    current_span: Span,
    cursor_offset: Location,
    errors_position: usize,

}

fn is_identifier_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_identifier_rest(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | '-')
}

fn is_digit(c: char) -> bool {
    matches!(c, '0'..='9')
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn multiple_single_char_tokens() {
        let source = "()";
        let mut lexer = Lexer::new(source);
        assert!(matches!(lexer.next_token(), TokenKind::LParen));
        assert!(matches!(lexer.next_token(), TokenKind::RParen));
    }

    #[test]
    fn skips_whitespace() {
        let source = "() ;  ;
        ;";
        let mut lexer = Lexer::new(source);
        assert!(matches!(lexer.next_token(), TokenKind::LParen));
        assert!(matches!(lexer.next_token(), TokenKind::RParen));
        assert!(matches!(lexer.next_token(), TokenKind::Semicolon));
        assert!(matches!(lexer.next_token(), TokenKind::Semicolon));
        assert!(matches!(lexer.next_token(), TokenKind::Semicolon));
    }

    #[test]
    fn simple_keyword() {
        let source = "SELECT";
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::Select),
            "Did not get SELECT token. received {token}");
    }

    #[test]
    fn lowercase_keyword() {
        let source = "select";
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::Select),
            "Did not get Select token. received {token}");
    }

    #[test]
    fn simple_identifier() {
        let source = "col";
        let _owned = String::from(source);
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::Name),
            "Did not get Name token. received {token}");
        assert!(matches!(lexer.current_value.clone(), TokenValue::Name(_owned)),
            "Did not get the Name col, got {:?}", &lexer.current_value);
    }

    #[test]
    fn table_qualified_identifiers() {
        let source = "table.col";
        let _expected = TokenValue::Name(Name::new(source.to_string()));
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::Name),
            "Did not get Name token. Received {token}");
        assert_eq!(lexer.current_value.clone(), _expected,
            "Did not get the Name table.col, got {:?}", &lexer.current_value);
        let source = "table.*";
        let _expected = TokenValue::Name(Name::new(source.to_string()));
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::Name),
            "Did not get Name token. Received {token}");
        assert_eq!(lexer.current_value.clone(), _expected,
            "Did not get the Name table.*, got {:?}", &lexer.current_value);
    }

    #[test]
    fn simple_numbers() {
        let source = "17 0.94 8.57 98.99";
        let mut lexer = Lexer::new(source);
        let t1 = lexer.next_token();
        assert!(matches!(t1, TokenKind::Int));
        assert!(matches!(lexer.current_value, TokenValue::Int(17)),
            "Did not get right value. got {:?} expected 17", lexer.current_value);
        let t2 = lexer.next_token();
        assert!(matches!(t2, TokenKind::Float));
        assert!(matches!(lexer.current_value, TokenValue::Float(0.94)),
            "Did not get right value. got {:?} expected 0.94", lexer.current_value);
        let t3 = lexer.next_token();
        assert!(matches!(t3, TokenKind::Float));
        assert!(matches!(lexer.current_value, TokenValue::Float(8.57)),
            "Did not get right value. got {:?} expected 8.57", lexer.current_value);
        let t4 = lexer.next_token();
        assert!(matches!(t4, TokenKind::Float));
        assert!(matches!(lexer.current_value, TokenValue::Float(98.99)),
            "Did not get right value. got {:?} expected 98.99", lexer.current_value);


    }

    #[test]
    fn comment_token() {
        let source = "SELECT 
            -- This is a comment
            ;";
        let mut lexer = Lexer::new(source);
        let _select = lexer.next_token();
        let comment = lexer.next_token();
        assert!(matches!(comment, TokenKind::Comment),
            "Comment token did not match. got {comment}");
        let semicolon = lexer.next_token();
        assert!(matches!(semicolon, TokenKind::Semicolon),
            "Lost token following comment. expected semicolon, got {semicolon}");
    }

    #[test]
    fn string_literal() {
        let source = "'string lit'";
        let _expected = TokenValue::String(String::from(source));
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::String),
            "String token did not match. Got {token}");
        assert_eq!(_expected, lexer.current_value,
            "Did not get 'string lit', got {:?}", lexer.current_value);
    }

    #[test]
    fn unterminated_string() {
        let source = "'this is a string";
        let _expected = LexicalErrorKind::UnterminatedString;
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::Unknown),
            "String token did not match. Got {token}");
        let e = lexer.errors.first().unwrap();
        assert_eq!(_expected, e.error_kind(),
            "Did not get UnterminatedString, got {:?}", e.error_kind());
    }

    #[test]
    fn escaped_string_terminator() {
        let source = "'string\\'s are lit'";
        let _expected = TokenValue::String(String::from(source));
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::String),
            "String token did not match. Got {token}");
        assert_eq!(_expected, lexer.current_value,
            "Did not get 'string lit', got {:?}", lexer.current_value);
    }

    #[test]
    fn double_bkslh_string() {
        let source = "'string\\\\'";
        let _expected = TokenValue::String(String::from(source));
        let mut lexer = Lexer::new(source);
        let token = lexer.next_token();
        assert!(matches!(token, TokenKind::String),
            "String token did not match. Got {token}");
        assert_eq!(_expected, lexer.current_value,
            "Did not get 'string lit', got {:?}", lexer.current_value);

    }
}
