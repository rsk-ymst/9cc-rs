use std::vec;


use crate::parser::{Node, OpxNode, Parser};
use crate::tokenizer::{Token, Tokenizer};
use crate::utils;


pub struct AsmGenerator {
    // node: OpxNode,
    asm_line: Vec<String>
}

impl AsmGenerator {
    fn new() -> Self {
        Self {
            asm_line: vec![
                ".intel_syntax noprefix".into(),
                ".globl main".into(),
                "main:".into(),
            ]
        }
    }

    fn gen(mut self, node: Option<Box<Node>>) -> Vec<String> {
        self.gen_helper(node.as_ref());

        // 最後のスタックの値を取り出しプログラムを終了
        self.asm_line.push("  pop rax".to_owned());
        self.asm_line.push("  ret".to_owned());

        utils::vec_plus_n(self.asm_line).unwrap()
    }

    fn gen_helper(&mut self, node: Option<&Box<Node>>) {
        if let Some(node)  = node {
            if let Token::Num(x) = node.token {
                #[cfg(debug_assertion)]
                println!("  push {}", x);

                self.asm_line.push(format!("  push {}", x));
                return;
            }
        }

        self.gen_helper(node.unwrap().lhs.as_ref());
        self.gen_helper(node.unwrap().rhs.as_ref());

        self.asm_line.push("  pop rdi".to_owned());
        self.asm_line.push("  pop rax".to_owned());

        match node.unwrap().token {
            Token::Add => {
                self.asm_line.push(format!("  add rax, rdi"));
            },
            Token::Sub => {
                self.asm_line.push(format!("  sub rax, rdi"));
            },
            Token::Mul => {
                self.asm_line.push(format!("  imul rax, rdi"));
            },
            Token::Div => {
                self.asm_line.push(format!("  cqo"));
                self.asm_line.push(format!("  idiv rdi"));
            },
            Token::Lbr => todo!(),
            Token::Rbr => todo!(),
            Token::RESERVED(_) => todo!(),
            Token::Num(_) => todo!(),
            Token::EOF => todo!(),
        }

        self.asm_line.push(format!("  push rax"));
    }
}

mod tests {
    use crate::{cmd, file, generator::AsmGenerator, parser::Parser, tokenizer::Tokenizer};

    #[test]
    pub fn tokenize_then_parse_then_gen() {
        let input = "2*3+4*5";
        let tokenizer = Tokenizer::new(input.to_owned());

        let tokens = tokenizer.tokenize();
        println!("{tokens:?}");

        let mut parser = Parser::from(tokens);
        let head = parser.expr();

        let mut generator = AsmGenerator::new();
        let x = generator.gen(head);

        println!("{x:#?}");

        file::write_vec_in_file("tmp.s", x);
        let status = cmd::run_assembly("tmp.s");

        println!("{status:#?}");
    }
}
