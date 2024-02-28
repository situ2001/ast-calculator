use crate::expression::{InfixExpression, IntegerLiteralExpression, PrefixExpression};
use crate::TokenType;

pub trait Visitor {
    type Result;

    fn visit_integer_literal(&self, expr: &IntegerLiteralExpression) -> Self::Result;
    fn visit_infix_expression(&self, expr: &InfixExpression) -> Self::Result;
    fn visit_prefix_expression(&self, expr: &PrefixExpression) -> Self::Result;
}

struct AstVisitor;

impl Visitor for AstVisitor {
    type Result = f64;
    fn visit_integer_literal(&self, expr: &IntegerLiteralExpression) -> Self::Result {
        expr.value as f64
    }

    fn visit_infix_expression(&self, expr: &InfixExpression) -> Self::Result {
        let left = expr.left.accept(self);
        let right = expr.right.accept(self);

        match expr.token.token_type() {
            TokenType::Plus => left + right,
            TokenType::Minus => left - right,
            TokenType::Multiply => left * right,
            TokenType::Divide => left / right,
            TokenType::Pow => left.powf(right),
            TokenType::Mod => left % right,
            _ => panic!("Unknown operator: {}", expr.token.literal()),
        }
    }

    fn visit_prefix_expression(&self, expr: &PrefixExpression) -> Self::Result {
        let right = expr.right.accept(self);

        match expr.token.token_type() {
            TokenType::Minus => -right,
            _ => panic!("Unknown operator: {}", expr.token.literal()),
        }
    }
}

pub fn eval(expr: Box<dyn crate::expression::Expression>) -> f64 {
    expr.accept(&AstVisitor)
}

#[cfg(test)]
mod tests {
    use crate::evaluate::AstVisitor;
    use crate::lexer::Lexer;

    #[test]
    fn eval_test() {
        let test_cases = vec![
            ("5+ 5", 10.0),
            ("1 + 2 - 3", 0.0),
            ("10 - 12 * 12", -134.0),
            ("16 / 4 ^ 2", 1.0),
            ("(5 - 2) * 3", 9.0),
            (" 5 +((12 + 8) / 4))", 10.0),
            ("-4 * 5 + 2", -18.0),
            ("5 + -3", 2.0),
            ("5/4", 1.25),
        ];

        for (input, expected) in test_cases {
            let lexer = Lexer::new(input.to_string());
            let mut parser = crate::parser::Parser::new(lexer);
            let expr = parser.parse();
            let result = expr.accept(&AstVisitor);
            assert_eq!(result, expected);
        }
    }
}
