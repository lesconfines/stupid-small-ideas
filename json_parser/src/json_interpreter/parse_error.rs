use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseError {
    ParsingError,
    CommaEndingObject,
    ArrayEnd,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::ParsingError => write!(f, "Parse Error"),
            ParseError::CommaEndingObject => write!(f, "Comma Ending Object"),
            ParseError::ArrayEnd => write!(f, "Array End"),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
