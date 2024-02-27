// Rustの列挙子は変数を紐づけることが出来るので、そもそも構造体にする必要がないのでは。
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,      // +
    Sub,      // -
    Mul,      // *
    Div,      // /
    Lbr,      // (
    Rbr,      // )
    EQ,       // ==
    NE,       // !=
    LT,       // <
    LE,       // <=
    Num(i32), // 整数トークン
    EOF,      // 入力末端
}

pub struct Tokenizer {
    pub target: String,
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
                    let mut num_buf = "".to_string();

                    // 文字列を数値にする
                    while let Some(i) = chars.peek() {
                        if i.is_digit(10) {
                            num_buf.push(*i);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    let parsed_num = num_buf.parse::<i32>().unwrap();
                    let new_token = Token::Num(parsed_num);

                    self.tokens.push(new_token);
                }
                '(' => {
                    self.tokens.push(Token::Lbr);
                    chars.next();
                }
                ')' => {
                    self.tokens.push(Token::Rbr);
                    chars.next();
                }
                '=' => {
                    chars.next();

                    // == と続く場合……
                    if let Some(i) = chars.peek() {
                        match i {
                            '=' => {
                                self.tokens.push(Token::EQ);
                                chars.next();
                            }
                            _ => {
                                // TODO: 代入演算子の挿入
                            }
                        }
                    }
                }
                '>' => {
                    chars.next();

                    if let Some(i) = chars.peek() {
                        match i {
                            '=' => {
                                self.tokens.push(Token::LE);
                                chars.next();
                            }
                            _ => {
                                self.tokens.push(Token::LT);
                            }
                        }
                    }
                }
                '!' => {
                    chars.next();

                    if let Some(i) = chars.peek() {
                        match i {
                            '=' => {
                                self.tokens.push(Token::NE);
                                chars.next();
                            }
                            _ => (),
                        }
                    }
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
    use std::vec;

    use crate::tokenizer::{self, Token, Tokenizer};

    #[test]
    fn test() {
        let input = "3 + 4 * (2 - 1)";
        let tokenizer = Tokenizer::new(input.to_owned());
        let tokens = tokenizer.tokenize();

        println!("{:?}", tokens);
    }

    #[test]
    fn test_EQ() {
        let input = "0 == 0";
        let tokenizer = Tokenizer::new(input.to_owned());
        let tokens = tokenizer.tokenize();

        assert_eq!(vec![Token::Num(0), Token::EQ, Token::Num(0)], tokens);

        println!("{:?}", tokens);
    }

    #[test]
    fn test_NE() {
        let input = "1 != 0";
        let tokenizer = Tokenizer::new(input.to_owned());
        let tokens = tokenizer.tokenize();

        assert_eq!(vec![Token::Num(1), Token::NE, Token::Num(0)], tokens);

        println!("{:?}", tokens);
    }

    #[test]
    fn test_LE() {
        let input = "1 >= 0";
        let tokenizer = Tokenizer::new(input.to_owned());
        let tokens = tokenizer.tokenize();

        assert_eq!(vec![Token::Num(1), Token::LE, Token::Num(0)], tokens);

        println!("{:?}", tokens);
    }

    #[test]
    fn test_LT() {
        let input = "1 > 0";
        let tokenizer = Tokenizer::new(input.to_owned());
        let tokens = tokenizer.tokenize();

        assert_eq!(vec![Token::Num(1), Token::LT, Token::Num(0)], tokens);

        println!("{:?}", tokens);
    }
}
