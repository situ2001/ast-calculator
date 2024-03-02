pub mod evaluate;
pub mod expression;
pub mod lexer;
pub mod parser;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn evalExpression(s: &str) -> f64 {
    let lexer = lexer::Lexer::new(s.to_string());
    let mut parser = parser::Parser::new(lexer);
    let expr = parser.parse();
    evaluate::eval(expr)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest = 0,
    /// + and -
    Sum = 1,
    /// *  and / and %
    Product = 2,
    /// - ahead of a number
    Prefix = 3,
    /// ^
    Pow = 4,
    /// (...)
    Grouping = 5,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TokenType {
    Int,
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,
    Pow,
    LeftParen,
    RightParen,
    EOF,
    Illegal,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token(pub TokenType, pub String);

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token(token_type, literal)
    }

    pub fn token_type(&self) -> TokenType {
        self.0
    }

    pub fn literal(&self) -> String {
        self.1.clone()
    }

    pub fn precedence(&self) -> Precedence {
        match self.0 {
            TokenType::Plus | TokenType::Minus => Precedence::Sum,
            TokenType::Multiply | TokenType::Divide | TokenType::Mod => Precedence::Product,
            TokenType::Pow => Precedence::Pow,
            TokenType::LeftParen => Precedence::Grouping,
            _ => Precedence::Lowest,
        }
    }
}
