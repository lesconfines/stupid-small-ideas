use crate::json_interpreter::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn read_string(&mut self, ch: char) -> String {
        let mut identifier = String::new();
        identifier.push(ch);
        while let Some(&ch) = self.peek_char() {
            if ch != '"' {
                identifier.push(self.read_char().unwrap());
            } else {
                self.read_char();
                break;
            }
        }
        identifier
    }

    fn read_int(&mut self, ch: char) -> i32 {
        let mut number = String::new();

        number.push(ch);
        while let Some(&ch) = self.peek_char() {
            if ch.is_numeric() {
                number.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_boolean(&mut self, ch: char) -> Option<bool> {
        let ch_array_true = vec![
            Some(ch),
            self.read_char(),
            self.read_char(),
            self.read_char(),
        ];
        match ch_array_true.iter().all(|x| x.is_some()) {
            true => {
                let true_value_attempt =
                    ch_array_true.iter().map(|x| x.unwrap()).collect::<String>();
                if true_value_attempt == "true" {
                    Some(true)
                } else if true_value_attempt + "e" == "false" {
                    self.read_char();
                    Some(false)
                } else {
                    None
                }
            }
            false => None,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.read_char() {
            Some('{') => Token::LBraces,
            Some('}') => Token::RBraces,
            Some('[') => Token::LBracket,
            Some(']') => Token::RBracket,
            Some(',') => Token::Comma,
            Some(':') => Token::DoubleDot,
            Some('"') => {
                if let Some(ch) = self.read_char() {
                    Token::String(self.read_string(ch))
                } else {
                    Token::Illegal
                }
            }
            Some(ch) => {
                if is_letter(ch) {
                    match self.read_boolean(ch) {
                        Some(boolean) => Token::Boolean(boolean),
                        None => Token::Illegal,
                    }
                } else if ch.is_numeric() {
                    Token::Int(self.read_int(ch))
                } else {
                    Token::Illegal
                }
            }
            None => Token::EOF,
        }
    }
}
