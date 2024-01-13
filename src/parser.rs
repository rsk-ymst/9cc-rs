use anyhow::{anyhow, Result};
use thiserror::Error;

pub struct Parser {
    code: String,
    cursor: usize,
}

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
}

impl Parser {
    pub fn new(code: String) -> Parser {
        Self { code, cursor: 0 }
    }

    pub fn parse(&mut self) -> anyhow::Result<()> {
        while self.cursor < self.code.len() {
            let target_char = self.code.as_bytes().get(self.cursor);

            if let Some(&target_char) = target_char {
                let _x: u8 = 1;

                if target_char.is_ascii() {
                    println!("{:?}", target_char as char);
                } else {
                    return Err(anyhow!("Length must be less than 10"));
                }

                self.cursor += 1;
            }
        }

        Ok(())
    }

    // pub fn atoi(&mut self) -> anyhow::Result<()> {
    //     // loop {
    //     //     self.co
    //     // }
    //     println!("{:?}", target_char as char);

    //     Ok(())
    // }
}
