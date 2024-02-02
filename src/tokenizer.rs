// Rustの列挙子は変数を紐づけることが出来るので、そもそも構造体にする必要がないのでは。
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,                // +
    Sub,                // -
    Mul,                // *
    Div,                // /
    Lbr,                // (
    Rbr,                // )
    RESERVED(Operator), // 記号
    Num(i32),           // 整数トークン
    EOF,                // 入力末端
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,

    Lbr, // (
    Rbr, // )
}

pub struct Tokenizer {
    pub target: String,
    // pub head: Option<Box<Token>>,
    pub tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(target: String) -> Self {
        Self {
            target,
            tokens: vec![],
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut chars = self.target.chars().peekable();

        println!("{chars:?}");

        while let Some(c) = chars.peek() {
            // self.parse_char(c);
            println!("{}", c);
            match c {
                ' ' => {
                    chars.next();
                }
                '+' => {
                    self.tokens.push(Token::Add);
                    chars.next();
                }
                '-' => {
                    self.tokens.push(Token::Sub);
                    chars.next();
                }
                '*' => {
                    self.tokens.push(Token::Mul);
                    chars.next();
                }
                '/' => {
                    self.tokens.push(Token::Div);
                    chars.next();
                }
                '0'..='9' => {
                    let parsed_num = c.to_digit(10).unwrap() as i32;
                    let new_token = Token::Num(parsed_num);
                    self.tokens.push(new_token);
                    chars.next();
                }
                '(' => {
                    self.tokens.push(Token::Lbr);
                    chars.next();
                }
                ')' => {
                    self.tokens.push(Token::Rbr);
                    chars.next();
                }
                _ => {
                    chars.next();
                }
            }
        }

        self.tokens
    }

    pub fn parse_char(&mut self, c: char) {
        match c {
            ' ' => (),
            '+' => {}
            '-' => {}
            '0'..='9' => {
                let parsed_num = c.to_digit(10).unwrap() as i32;
                let new_token = Token::Num(parsed_num);
                self.tokens.push(new_token);
            }
            _ => {}
        }
    }
}

mod tests {
    use crate::tokenizer::{self, Tokenizer};

    #[test]
    fn test() {
        let input = "3 + 4 * (2 - 1)";
        let tokenizer = Tokenizer::new(input.to_owned());
        let tokens = tokenizer.tokenize();

        println!("{:?}", tokens);
    }

    #[test]
    fn test2() {
        println!("xxxx");
    }
}
