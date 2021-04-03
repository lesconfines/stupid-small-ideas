use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers + Litterals
    String(String),
    Int(i32),
    Boolean(bool),

    //Delimiters
    Comma,
    DoubleDot,

    LBracket,
    RBracket,
    LBraces,
    RBraces,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Illegal => write!(f, "Token::Illegal"),
            Token::EOF => write!(f, "Token::EOF"),
            Token::Int(x) => write!(f, "Token::Int({})", x),
            Token::String(string) => write!(f, "Token::String({})", string),
            Token::Boolean(boolean) => write!(f, "Token::Boolean({})", boolean),
            Token::Comma => write!(f, "Token::Comma (,)"),
            Token::LBracket => write!(f, "Token::LBracket ["),
            Token::RBracket => write!(f, "Token::RBracket ]"),
            Token::LBraces => write!(f, "Token::LBraces {{"),
            Token::RBraces => write!(f, "Token::RBraces }}"),
            Token::DoubleDot => write!(f, "Token::DoubleDot :"),
        }
    }
}
