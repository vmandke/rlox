/*
Recursive Descent Parser for Lox

eg:     '('     '3'     '+'     '4'     ')'     '*'     '5'
idx:     0       1       2       3       4       5       6
---------------------------------------------------------------------------
descend through rule levels of lower-> higher precedence


expr -> equality
equality -> comparison ( ( "!=" | "==" ) comparison )*
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )
term -> factor ( ( "-" | "+" ) factor )*
factor -> unary ( ( "/" | "*" ) unary )*
unary -> ( "!" | "-" ) unary | primary
primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"


---------------------------------------------------------------------------
idx 0: seen '(' rule expr [evaluate equality]
idx 0: seen                           => equality [evaluate comparison]
idx 0: seen                           => comparison [evaluate term]
idx 0: seen                           => term [evaluate factor]
idx 0: seen                           => factor [evaluate unary]
idx 0: seen                           => unary [evaluate ( "!" | "-" ) unary | primary]
idx 0: seen '('   advance             => primary [evaluate NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"]
idx 1: seen '('                             "(" expression ")"  => expression [evaluate equality]
idx 1: seen '3'                                 expression ")"  => equality [evaluate comparison]
idx 1: seen '3'                                 expression ")"  => comparison [evaluate term]
idx 1: seen '3'                                 expression ")"  => term [evaluate factor]
idx 1: seen '3'                                 expression ")"  => factor [evaluate unary]
idx 1: seen '3'                                 expression ")"  => unary [evaluate ( "!" | "-" ) unary | primary]
idx 1: seen '3'   advance                       expression ")"  => primary [evaluate NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"]
idx 2: seen '3'                                           => (primary rule matches NUMBER)
idx 2: seen '3'                                           => return NUMBER(3) to unary
idx 2: seen '+'      factor ( ( "-" | "+" ) factor )*     => return NUMBER(3) to factor
idx 2: seen '+'      term ( ( "-" | "+" ) factor )*       => match '+'
idx 3: seen '4'                                                 => factor
idx 3: seen '4'                                                 => unary
idx 3: seen '4'                                                 => primary [evaluate NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"]
idx 4: seen '4'                                                 => (primary rule matches NUMBER)
idx 4: seen '4'                                                 => return NUMBER(4) to unary
idx 4: seen '4'                                                 => return NUMBER(4) to factor
idx 4: seen '4'                                                 => combine NUMBER(3) + NUMBER(4)
idx 4: seen ')'                                                 => term ( ( "-" | "+" ) factor )*
idx 4: seen ')'                                                 => no ( "-" | "+" ) so exit term loop
idx 4: seen ')'                                                 => return (NUMBER(3) + NUMBER(4)) to comparison
idx 4: seen ')'                                                 => return (NUMBER(3) + NUMBER(4)) to equality
idx 4: seen ')'                                                 => return (NUMBER(3) + NUMBER(4)) to expr
idx 4: seen ')'   advance                                       => primary matches "(" expression ")" → consume ')'
idx 5: seen '*'                                                 => return (NUMBER(3) + NUMBER(4)) to unary
idx 5: seen '*'                                                 => return (NUMBER(3) + NUMBER(4)) to factor
idx 5: seen '*'   advance    factor ( ( "/" | "*" ) unary )*    => match '*'
idx 6: seen '5'                                                 => unary
idx 6: seen '5'   advance                                       => primary [evaluate NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"]
idx 7: seen '5'                                                 => (primary rule matches NUMBER)
idx 7: seen '5'                                                 => return NUMBER(5) to unary
idx 7: seen '5'                                                 => combine (NUMBER(3) + NUMBER(4)) * NUMBER(5)
idx 7: seen EOF   advance                                       => exit factor loop
idx 7: seen EOF                                                 => return ((NUMBER(3) + NUMBER(4)) * NUMBER(5)) to term
idx 7: seen EOF                                                 => no ( "-" | "+" ) so exit term loop
idx 7: seen EOF                                                 => return ((NUMBER(3) + NUMBER(4)) * NUMBER(5)) to comparison
idx 7: seen EOF                                                 => return ((NUMBER(3) + NUMBER(4)) * NUMBER(5)) to equality
idx 7: seen EOF                                                 => return ((NUMBER(3) + NUMBER(4)) * NUMBER(5)) to expr


*/

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            pos: 0,
        }
    }
    fn get(&self, _pos: usize) -> Option<&Token> {
        if _pos < self.tokens.len() {
            Some(&self.tokens[_pos])
        } else {
            None
        }
    }

    pub fn advance(&mut self) -> Option<&Token> {
        self.pos += 1;
        self.get(self.pos)
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.get(self.pos)
    }

    pub fn consume(&mut self, expected_token_type: &TokenType) -> Result<&Token, LoxError> {
        if let Some(token) = self.peek() {
            if &token.token_type == expected_token_type {
                return self.advance().ok_or_else(|| {
                    LoxError::ParserErrorCannotConsumeExpectedType {
                        expected_token_type: format!("{:?}", expected_token_type),
                    }
                });
            }
        }
        Err(LoxError::ParserErrorCannotConsumeExpectedType {
            expected_token_type: format!("{:?}", expected_token_type),
        })
    }
}

