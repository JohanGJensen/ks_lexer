#[derive(Debug)]
pub enum TokenType {
    // values
    String,
    Number,

    // variables & keywords
    Function,
    StringVar,
    IntVar,
    Identifier,
    If,
    Else,
    Return,

    // delimiters
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    SemiColon,
    Colon,
    EOF,
    
    // operators
    Assign,
    Equal,
    NotEqual,
    Plus,
    Minus,
    Divide,
    Multiply,
}

pub struct Token {
    pub token: TokenType,
    pub value: String,
}
