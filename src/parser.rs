use std::{borrow::Borrow, iter::Peekable, slice::Iter, vec::IntoIter};

use crate::{tokenizer::{Operator, Token}, opx_node};
use anyhow::{anyhow, Result};
use thiserror::Error;

pub struct Parser {
    // code: String,
    // tokens: Vec<Token>,
    stream: Box<dyn Iterator<Item = Token>>,
    cursor: usize,
}

impl From<Vec<Token>> for Parser {
    fn from(tokens: Vec<Token>) -> Self {
        Self {
            stream: Box::new(tokens.into_iter()),
            cursor: 0
        }
    }
}

struct Node {
    token: Token,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl Node {
    pub fn new(token: Token, lhs: Option<Box<Node>>, rhs: Option<Box<Node>>) -> Self {
        Self { token, lhs, rhs }
    }
}

type OpxNode = Option<Box<Node>>;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let iterative = tokens.into_iter();
        Self {
            stream: Box::new(iterative),
            cursor: 0,
        }
    }

    pub fn parse(&mut self) {
        // let mut x = self.tokens.into_iter().peekable();

        let x = self.stream.next().unwrap();
        let node = self.mul();

        // while let Some(x) = x.next() {
        //     match x {
        //         Token::RESERVED(Operator::Lbr) => {
        //             self.expr();
        //         },
        //         _ => return
        //     }
        // }

        // while self.cursor < self.code.len() {
        //     let target_char = self.code.as_bytes().get(self.cursor);

        //     if let Some(&target_char) = target_char {
        //         let _x: u8 = 1;

        //         if target_char.is_ascii() {
        //             println!("{:?}", target_char as char);
        //         } else {
        //             return Err(anyhow!("Length must be less than 10"));
        //         }

        //         self.cursor += 1;
        //     }
        // }
    }

    pub fn expr(&mut self) -> OpxNode {
        let Node = self.mul();

        while let Some(token) = self.consume_token() {
            match token {
                Token::Add => {
                    return opx_node!(Token::Add, None, self.mul());
                }
                Token::Sub => {
                    return opx_node!(Token::Sub, None, self.mul());
                }
                _ => {
                    panic!("Unexpected token")
                }
            }
        }

        todo!()
    }

    pub fn mul(&mut self) -> OpxNode {
        let node = self.primary();

        while let Some(token) = self.consume_token() {
            match token {
                Token::Mul => {
                    return opx_node!(Token::Mul, node, None);
                }
                Token::Div => {
                    return opx_node!(Token::Div, node, None);
                }
                _ => {
                    panic!("Unexpected token")
                }
            }
        }

        // Add a default return value here
        todo!()
    }

    pub fn primary(&mut self) -> OpxNode {
        if let Some(token) = self.consume_token() {
            match token {
                Token::Lbr => {
                    todo!()
                }
                Token::Num(val) => {
                    return opx_node!(Token::Num(val), None, None);
                }
                _ => {
                    panic!("Unexpected token")
                }
            }
        }

        todo!()
    }

    pub fn consume_token(&mut self) -> Option<Token> {
        self.stream.next()
    }

    // pub fn atoi(&mut self) -> anyhow::Result<()> {
    //     // loop {
    //     //     self.co
    //     // }
    //     println!("{:?}", target_char as char);

    //     Ok(())
    // }
}

mod macros {
    use crate::parser::Node;

    #[macro_export]
    macro_rules! opx_node {
        // use crate::tokenizer::Token;
        ($token:expr, $lhs:expr, $rhs:expr) => {
            Some(Box::new(Node::new($token, $lhs, $rhs)))
        }
    }
}


mod tests {
    // use crate::tokenizer::Token;

    use crate::{tokenizer::*, opx_node};
    use crate::parser::Node;

    // use ::node;

    #[test]
    fn node_test() {
        let node = opx_node!(Token::Div, None, None);
        // let x: Vec<_> = vec![];
    }
}
