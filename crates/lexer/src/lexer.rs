// use logos::{Logos};
//
// use super::token::{LexicalError, Token};
//
//
// pub struct Lexer<'input> {
//     token_stream: Iter<'input, Token>,
// }
//
// impl<'input> Lexer<'input> {
//     pub fn new(input: &'input str) -> Self {
//         // Token::lexer() provided by Logos trait
//         Self {
//             token_stream: Token::lexer(input).spanned(),
//         }
//     }
// }
//
// impl<'input> Iterator for Lexer<'input> {
//     type Item = Spanned<Token, usize, LexicalError>;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.token_stream
//             .next()
//             .map(|(token, span)| Ok((span.start, token?, span.end)))
//     }
// }
