use std::vec;

use crate::parser::{Node, OpxNode, Parser};
use crate::tokenizer::{Token, Tokenizer};
use crate::{cmd, file, utils};

pub struct AsmGenerator {
    asm_line: Vec<String>,
}

impl AsmGenerator {
    fn new() -> Self {
        Self {
            asm_line: vec![
                ".intel_syntax noprefix".into(),
                ".globl main".into(),
                "main:".into(),
            ],
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
        if let Some(node) = node {
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
            }
            Token::Sub => {
                self.asm_line.push(format!("  sub rax, rdi"));
            }
            Token::Mul => {
                self.asm_line.push(format!("  imul rax, rdi"));
            }
            Token::Div => {
                self.asm_line.push(format!("  cqo"));
                self.asm_line.push(format!("  idiv rdi"));
            }
            Token::Lbr => todo!(),
            Token::Rbr => todo!(),
            Token::Num(_) => todo!(),
            Token::EOF => todo!(),
            Token::EQ => {
                self.asm_line.push(format!("  cmp rax, rdi"));
                self.asm_line.push(format!("  sete al"));
                self.asm_line.push(format!("  movzb rax, al"));
            },
            Token::NE => {
                self.asm_line.push(format!("  cmp rax, rdi"));
                self.asm_line.push(format!("  setne al"));
                self.asm_line.push(format!("  movzb rax, al"));
            },
            Token::LT => {
                self.asm_line.push(format!("  cmp rax, rdi"));
                self.asm_line.push(format!("  setl al"));
                self.asm_line.push(format!("  movzb rax, al"));
            },
            Token::LE => {
                self.asm_line.push(format!("  cmp rax, rdi"));
                self.asm_line.push(format!("  setle al"));
                self.asm_line.push(format!("  movzb rax, al"));
            },
            Token::GT => {
                self.asm_line.push(format!("  cmp rax, rdi"));
                self.asm_line.push(format!("  setg al"));
                self.asm_line.push(format!("  movzb rax, al"));
            },
            Token::GE => {
                self.asm_line.push(format!("  cmp rax, rdi"));
                self.asm_line.push(format!("  setge al"));
                self.asm_line.push(format!("  movzb rax, al"));
            },
        }

        self.asm_line.push(format!("  push rax"));
    }
}

pub fn run_expr(expr: &str) -> anyhow::Result<usize> {
    let tokenizer = Tokenizer::new(expr.to_owned());
    let tokens = tokenizer.tokenize();
    println!("Final token: {tokens:?}");

    let mut parser = Parser::from(tokens);
    let head = parser.expr();
    println!("Final: {head:?}");

    let generator = AsmGenerator::new();
    let asm_line = generator.gen(head);

    let _ = file::write_vec_in_file("tmp.s", asm_line);
    cmd::run_assembly("tmp.s")
}

pub fn assert_expr_eq(expr: &str, expect: usize) {
    let status = run_expr(expr).unwrap();
    assert_eq!(status, expect);
}

mod tests {
    use anyhow::{Ok, Result};

    use crate::{
        cmd, file,
        generator::{run_expr, AsmGenerator},
        parser::Parser,
        tokenizer::Tokenizer,
    };

    use super::assert_expr_eq;

    #[test]
    fn entry() {
        expr_test_set();
        unary_test();
        cmp_test();
    }

    #[test]
    pub fn expr_test_set() {
        // cargo test は並行実行されるので、エントリポイントを設け逐次実行させる
        assert_expr_eq("2*3+4*5", 26);
        assert_expr_eq("2*(3+4)*5", 70);
        assert_expr_eq("5+6*7", 47);
        assert_expr_eq("5*(9-6)", 15);
        assert_expr_eq("(3+5)/2", 4);
    }

    #[test]
    pub fn unary_test() {
        // assert_expr_eq("-1+2", 1); // 0-1+2
        assert_expr_eq("-10+20", 10);
        assert_expr_eq("- -10", 10);
        assert_expr_eq("- - +10", 10);
    }

    #[test]
    pub fn cmp_test() {
        // assert_expr_eq("-1+2", 1); // 0-1+2
        assert_expr_eq("0==1", 0);
        assert_expr_eq("0==0", 1);
        assert_expr_eq("23==23", 1);

        assert_expr_eq("0!=1", 1);
        assert_expr_eq("0!=0", 0);

        assert_expr_eq("0 > 1", 0);
        assert_expr_eq("1 > 0", 1);

        assert_expr_eq("1 >= 0", 1);
        assert_expr_eq("1 >= 1", 1);
        assert_expr_eq("1 >= 2", 0);

        assert_expr_eq("0 < 1", 1);
        assert_expr_eq("1 < 0", 0);

        assert_expr_eq("1 <= 0", 0);
        assert_expr_eq("1 <= 1", 1);
        assert_expr_eq("1 <= 2", 1);
        // assert_expr_eq("- -10", 10);
        // assert_expr_eq("- - +10", 10);
    }
}
