// Define the AST grammar
/*
expression   := literal | grouping | unary | binary ;
literal      := NUMBER | STRING | "true" | "false" | "nil" ;
grouping     := "(" expression ")" ;
unary        := ("-" | "!") expression ;
binary       := expression operator expression ;
operator     :=  "==" | "!=" | "<" | "<=" | ">" | ">="
                | "+" | "-"  | "*" | "/" ;
*/

use crate::tokenize;

pub enum Expr {
    Literal(Literal),
    Grouping(Box<Expr>),
    Unary {
        operator: UnaryOperator,
        operand: Box<Expr>,
    },
    Binary {
        operator: BinaryOperator,
        operand1: Box<Expr>,
        operand2: Box<Expr>,
    },
}

pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

pub enum UnaryOperator {
    Minus,
    Not,
}

pub enum BinaryOperator {
    EqualEqual,
    BangEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl BinaryOperator {
    pub fn from_token_type(token: &tokenize::TokenType) -> Option<Self> {
        match token {
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::EqualEqual) => Some(BinaryOperator::EqualEqual),
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::BangEqual) => Some(BinaryOperator::BangEqual),
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::Less) => Some(BinaryOperator::LessThan),
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::LessEqual) => Some(BinaryOperator::LessThanOrEqual),
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::Greater) => Some(BinaryOperator::GreaterThan),
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::GreaterEqual) => Some(BinaryOperator::GreaterThanOrEqual),
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::Plus) => Some(BinaryOperator::Plus),
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::Minus) => Some(BinaryOperator::Minus),
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::Star) => Some(BinaryOperator::Multiply),
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::Slash) => Some(BinaryOperator::Divide),
            _ => None,
        }
    }
}

impl UnaryOperator {
    pub fn from_token_type(token: &tokenize::TokenType) -> Option<Self> {
        match token {
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::Minus) => Some(UnaryOperator::Minus),
            tokenize::TokenType::BoundaryTokens(tokenize::BoundaryTokens::Bang) => Some(UnaryOperator::Not),
            _ => None,
        }
    }
}


impl Literal {
    pub fn from_token_type(token: &tokenize::TokenType) -> Option<Self> {
        match token {
            tokenize::TokenType::Literals(tokenize::Literals::Number(n)) => Some(Literal::Number(*n)),
            tokenize::TokenType::Literals(tokenize::Literals::String(s)) => Some(Literal::String(s.clone())),
            tokenize::TokenType::Keywords(tokenize::Keywords::False) => Some(Literal::Boolean(false)),
            tokenize::TokenType::Keywords(tokenize::Keywords::True) => Some(Literal::Boolean(true)),
            tokenize::TokenType::Literals(tokenize::Literals::Nil) => Some(Literal::Nil),
            _ => None,
        }
    }
}