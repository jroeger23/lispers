use super::token::Token;
use super::tokenizer::tokenize;
use super::tokenizer::TokenStream;
use super::tokenizer::TokenizerError;
use crate::lisp::Expression;
use std::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken(Token),
    TokenizerError(TokenizerError),
    UnexpectedEndOfInput,
}

impl From<TokenizerError> for ParserError {
    fn from(value: TokenizerError) -> Self {
        ParserError::TokenizerError(value)
    }
}

fn parse_list<I>(stream: &mut Peekable<TokenStream<I>>) -> Result<Expression, ParserError>
where
    I: Iterator<Item = char>,
{
    let mut list = Vec::new();

    loop {
        match stream.peek() {
            // Return current list or nil
            Some(Ok(Token::ParClose)) => {
                stream.next();
                if list.len() == 0 {
                    return Ok(Expression::Nil);
                } else {
                    return Ok(list.into());
                }
            }
            // Switch to cons-pair parsing
            Some(Ok(Token::Dot)) => {
                stream.next();
                if list.len() > 1 || list.len() == 0 {
                    return Err(ParserError::UnexpectedToken(Token::Dot));
                } else {
                    let second_expr = parse_expression(stream)?;
                    match stream.next() {
                        Some(Ok(Token::ParClose)) => {
                            return Ok(Expression::Cell(
                                Box::new(list[0].to_owned()),
                                Box::new(second_expr),
                            ));
                        }
                        Some(Ok(t)) => {
                            return Err(ParserError::UnexpectedToken(t));
                        }
                        Some(Err(e)) => {
                            return Err(e.into());
                        }
                        None => {
                            return Err(ParserError::UnexpectedEndOfInput);
                        }
                    }
                }
            }
            _ => {}
        }
        list.push(parse_expression(stream)?);
    }
}

fn parse_expression<I>(stream: &mut Peekable<TokenStream<I>>) -> Result<Expression, ParserError>
where
    I: Iterator<Item = char>,
{
    match stream.next() {
        Some(Ok(Token::ParOpen)) => parse_list(stream),
        Some(Ok(Token::Nil)) => Ok(Expression::Nil),
        Some(Ok(Token::IntLiteral(n))) => Ok(Expression::Integer(n)),
        Some(Ok(Token::FloatLiteral(f))) => Ok(Expression::Float(f)),
        Some(Ok(Token::StringLiteral(s))) => Ok(Expression::String(s)),
        Some(Ok(Token::True)) => Ok(Expression::True),
        Some(Ok(Token::Symbol(s))) => Ok(Expression::Symbol(s)),
        Some(Ok(Token::Quote)) => Ok(Expression::Quote(Box::new(parse_expression(stream)?))),
        Some(Err(e)) => Err(ParserError::TokenizerError(e)),
        Some(Ok(x)) => Err(ParserError::UnexpectedToken(x)),
        None => Err(ParserError::UnexpectedEndOfInput),
    }
}

pub struct ExpressionStream<I: Iterator<Item = char>> {
    token_stream: Peekable<TokenStream<I>>,
}

impl<I: Iterator<Item = char>> ExpressionStream<I> {
    pub fn from_token_stream(token_stream: TokenStream<I>) -> Self {
        ExpressionStream {
            token_stream: token_stream.peekable(),
        }
    }

    pub fn from_char_stream(char_stream: I) -> Self {
        ExpressionStream {
            token_stream: tokenize(char_stream).peekable(),
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for ExpressionStream<I> {
    type Item = Result<Expression, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.token_stream.peek() == None {
            return None;
        }

        Some(parse_expression(&mut self.token_stream))
    }
}

#[test]
fn test_parser() {
    let input = "(1 2 3) (4 5 6) (1 . 2) (1 . (2 . (3))) \"test\" '(a b c true nil)";
    let ts = tokenize(input.chars());
    let es = ExpressionStream::from_token_stream(ts);
    let exprs = es.collect::<Result<Vec<Expression>, ParserError>>();
    assert_eq!(
        exprs,
        Ok(vec![
            vec![
                Expression::Integer(1),
                Expression::Integer(2),
                Expression::Integer(3),
            ]
            .into(),
            vec![
                Expression::Integer(4),
                Expression::Integer(5),
                Expression::Integer(6),
            ]
            .into(),
            Expression::Cell(
                Box::new(Expression::Integer(1)),
                Box::new(Expression::Integer(2)),
            ),
            vec![
                Expression::Integer(1),
                Expression::Integer(2),
                Expression::Integer(3),
            ]
            .into(),
            Expression::String("test".to_string()),
            Expression::Quote(Box::new(
                vec![
                    Expression::Symbol("a".to_string()),
                    Expression::Symbol("b".to_string()),
                    Expression::Symbol("c".to_string()),
                    Expression::True,
                    Expression::Nil,
                ]
                .into()
            )),
        ])
    );
}
