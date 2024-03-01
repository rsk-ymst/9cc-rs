use std::{borrow::Borrow, iter::Peekable, slice::Iter, vec::IntoIter};

use crate::{opx_node, tokenizer::Token};
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

    // expr = equality
    pub fn expr(&mut self) -> OpxNode {
        // let mut node = self.mul();
        // println!("expr {:?}", node);

        // while let Some(token) = self.peek_token() {
        //     println!("expr: {token:?}");

        //     match token {
        //         Token::Add => {
        //             println!("Add!");
        //             self.consume_token();
        //             node = opx_node!(Token::Add, node, self.mul());
        //         }
        //         Token::Sub => {
        //             println!("Sub!");
        //             self.consume_token();
        //             node = opx_node!(Token::Sub, node, self.mul());
        //         }
        //         _ => {
        //             return node;
        //         }
        //     }
        // }

        // node

        self.equality()
    }

    // equality = relational ("==" relational | "!=" relational)*
    pub fn equality(&mut self) -> OpxNode {
        let mut node = self.relational();

        while let Some(token) = self.peek_token() {
            match token {
                Token::EQ | Token::NE => {
                    let tk = self.consume_token().unwrap();
                    node = opx_node!(tk, node, self.relational());
                }
                _ => {
                    return node;
                }
            }
        }

        node
    }

    // relational = add ("<" add | "<=" add | ">" add | ">=" add)*
    pub fn relational(&mut self) -> OpxNode {
        let mut node = self.add();

        while let Some(token) = self.peek_token() {
            match token {
                Token::LT | Token::LE | Token::GT | Token::GE => {
                    let tk = self.consume_token().unwrap();
                    node = opx_node!(tk, node, self.add());
                }
                _ => {
                    return node;
                }
            }
        }

        node
    }

    // add = mul ("+" mul | "-" mul)*
    pub fn add(&mut self) -> OpxNode {
        let mut node = self.mul();

        while let Some(token) = self.peek_token() {
            // println!("mul: {token:?}");

            match token {
                Token::Add => {
                    self.consume_token();
                    node = opx_node!(Token::Add, node, self.mul());
                }
                Token::Sub => {
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

    // add = mul ("+" mul | "-" mul)*
    fn mul(&mut self) -> OpxNode {
        let mut node = self.unary();

        while let Some(token) = self.peek_token() {
            match token {
                Token::Mul => {
                    self.consume_token();
                    node = opx_node!(Token::Mul, node, self.unary());
                }
                Token::Div => {
                    self.consume_token();
                    node = opx_node!(Token::Div, node, self.unary());
                }
                _ => {
                    // println!("{token:?}");
                    return node;
                }
            }
        }

        node
    }

    // unary = ("+" | "-")? unary | primary
    fn unary(&mut self) -> OpxNode {
        if let Some(token) = self.peek_token() {
            match token {
                Token::Add => {
                    self.consume_token();
                    return self.unary();
                }
                Token::Sub => {
                    self.consume_token();
                    return opx_node!(
                        Token::Sub,
                        opx_node!(Token::Num(0), None, None),
                        self.unary()
                    );
                }
                _ => {
                    return self.primary();
                }
            }
        }

        self.primary()
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
                return true;
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
        };
    }
}

mod tests {

    use crate::parser::{Node, Parser};
    use crate::{opx_node, tokenizer::*};

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
