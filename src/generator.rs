use std::vec;


use crate::parser::{OpxNode, Node};
use crate::tokenizer::Token;


pub struct AsmGenerator {
    node: OpxNode,
    asm_line: Vec<String>
}

impl AsmGenerator {
    fn gen(&mut self, node: Option<Box<Node>>) -> Vec<String> {

        self.gen_helper(node.as_ref());

        todo!()
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

        println!("{}", "hoge1");
        println!("{}", "hoge2");

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
        // printf("  push rax\n");
    }
}
