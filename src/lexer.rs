use crate::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token(TokenType::EOF, "".to_string());
        }

        let ch = self.input.chars().nth(self.position).unwrap();
        let token = match ch {
            '+' => Token(TokenType::Plus, ch.to_string()),
            '-' => Token(TokenType::Minus, ch.to_string()),
            '*' => Token(TokenType::Multiply, ch.to_string()),
            '/' => Token(TokenType::Divide, ch.to_string()),
            '^' => Token(TokenType::Pow, ch.to_string()),
            '(' => Token(TokenType::LeftParen, ch.to_string()),
            ')' => Token(TokenType::RightParen, ch.to_string()),
            '%' => Token(TokenType::Mod, ch.to_string()),
            ch if ch.is_numeric() => {
                let literal = self.read_number();
                Token(TokenType::Int, literal)
            }
            _ => Token(TokenType::Illegal, ch.to_string()),
        };
        self.position += 1;
        token
    }

    fn read_number(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len()
            && self.input.chars().nth(self.position).unwrap().is_numeric()
        {
            self.position += 1;
        }
        let end = self.position;
        let result = self.input[start..end].to_string();

        self.position -= 1;

        result
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self
                .input
                .chars()
                .nth(self.position)
                .unwrap()
                .is_whitespace()
        {
            self.position += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Token, TokenType};

    use super::Lexer;

    #[test]
    fn next_token_test() {
        let input = "1 + 2 * (12 - 6) / 3 + 2 ^ 3";
        let expected_seq = vec![
            Token(TokenType::Int, "1".to_string()),
            Token(TokenType::Plus, "+".to_string()),
            Token(TokenType::Int, "2".to_string()),
            Token(TokenType::Multiply, "*".to_string()),
            Token(TokenType::LeftParen, "(".to_string()),
            Token(TokenType::Int, "12".to_string()),
            Token(TokenType::Minus, "-".to_string()),
            Token(TokenType::Int, "6".to_string()),
            Token(TokenType::RightParen, ")".to_string()),
            Token(TokenType::Divide, "/".to_string()),
            Token(TokenType::Int, "3".to_string()),
            Token(TokenType::Plus, "+".to_string()),
            Token(TokenType::Int, "2".to_string()),
            Token(TokenType::Pow, "^".to_string()),
            Token(TokenType::Int, "3".to_string()),
            Token(TokenType::EOF, "".to_string()),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected in expected_seq.iter() {
            let actual = lexer.next_token();
            assert_eq!(actual, *expected);
        }
    }
}
