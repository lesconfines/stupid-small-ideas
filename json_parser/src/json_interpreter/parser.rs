use crate::json_interpreter::lexer::Lexer;
use crate::json_interpreter::parse_error::{ParseError, ParseResult};
use crate::Token;
use std::collections::HashMap;
use std::fmt;

pub enum JsonValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
}

fn display_json_value(json_value: &JsonValue, f: &mut fmt::Formatter, space: usize) -> fmt::Result {
    let spaces = std::iter::repeat("  ").take(space).collect::<String>();
    match json_value {
        JsonValue::String(string) => writeln!(f, "{}string \"{}\"", spaces, string),
        JsonValue::Number(num) => writeln!(f, "{}number {}", spaces, num),
        JsonValue::Boolean(boolean) => writeln!(f, "{}boolean {}", spaces, boolean),
        JsonValue::Object(map) => {
            writeln!(f, "\n{}Object {{", spaces);
            map.iter().for_each(|(key, value)| {
                write!(f, "{}  {}: ", spaces, key);
                display_json_value(value, f, space + 1).unwrap();
            });
            writeln!(f, "{}}}", spaces)
        }
        JsonValue::Array(array) => {
            writeln!(f, "\n{}Array {{", spaces);
            array.iter().for_each(|value| {
                display_json_value(value, f, space + 1).unwrap();
            });
            writeln!(f, "{}}}", spaces)
        }
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        display_json_value(self, f, 0)
    }
}

pub struct Parser<'a> {
    l: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser {
            l: Lexer::new(input),
        }
    }

    pub fn parse(&mut self) -> ParseResult<JsonValue> {
        self.parse_json_value()
    }

    fn parse_json_value(&mut self) -> ParseResult<JsonValue> {
        match self.l.next_token() {
            Token::LBraces => self.parse_object(),
            Token::LBracket => self.parse_array(),
            Token::RBracket => Err(ParseError::ArrayEnd),
            Token::Boolean(boolean) => Ok(JsonValue::Boolean(boolean)),
            Token::Int(num) => Ok(JsonValue::Number(num as f64)),
            Token::String(string) => Ok(JsonValue::String(string)),
            _ => Err(ParseError::ParsingError),
        }
    }

    fn parse_object(&mut self) -> ParseResult<JsonValue> {
        let mut paires = vec![];
        loop {
            match self.parse_pair() {
                Ok(paire) => paires.push(paire),
                Err(ParseError::CommaEndingObject) => break,
                Err(_) => return Err(ParseError::ParsingError),
            }
        }
        Ok(JsonValue::Object(paires.into_iter().collect()))
    }

    fn parse_array(&mut self) -> ParseResult<JsonValue> {
        let mut json_values = vec![];
        loop {
            match self.parse_json_value() {
                Ok(value) => {
                    json_values.push(value);
                    match self.l.next_token() {
                        Token::Comma => continue,
                        Token::RBracket => break,
                        _ => panic!(),
                    }
                }
                Err(ParseError::ArrayEnd) => break,
                Err(_) => return Err(ParseError::ParsingError),
            }
        }
        Ok(JsonValue::Array(json_values))
    }

    fn parse_pair(&mut self) -> ParseResult<(String, JsonValue)> {
        match self.l.next_token() {
            Token::String(key) => match self.parse_paire_value() {
                Ok(value) => Ok((key, value)),
                _ => Err(ParseError::ParsingError),
            },
            Token::Int(num) => {
                let key = num.to_string();
                match self.parse_paire_value() {
                    Ok(value) => Ok((key, value)),
                    _ => Err(ParseError::ParsingError),
                }
            }
            Token::Comma => self.parse_pair(),
            Token::RBraces => Err(ParseError::CommaEndingObject),
            _ => Err(ParseError::ParsingError),
        }
    }

    fn parse_paire_value(&mut self) -> ParseResult<JsonValue> {
        match self.l.next_token() {
            Token::DoubleDot => self.parse_json_value(),
            _ => Err(ParseError::ParsingError),
        }
    }
}
