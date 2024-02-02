use std::{borrow::Borrow, iter::Peekable, slice::Iter, vec::IntoIter};

use crate::{tokenizer::{Operator, Token}, opx_node};
use anyhow::{anyhow, Result};
use thiserror::Error;

pub struct Parser<I: Iterator> {
    stream: Peekable<I>,
    cursor: usize,
}

impl From<Vec<Token>> for Parser<IntoIter<Token>> {
    fn from(tokens: Vec<Token>) -> Self {
        Self {
            stream: tokens.into_iter().peekable(),
            cursor: 0,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Node {
    pub token: Token,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}

impl Node {
    pub fn new(token: Token, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Self {
        Self { token, lhs, rhs }
    }
}

pub type OpxNode = Option<Box<Node>>;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },

    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
}

impl Parser<IntoIter<Token>> {

    pub fn parse(&mut self) -> OpxNode {
        self.expr()
    }

    pub fn expr(&mut self) -> OpxNode {
        let mut node = self.mul();
        println!("expr {:?}", node);


        while let Some(token) = self.peek_token() {
            println!("expr: {token:?}");

            match token {
                Token::Add => {
                    println!("Add!");
                    self.consume_token();
                    node = opx_node!(Token::Add, node, self.mul());
                }
                Token::Sub => {
                    println!("Sub!");
                    self.consume_token();
                    node = opx_node!(Token::Sub, node, self.mul());
                }
                _ => {
                    return node;
                }
            }
        }

        node
    }

    fn mul(&mut self) -> OpxNode {
        let mut node = self.primary();
        println!("mul {:?}", node);

        while let Some(token) = self.peek_token() {
            println!("mul: {token:?}");

            match token {
                Token::Mul => {
                    self.consume_token();
                    node = opx_node!(Token::Mul, node, self.primary());
                }
                Token::Div => {
                    self.consume_token();
                    node = opx_node!(Token::Div, node, self.primary());
                }
                _ => {
                    println!("{token:?}");
                    return node;
                }
            }
        }

        node
    }

    fn primary(&mut self) -> OpxNode {
        println!("primary!");

        if let Some(token) = self.peek_token() {
            println!("prim: {token:?}");
            match token {
                Token::Lbr => {
                    self.consume_token();
                    let node = self.expr();
                    self.expect_token(Token::Rbr);

                    return node;
                }
                _ => {
                    let token = self.consume_token().unwrap();
                    return opx_node!(token, None, None);
                }
            }
        }

        None
    }

    fn peek_token(&mut self) -> Option<&Token> {
        self.stream.peek()
    }

    fn consume_token(&mut self) -> Option<Token> {
        self.stream.next()
    }

    fn expect_token(&mut self, token: Token) -> bool {
        if let Some(x) = self.peek_token() {
            if x == &token {
                self.consume_token();
                return true
            }
        }

        false
    }
}

mod macros {
    use crate::parser::Node;

    #[macro_export]
    macro_rules! opx_node {
        ($token:expr, $lhs:expr, $rhs:expr) => {
            Some(Box::new(Node::new($token, $lhs, $rhs)))
        }
    }
}


mod tests {

    use crate::{tokenizer::*, opx_node};
    use crate::parser::{Node, Parser};

    #[test]
    fn node_test() {
        let node = opx_node!(Token::Div, None, None);
    }

    #[test]
    fn tokenize_and_parse() {
        let input = "2*3+4*5";
        let tokenizer = Tokenizer::new(input.to_owned());

        let tokens = tokenizer.tokenize();
        println!("{tokens:?}");

        let mut parser = Parser::from(tokens);
        let out = parser.expr();

        println!("{:#?}", out);
    }
}
