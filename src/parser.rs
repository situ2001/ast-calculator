use crate::{
    expression::{Expression, InfixExpression, IntegerLiteralExpression, PrefixExpression},
    lexer::Lexer,
    Precedence, {Token, TokenType},
};

#[allow(dead_code)]
pub struct Parser {
    lexer: Lexer,
    token_current: Token,
    token_next: Token,
}

impl Parser {
    #[allow(dead_code)]
    pub fn new(lexer: Lexer) -> Self {
        let mut lexer = lexer;

        let token_current = lexer.next_token();
        let token_next = lexer.next_token();

        assert_ne!(token_current.token_type(), TokenType::Illegal);
        assert_ne!(token_next.token_type(), TokenType::Illegal);

        Parser {
            lexer,
            token_current,
            token_next,
        }
    }

    #[allow(dead_code)]
    pub fn parse(&mut self) -> Box<dyn Expression> {
        self.parse_expression(Precedence::Lowest)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Box<dyn Expression> {
        let mut left_expr = match self.token_current.token_type() {
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Minus => self.parse_prefix_expression(),
            TokenType::LeftParen => self.parse_grouped_expression(),
            _ => unimplemented!("{:?}", self.token_current.token_type()),
        };

        while (!(self.token_next.token_type() == TokenType::EOF))
            && precedence < self.token_next.precedence()
        {
            match self.token_next.token_type() {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Multiply
                | TokenType::Divide
                | TokenType::Pow
                | TokenType::Mod => {
                    self.read_next_token();
                    left_expr = self.parse_infix_expression(left_expr);
                }
                _ => {
                    return left_expr;
                }
            };
        }

        left_expr
    }

    fn read_next_token(&mut self) {
        self.token_current = self.token_next.clone();
        self.token_next = self.lexer.next_token();
        assert_ne!(self.token_current.token_type(), TokenType::Illegal);
        assert_ne!(self.token_next.token_type(), TokenType::Illegal);
    }

    fn parse_integer_literal(&mut self) -> Box<dyn Expression> {
        let res = IntegerLiteralExpression {
            token: self.token_current.clone(),
            value: self.token_current.literal().parse().unwrap(),
        };

        Box::new(res)
    }

    fn parse_prefix_expression(&mut self) -> Box<dyn Expression> {
        let token = self.token_current.clone();

        self.read_next_token();

        let expr = PrefixExpression {
            token,
            right: self.parse_expression(Precedence::Prefix),
        };

        Box::new(expr)
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression>) -> Box<dyn Expression> {
        let token = self.token_current.clone();

        let precedence = self.token_current.precedence();
        self.read_next_token();
        let right = self.parse_expression(precedence);

        let expr = InfixExpression { token, left, right };

        Box::new(expr)
    }

    fn parse_grouped_expression(&mut self) -> Box<dyn Expression> {
        self.read_next_token();

        // reset the precedence to lowest to allow for the inner expression to be parsed
        let expr = self.parse_expression(Precedence::Lowest);

        // ensure the next token is a right paren
        assert_eq!(self.token_next.token_type(), TokenType::RightParen);

        self.read_next_token();

        expr
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parser_test() {
        use crate::lexer::Lexer;
        use crate::parser::Parser;

        let test_cases = vec![
            ("5+ 5", "(5 + 5)"),
            ("1 + 2 - 3", "((1 + 2) - 3)"),
            ("10 - 12 * 12", "(10 - (12 * 12))"),
            ("16 / 4 ^ 2", "(16 / (4 ^ 2))"),
            ("(5 - 2) * 3", "((5 - 2) * 3)"),
            (" 5 +((12 + 8) / 4))", "(5 + ((12 + 8) / 4))"),
            ("-4 * 5 + 2", "(((- 4) * 5) + 2)"),
            ("5 + -3", "(5 + (- 3))"),
        ];

        for (input, expected) in test_cases {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let expr = parser.parse();
            dbg!(input);
            dbg!(&expr);
            dbg!(expected);
            assert_eq!(expr.to_string(), expected);
        }
    }
}
