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

pub struct Parser2<T: Iterator<Item = Token>>
{
    // code: String,
    // tokens: Vec<Token>,
    stream: T,
    cursor: usize,
}

impl From<Vec<Token>> for Parser {
    fn from(tokens: Vec<Token>) -> Self {
        Self {
            stream: Box::new(tokens.into_iter().peekable()),
            cursor: 0
        }
    }
}

#[derive(Debug)]
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

    pub fn parse(&mut self) -> OpxNode {
        // let mut x = self.tokens.into_iter().peekable();

        // let x = self.stream.next().unwrap();
        // let node = self.mul();

        self.expr()

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
        let mut node = self.mul();
        println!("prim {:?}", node);


        while let Some(token) = self.consume_token() {
            match token {
                Token::Add => {
                    println!("Add!");
                    node = opx_node!(Token::Add, node, self.mul());
                }
                Token::Sub => {
                    println!("Sub!");
                    node = opx_node!(Token::Sub, node, self.mul());
                }
                _ => {
                    return node;
                }
            }
        }

        None
    }

    pub fn mul(&mut self) -> OpxNode {
        let mut node = self.primary();
        println!("mul {:?}", node);

        while let Some(token) = self.consume_token() {
            match token {
                Token::Mul => {
                    node = opx_node!(Token::Mul, node, self.primary());
                }
                Token::Div => {
                    node = opx_node!(Token::Div, node, self.primary());
                }
                _ => {
                    println!("{token:?}");
                    return node;
                }
            }
        }

        // Add a default return value here
        node
    }

    pub fn primary(&mut self) -> OpxNode {
        println!("primary!");

        if let Some(token) = self.consume_token() {
            println!("{token:?}");
            match token {
                Token::Lbr => {
                    return self.expr();
                }
                _ => {
                    return opx_node!(token, None, None);
                }
            }
        }

        None
    }

    pub fn consume_token(&mut self) -> Option<Token> {
        self.stream.next()
    }

    //pub fn peek_token(&self) -> Option<&Token> {
     //   let x = self.stream.peekable();
     //   let x = self.stream.peekable();
     //   x.peek;
    //}

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
    use crate::parser::{Node, Parser};

    // use ::node;

    #[test]
    fn node_test() {
        let node = opx_node!(Token::Div, None, None);
        // let x: Vec<_> = vec![];
    }

    #[test]
    fn tokenize_and_parse() {
        let input = "3 + 4 * (2 - 1)";
        let tokenizer = Tokenizer::new(input.to_owned());

        let tokens = tokenizer.tokenize();
        println!("{tokens:?}");

        let mut parser = Parser::from(tokens);
        let out = parser.expr();

        println!("{:?}", out);
    }
}