use crate::{
    errors::LoxError,
    grammar,
    tokenize::{self, BoundaryTokens, Keywords, Literals, Token, TokenType},
};

pub fn parse(tokens: Vec<tokenize::Token>) -> Result<grammar::Expr, LoxError> {
    let mut parser = Parser::new(tokens);
    expression(&mut parser)
}

/*

expr -> equality
equality -> comparison ( ( "!=" | "==" ) comparison )*
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )
term -> factor ( ( "-" | "+" ) factor )*
factor -> unary ( ( "/" | "*" ) unary )*
unary -> ( "!" | "-" ) unary | primary
primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"

*/

fn token_to_binary_op(token_type: &TokenType) -> Option<grammar::BinaryOperator> {
    match token_type {
        TokenType::BoundaryTokens(BoundaryTokens::Star) => Some(grammar::BinaryOperator::Multiply),
        TokenType::BoundaryTokens(BoundaryTokens::Slash) => Some(grammar::BinaryOperator::Divide),
        TokenType::BoundaryTokens(BoundaryTokens::Plus) => Some(grammar::BinaryOperator::Plus),
        TokenType::BoundaryTokens(BoundaryTokens::Minus) => Some(grammar::BinaryOperator::Minus),
        TokenType::BoundaryTokens(BoundaryTokens::EqualEqual) => {
            Some(grammar::BinaryOperator::EqualEqual)
        }
        TokenType::BoundaryTokens(BoundaryTokens::BangEqual) => {
            Some(grammar::BinaryOperator::BangEqual)
        }
        TokenType::BoundaryTokens(BoundaryTokens::Less) => Some(grammar::BinaryOperator::LessThan),
        TokenType::BoundaryTokens(BoundaryTokens::LessEqual) => {
            Some(grammar::BinaryOperator::LessThanOrEqual)
        }
        TokenType::BoundaryTokens(BoundaryTokens::Greater) => {
            Some(grammar::BinaryOperator::GreaterThan)
        }
        TokenType::BoundaryTokens(BoundaryTokens::GreaterEqual) => {
            Some(grammar::BinaryOperator::GreaterThanOrEqual)
        }
        _ => None,
    }
}

fn expression(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    equality(parser)
}

fn equality(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    let mut expr = comparison(parser)?;
    loop {
        let token_type = match parser.peek() {
            Some(t) => t.token_type.clone(),
            None => break,
        };
        let Some(op) = token_to_binary_op(&token_type) else {
            break;
        };
        parser.advance();
        let right = comparison(parser)?;
        expr = grammar::Expr::Binary {
            operator: op,
            operand1: Box::new(expr),
            operand2: Box::new(right),
        };
    }
    Ok(expr)
}

fn comparison(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    let mut expr = term(parser)?;
    loop {
        let token_type = match parser.peek() {
            Some(t) => t.token_type.clone(),
            None => break,
        };
        let Some(op) = token_to_binary_op(&token_type) else {
            break;
        };
        parser.advance();
        let right = term(parser)?;
        expr = grammar::Expr::Binary {
            operator: op,
            operand1: Box::new(expr),
            operand2: Box::new(right),
        };
    }
    Ok(expr)
}

fn term(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    let mut expr = factor(parser)?;
    loop {
        let token_type = match parser.peek() {
            Some(t) => t.token_type.clone(),
            None => break,
        };
        let Some(op) = token_to_binary_op(&token_type) else {
            break;
        };
        parser.advance();
        let right = factor(parser)?;
        expr = grammar::Expr::Binary {
            operator: op,
            operand1: Box::new(expr),
            operand2: Box::new(right),
        };
    }
    Ok(expr)
}

