// Rustの列挙子は変数を紐づけることが出来るので、そもそも構造体にする必要がないのでは。
#[derive(Debug)]
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

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,

    Lbr, // (
    Rbr, // )
}

// struct Token {
//     kind: TokenKind,
//     next: Box<Token>,
//     val: i32,
// }

// impl Token {
//     pub fn new(kind: Token, next: Box<Token>, val: i32) -> Self {
//         Self {
//             kind,
//             next,
//             val,
//         }
//     }
// }

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
            // '-' => {

            // },
            _ => {}
        }
    }
}

// トークンの種類を定義
// #[derive(Debug, PartialEq)]
// enum Token {
//     Int(i32),
//     Plus,
//     Minus,
//     Star,
//     Slash,
//     LParen,
//     RParen,
//     EOF,
// }

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
        // let input = "3 + 4 * (2 - 1)";
        // let tokenizer = Tokenizer::new(input.to_owned());
        // let tokens = tokenizer.tokenize();

        println!("xxxx");
    }
}

// // 字句解析器
// fn lexer(input: &str) -> Vec<Token> {
//     let mut tokens = Vec::new();
//     let mut chars = input.chars().peekable();

//     while let Some(&ch) = chars.peek() {
//         match ch {
//             '+' => {
//                 tokens.push(Token::Plus);
//                 chars.next();
//             },
//             '-' => {
//                 tokens.push(Token::Minus);
//                 chars.next();
//             },
//             '*' => {
//                 tokens.push(Token::Star);
//                 chars.next();
//             },
//             '/' => {
//                 tokens.push(Token::Slash);
//                 chars.next();
//             },
//             '(' => {
//                 tokens.push(Token::LParen);
//                 chars.next();
//             },
//             ')' => {
//                 tokens.push(Token::RParen);
//                 chars.next();
//             },
//             '0'..='9' => {
//                 let mut number = 0;
//                 while let Some(&ch) = chars.peek() {
//                     if ch.is_digit(10) {
//                         number = number * 10 + ch.to_digit(10).unwrap() as i32;
//                         chars.next();
//                     } else {
//                         break;
//                     }
//                 }
//                 tokens.push(Token::Int(number));
//             },
//             _ if ch.is_whitespace() => {
//                 chars.next();
//             }
//             _ => {
//                 // 無視される文字またはエラー処理
//                 chars.next();
//             }
//         }
//     }

//     tokens.push(Token::EOF);
//     tokens
// }

// fn main() {
//     let input = "3 + 4 * (2 - 1)";
//     let tokens = lexer(input);
//     println!("{:?}", tokens);
// }
