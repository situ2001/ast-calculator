use std::fmt::Debug;

use crate::{evaluate::Visitor, Token};

/// Expression is a trait that represents an expression in the AST.
///
/// Think it as a component of the AST.
pub trait Expression: ToString + Debug + Visitable<f64> {
    fn token_literal(&self) -> String;
}

pub trait Visitable<T> {
    fn accept(&self, visitor: &dyn Visitor<Result = T>) -> T;
}

impl<F64> Visitable<F64> for IntegerLiteralExpression {
    fn accept(&self, visitor: &dyn Visitor<Result = F64>) -> F64 {
        visitor.visit_integer_literal(self)
    }
}

impl<F64> Visitable<F64> for InfixExpression {
    fn accept(&self, visitor: &dyn Visitor<Result = F64>) -> F64 {
        visitor.visit_infix_expression(self)
    }
}

impl<F64> Visitable<F64> for PrefixExpression {
    fn accept(&self, visitor: &dyn Visitor<Result = F64>) -> F64 {
        visitor.visit_prefix_expression(self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntegerLiteralExpression {
    pub token: Token,
    pub value: i64,
}

impl Expression for IntegerLiteralExpression {
    fn token_literal(&self) -> String {
        self.token.1.clone()
    }
}

impl ToString for IntegerLiteralExpression {
    fn to_string(&self) -> String {
        self.token.1.clone()
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub left: Box<dyn Expression>,
    pub token: Token,
    pub right: Box<dyn Expression>,
}

impl Expression for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.1.clone()
    }
}

impl ToString for InfixExpression {
    fn to_string(&self) -> String {
        format!(
            "({} {} {})",
            self.left.to_string(),
            self.token.literal(),
            self.right.to_string()
        )
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,

    pub right: Box<dyn Expression>,
}

impl Expression for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.1.clone()
    }
}

impl ToString for PrefixExpression {
    fn to_string(&self) -> String {
        format!("({} {})", self.token_literal(), self.right.to_string())
    }
}
