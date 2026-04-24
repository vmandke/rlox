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

use std::rc::Rc;

use crate::{
    errors::LoxError,
    grammar::{self, Stmt},
    parser,
    tokenize::{self, BoundaryTokens, Keywords, Literals, Token, TokenType},
};

pub fn parse(tokens: Vec<tokenize::Token>) -> Result<Vec<grammar::Stmt>, LoxError> {
    let mut parser = Parser::new(tokens);
    program(&mut parser)
}

fn program(parser: &mut Parser) -> Result<Vec<grammar::Stmt>, LoxError> {
    let mut stmts = Vec::new();
    loop {
        match parser.peek() {
            None
            | Some(tokenize::Token {
                token_type: TokenType::Eof,
                ..
            }) => break,
            _ => {}
        }
        let stmt = declaration(parser)?;
        stmts.push(stmt);
    }
    Ok(stmts)
}

/*

program      ->  declaration* EOF ;
declaration  ->  varDecl | statement ;
statement    ->  exprStmt | printStmt | ifStmt | block ;
ifStmt         → "if" "(" expression ")" statement ( "else" statement )? ;
exprStmt     ->  expression ";" ;
printStmt    ->  "print" expression ";" ;
expr         ->  equality
equality     ->  comparison ( ( "!=" | "==" ) comparison )*
comparison   ->  term ( ( ">" | ">=" | "<" | "<=" ) term )
term         ->  factor ( ( "-" | "+" ) factor )*
factor       ->  unary ( ( "/" | "*" ) unary )*
unary        ->  ( "!" | "-" ) unary | primary
primary      ->  NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")"

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

fn declaration(parser: &mut Parser) -> Result<grammar::Stmt, LoxError> {
    // The book also talks of catching the errors 'LoxErrors' here, so
    // then the parser synchronizes (basically goes token by token until
    // valid stmt is found)... Skipping that for now, can add later
    // TODO(vin) ^

    // exprStmt | printStmt ;
    let token_type = match parser.peek() {
        Some(t) => t.token_type.clone(),
        None => {
            return Err(LoxError::ParserErrorStatementExpected(
                "Primary rule expected an expression but found end of input".into(),
            ));
        }
    };
    match token_type {
        TokenType::Keywords(Keywords::Fun) => parse_function_decl(parser),
        TokenType::Keywords(Keywords::Var) => parse_var_decl(parser),
        TokenType::BoundaryTokens(BoundaryTokens::LeftBrace) => {
            parser.advance();
            let mut blk_stmts = Vec::new();
            loop {
                match parser.peek() {
                    None
                    | Some(tokenize::Token {
                        token_type: TokenType::Eof,
                        ..
                    }) => {
                        return Err(LoxError::ParserErrorStatementExpected(
                            "Unterminated block: expected '}'".into(),
                        ));
                    }
                    Some(t)
                        if t.token_type
                            == TokenType::BoundaryTokens(BoundaryTokens::RightBrace) =>
                    {
                        parser.advance();
                        break;
                    }
                    _ => {}
                }
                blk_stmts.push(declaration(parser)?);
            }
            Ok(grammar::Stmt::BlockStmt { blk_stmts })
        }
        _ => statement(parser),
    }
}

fn parse_function_decl(parser: &mut Parser) -> Result<grammar::Stmt, LoxError> {
    // parse fun
    parser.advance();
    // parse identifier
    let name = match parser.peek() {
        Some(t) => match &t.token_type {
            TokenType::Literals(Literals::Identifier(s)) => s.clone(),
            _ => {
                return Err(LoxError::ParserErrorExpressionExpected(
                    "Expected identifier after 'fun'".into(),
                ));
            }
        },
        None => {
            return Err(LoxError::ParserErrorExpressionExpected(
                "Expected identifier after 'fun'".into(),
            ));
        }
    };
    parser.advance();
    // consume '('
    parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::LeftParen))?;
    // parse parameters
    let mut parameters = Vec::new();
    loop {
        // collect parameters until we see a ','
        match parser.peek() {
            Some(t) if t.token_type == TokenType::BoundaryTokens(BoundaryTokens::Comma) => {
                parser.advance();
            }
            Some(t) if t.token_type == TokenType::BoundaryTokens(BoundaryTokens::RightParen) => {
                break;
            }
            Some(t) => match &t.token_type {
                TokenType::Literals(Literals::Identifier(s)) => {
                    parameters.push(s.clone());
                    parser.advance();
                }
                _ => {
                    return Err(LoxError::ParserErrorExpressionExpected(
                        "Expected identifier in parameter list".into(),
                    ));
                }
            },
            None => {
                return Err(LoxError::ParserErrorExpressionExpected(
                    "Expected identifier in parameter list".into(),
                ));
            }
        }
    }
    // consume ')'
    parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::RightParen))?;
    // parse body (block stmt)
    let body = Rc::new(parse_branch(parser)?);
    Ok(grammar::Stmt::FunctionDeclStmt {
        name,
        parameters,
        body,
    })
}

fn parse_var_decl(parser: &mut Parser) -> Result<grammar::Stmt, LoxError> {
    // "var" IDENTIFIER ( "=" expression )? ";" ;
    parser.advance();
    let name = match parser.peek() {
        Some(t) => match &t.token_type {
            TokenType::Literals(Literals::Identifier(s)) => s.clone(),
            _ => {
                return Err(LoxError::ParserErrorExpressionExpected(
                    "Expected identifier after 'var'".into(),
                ));
            }
        },
        None => {
            return Err(LoxError::ParserErrorExpressionExpected(
                "Expected identifier after 'var'".into(),
            ));
        }
    };
    parser.advance();
    let init_expr = match parser.peek() {
        Some(t) if t.token_type == TokenType::BoundaryTokens(BoundaryTokens::Equal) => {
            parser.advance();
            expression(parser)?
        }
        _ => grammar::Expr::Literal(grammar::Literal::Nil),
    };
    parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::Semicolon))?;
    Ok(grammar::Stmt::VarDeclStmt {
        identifier_name: name,
        expr: init_expr,
    })
}

// Same block logic is shared between BlockStmt, IfStmt, and WhileStmt
fn parse_branch(parser: &mut Parser) -> Result<Vec<grammar::Stmt>, LoxError> {
    if let Some(t) = parser.peek() {
        if t.token_type == TokenType::BoundaryTokens(BoundaryTokens::LeftBrace) {
            parser.advance();
            let mut stmts = Vec::new();
            loop {
                match parser.peek() {
                    None
                    | Some(tokenize::Token {
                        token_type: TokenType::Eof,
                        ..
                    }) => {
                        return Err(LoxError::ParserErrorStatementExpected(
                            "Unterminated block: expected '}'".into(),
                        ));
                    }
                    Some(t)
                        if t.token_type
                            == TokenType::BoundaryTokens(BoundaryTokens::RightBrace) =>
                    {
                        parser.advance();
                        break;
                    }
                    _ => {}
                }
                stmts.push(declaration(parser)?);
            }
            return Ok(stmts);
        }
    }
    Ok(vec![declaration(parser)?])
}

fn statement(parser: &mut Parser) -> Result<grammar::Stmt, LoxError> {
    // exprStmt | printStmt ;
    let token_type = match parser.peek() {
        Some(t) => t.token_type.clone(),
        None => {
            return Err(LoxError::ParserErrorStatementExpected(
                "Statement rule expected an expression but found end of input".into(),
            ));
        }
    };
    match token_type {
        TokenType::Keywords(Keywords::Print) => {
            parser.advance();
            let expr = expression(parser)?;
            parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::Semicolon))?;
            Ok(grammar::Stmt::PrintStmt { expr })
        }
        TokenType::Keywords(Keywords::If) => {
            parser.advance();
            parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::LeftParen))?;
            let condition = expression(parser)?;
            parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::RightParen))?;
            let then_branch = parse_branch(parser)?;
            let else_branch = if let Some(token) = parser.peek() {
                if token.token_type == TokenType::Keywords(Keywords::Else) {
                    parser.advance();
                    Some(parse_branch(parser)?)
                } else {
                    None
                }
            } else {
                None
            };
            Ok(grammar::Stmt::IfStmt {
                condition,
                then_branch,
                else_branch,
            })
        }
        TokenType::Keywords(Keywords::While) => {
            parser.advance();
            parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::LeftParen))?;
            let condition = expression(parser)?;
            parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::RightParen))?;
            let body = parse_branch(parser)?;
            Ok(grammar::Stmt::WhileStmt { condition, body })
        }
        TokenType::Keywords(Keywords::For) => {
            parser.advance();
            parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::LeftParen))?;
            // optional initializer statement
            let initializer_stmt = if let Some(token) = parser.peek() {
                match token.token_type {
                    TokenType::BoundaryTokens(BoundaryTokens::Semicolon) => {
                        // consume ';' and set initializer_stmt to None
                        parser.advance();
                        None
                    }
                    TokenType::Keywords(Keywords::Var) => Some(Box::new(parse_var_decl(parser)?)),
                    _ => Some(Box::new(parse_expr_stmt(parser)?)),
                }
            } else {
                None
            };

            // optional condition expression, followed by ';'
            let condition_stmt = if let Some(token) = parser.peek() {
                if token.token_type == TokenType::BoundaryTokens(BoundaryTokens::Semicolon) {
                    parser.advance();
                    None
                } else {
                    let expr = expression(parser)?;
                    parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::Semicolon))?;
                    Some(expr)
                }
            } else {
                None
            };

            // optional increment expression (no ';', ends at ')')
            let increment_stmt = if let Some(token) = parser.peek() {
                if token.token_type == TokenType::BoundaryTokens(BoundaryTokens::RightParen) {
                    None
                } else {
                    let expr = expression(parser)?;
                    Some(Box::new(grammar::Stmt::ExprStmt { expr }))
                }
            } else {
                None
            };

            parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::RightParen))?;
            let body = parse_branch(parser)?;
            Ok(grammar::Stmt::ForStmt {
                initializer_stmt,
                condition: condition_stmt,
                increment_stmt,
                body,
            })
        }
        _ => parse_expr_stmt(parser),
    }
}

fn parse_expr_stmt(parser: &mut Parser) -> Result<grammar::Stmt, LoxError> {
    let expr = expression(parser)?;
    parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::Semicolon))?;
    Ok(grammar::Stmt::ExprStmt { expr })
}

fn expression(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    assignment(parser)
}

fn assignment(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    let expr = logical_or(parser)?;
    let token_type = match parser.peek() {
        Some(t) => t.token_type.clone(),
        None => return Ok(expr),
    };
    match token_type {
        TokenType::BoundaryTokens(BoundaryTokens::Equal) => {
            parser.advance();
            let value = assignment(parser)?;
            match expr {
                grammar::Expr::Variable { name } => Ok(grammar::Expr::Assign {
                    name,
                    expr: Box::new(value),
                }),
                _ => Err(LoxError::ParserErrorExpressionExpected(
                    "Assignment rule expected an Var as lhs".into(),
                )),
            }
        }
        _ => Ok(expr),
    }
}

fn logical_or(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    let mut expr = logical_and(parser)?;
    loop {
        match parser.peek() {
            Some(t) if t.token_type == TokenType::Keywords(Keywords::Or) => {}
            _ => break,
        }
        parser.advance();
        let right = logical_and(parser)?;
        expr = grammar::Expr::LogicalOr {
            operand1: Box::new(expr),
            operand2: Box::new(right),
        };
    }
    Ok(expr)
}

fn logical_and(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    let mut expr = equality(parser)?;
    loop {
        match parser.peek() {
            Some(t) if t.token_type == TokenType::Keywords(Keywords::And) => {}
            _ => break,
        }
        parser.advance();
        let right = equality(parser)?;
        expr = grammar::Expr::LogicalAnd {
            operand1: Box::new(expr),
            operand2: Box::new(right),
        };
    }
    Ok(expr)
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
        None => return call_invoke(parser),
    };
    let op = match token_type {
        TokenType::BoundaryTokens(BoundaryTokens::Bang) => grammar::UnaryOperator::Not,
        TokenType::BoundaryTokens(BoundaryTokens::Minus) => grammar::UnaryOperator::Minus,
        _ => return call_invoke(parser),
    };
    parser.advance();
    let operand = unary(parser)?;
    Ok(grammar::Expr::Unary {
        operator: op,
        operand: Box::new(operand),
    })
}

fn call_invoke(parser: &mut Parser) -> Result<grammar::Expr, LoxError> {
    let mut expr = primary(parser)?;
    loop {
        // Try and match (a,...)(a....).... for invocations
        match parser.peek() {
            Some(t) if t.token_type == TokenType::BoundaryTokens(BoundaryTokens::LeftParen) => {
                // Collect arguments
                parser.advance();
                let mut arguments = Vec::new();
                loop {
                    // while not ')', parse arguments (which are expressions)
                    if let Some(t) = parser.peek() {
                        if t.token_type == TokenType::BoundaryTokens(BoundaryTokens::RightParen) {
                            break;
                        }
                    }
                    let arg = expression(parser)?;
                    arguments.push(arg);
                    // TODO(vin): The book talks of limiting the number of arguments to 255,
                    // Skipping the check for now, can add later

                    // continue in loop if peek for comma else break
                    match parser.peek() {
                        Some(t)
                            if t.token_type == TokenType::BoundaryTokens(BoundaryTokens::Comma) =>
                        {
                            parser.advance();
                        }
                        _ => break,
                    }
                }
                // consume the ')'
                parser.consume(&TokenType::BoundaryTokens(BoundaryTokens::RightParen))?;
                expr = grammar::Expr::InvokeCall {
                    callee: Box::new(expr),
                    arguments,
                };
            }
            _ => break,
        }
    }
    Ok(expr)
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
            Ok(grammar::Expr::Variable { name: s })
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

    #[test]
    fn test_number_less_than_string_parses() {
        // error is caught at runtime, not parse time
        let expr = parse_expr(r#"3 < "pancake""#);
        assert_eq!(print_lisp(&expr), r#"(< 3 "pancake")"#);
    }

    #[test]
    fn test_var_decl() {
        let mut source = Source::new("var a = 5;".to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let mut stmts = parse(tokens).expect("parse failed");
        assert_eq!(
            format!("{:?}", stmts.remove(0)),
            r#"VarDeclStmt { identifier_name: "a", expr: Literal(NumberInt(5)) }"#
        );
    }

    #[test]
    fn test_var_decl_no_init() {
        let mut source = Source::new("var b;".to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let mut stmts = parse(tokens).expect("parse failed");
        assert_eq!(
            format!("{:?}", stmts.remove(0)),
            r#"VarDeclStmt { identifier_name: "b", expr: Literal(Nil) }"#
        );
    }

    #[test]
    fn test_assignment_expr() {
        // a = 5  parses to (= a 5)
        let expr = parse_expr("a = 5");
        assert_eq!(print_lisp(&expr), "(= a 5)");
        assert_eq!(pretty_print(&expr), "a = 5");
    }

    #[test]
    fn test_assignment_right_associative() {
        // a = b = 3  parses to (= a (= b 3))
        let expr = parse_expr("a = b = 3");
        assert_eq!(print_lisp(&expr), "(= a (= b 3))");
    }

    #[test]
    fn test_invalid_assignment_lhs() {
        let mut source = Source::new("5 = 3;".to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let result = parse(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_var_decl_then_assign() {
        let mut source = Source::new("var x = 1; x = 2;".to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let mut stmts = parse(tokens).expect("parse failed");
        assert_eq!(stmts.len(), 2);
        let stmt0 = stmts.remove(0);
        let stmt1 = stmts.remove(0);
        assert_eq!(
            format!("{:?}", stmt0),
            r#"VarDeclStmt { identifier_name: "x", expr: Literal(NumberInt(1)) }"#
        );
        assert_eq!(
            format!("{:?}", stmt1),
            r#"ExprStmt { expr: Assign { name: "x", expr: Literal(NumberInt(2)) } }"#
        );
    }

    #[test]
    fn test_call_no_args() {
        // foo()  =>  foo()
        let expr = parse_expr("foo()");
        assert_eq!(pretty_print(&expr), "foo()");
    }

    #[test]
    fn test_call_with_args() {
        // foo(1, 2)  =>  foo(1, 2)
        let expr = parse_expr("foo(1, 2)");
        assert_eq!(pretty_print(&expr), "foo(1, 2)");
    }

    #[test]
    fn test_chained_call() {
        // foo(1)(2)  =>  foo(1)(2)
        let expr = parse_expr("foo(1)(2)");
        assert_eq!(pretty_print(&expr), "foo(1)(2)");
    }

    #[test]
    fn test_function_decl_no_params() {
        // fun greet() { print "hi"; }
        let mut source = Source::new(r#"fun greet() { print "hi"; }"#.to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let mut stmts = parse(tokens).expect("parse failed");
        assert_eq!(
            format!("{:?}", stmts.remove(0)),
            r#"FunctionDeclStmt { name: "greet", parameters: [], body: [PrintStmt { expr: Literal(String("hi")) }] }"#
        );
    }

    #[test]
    fn test_function_decl_with_params() {
        // fun add(a, b) { print a; }
        let mut source = Source::new("fun add(a, b) { print a; }".to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let mut stmts = parse(tokens).expect("parse failed");
        assert_eq!(
            format!("{:?}", stmts.remove(0)),
            r#"FunctionDeclStmt { name: "add", parameters: ["a", "b"], body: [PrintStmt { expr: Variable { name: "a" } }] }"#
        );
    }

    #[test]
    fn test_if_stmt() {
        let mut source = Source::new("if (true) { var a = 5; }".to_string());
        let tokens = scan(&mut source).expect("scan failed");
        println!("{tokens:?}");
        let stmts = parse(tokens).expect("parse failed");
        assert_eq!(stmts.len(), 1);
    }
}