fn factor(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    let mut expr = unary(parser)?;
    loop {
        let token_type = match parser.peek() {
            Some(t) => t.token_type.clone(),
            None => break,
        };
        let Some(op) = token_to_binary_op(&token_type) else {
            break;
        };
        parser.advance();
        let right = unary(parser)?;
        expr = grammar::Expr::Binary {
            operator: op,
            operand1: Box::new(expr),
            operand2: Box::new(right),
        };
    }
    Ok(expr)
}

fn unary(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    let token_type = match parser.peek() {
        Some(t) => t.token_type.clone(),
        None => return primary(parser),
    };
    let op = match token_type {
        TokenType::BoundaryTokens(BoundaryTokens::Bang) => grammar::UnaryOperator::Not,
        TokenType::BoundaryTokens(BoundaryTokens::Minus) => grammar::UnaryOperator::Minus,
        _ => return primary(parser),
    };
    parser.advance();
    let operand = unary(parser)?;
    Ok(grammar::Expr::Unary {
        operator: op,
        operand: Box::new(operand),
    })
}

fn primary(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    let token_type = match parser.peek() {
        Some(t) => t.token_type.clone(),
        None => {
            return Err(LoxError::ParserErrorExpressionExpected(
                "Primary rule expected an expression but found end of input".into(),
            ));
        }
    };
    match token_type {
        TokenType::Literals(Literals::NumberInt(n)) => {
            parser.advance();
            Ok(grammar::Expr::Literal(grammar::Literal::NumberInt(n)))
        }
        TokenType::Literals(Literals::NumberFloat(n)) => {
            parser.advance();
            Ok(grammar::Expr::Literal(grammar::Literal::NumberFloat(n)))
        }
        TokenType::Literals(Literals::String(s)) => {
            parser.advance();
            Ok(grammar::Expr::Literal(grammar::Literal::String(s)))
        }
        TokenType::Literals(Literals::Identifier(s)) => {
            parser.advance();
            Ok(grammar::Expr::Literal(grammar::Literal::Identifier(s)))
        }
        TokenType::Keywords(Keywords::True) => {
            parser.advance();
            Ok(grammar::Expr::Literal(grammar::Literal::Boolean(true)))
        }
        TokenType::Keywords(Keywords::False) => {
            parser.advance();
            Ok(grammar::Expr::Literal(grammar::Literal::Boolean(false)))
        }
        TokenType::Keywords(Keywords::Nil) | TokenType::Literals(Literals::Nil) => {
            parser.advance();
            Ok(grammar::Expr::Literal(grammar::Literal::Nil))
        }
        TokenType::BoundaryTokens(BoundaryTokens::LeftParen) => {
            parser.advance();
            let expr = expression(parser)?;
            parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::RightParen))?;
            Ok(grammar::Expr::Grouping(Box::new(expr)))
        }
        _ => Err(LoxError::ParserErrorExpressionExpected(
            "Primary rule expected an expression".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grammar::{pretty_print, print_lisp},
        reader::Source,
        tokenize::scan,
    };

    fn parse_expr(input: &str) -> grammar::Expr {
        let mut source = Source::new(input.to_string());
        // Test helper .expect("some test string")
        let tokens = scan(&mut source).expect("scan failed");
        // Debug helper
        // println!("{tokens:?}");
        let mut parser = Parser::new(tokens);
        expression(&mut parser).expect("parse failed")
    }

    #[test]
    fn test_number() {
        let expr = parse_expr("5");
        assert_eq!(print_lisp(&expr), "5");
        assert_eq!(pretty_print(&expr), "5");
    }

    #[test]
    fn test_grouped_multiply() {
        // (3 + 4) * 5  =>  lisp: (* (+ 3 4) (group))  =>  "(* (group (+ 3 4)) 5)"
        let expr = parse_expr("(3 + 4) * 5");
        assert_eq!(print_lisp(&expr), "(* (group (+ 3 4)) 5)");
        assert_eq!(pretty_print(&expr), "(((3 + 4)) * 5)");
    }

    #[test]
    fn test_binary_unary_combo() {
        let expr = parse_expr("5 + - 3");
        assert_eq!(print_lisp(&expr), "(+ 5 (- 3))");
        assert_eq!(pretty_print(&expr), "(5 + -3)");
    }

    #[test]
    fn test_eq_chain() {
        // a == b == c == d == e
        let expr = parse_expr("a == b == c == d == e");
        assert_eq!(print_lisp(&expr), "(== (== (== (== a b) c) d) e)");
        assert_eq!(pretty_print(&expr), "((((a == b) == c) == d) == e)");
    }
}
