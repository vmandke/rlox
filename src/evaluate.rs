use crate::{
    errors::LoxError,
    grammar::{BinaryOperator, Expr, InterpretedResult, Literal, Stmt, UnaryOperator},
    state::Environment,
};

//------------ Unary Minus Op ------------
pub fn unary_minus(operand: InterpretedResult) -> Result<InterpretedResult, LoxError> {
    match operand {
        InterpretedResult::NumberFloat(n) => Ok(InterpretedResult::NumberFloat(-1.0 * n)),
        InterpretedResult::NumberInt(n) => Ok(InterpretedResult::NumberInt(-1 * n)),
        _ => Err(LoxError::InterpretUnaryMinusUndefined(
            "Only Number has additive inverse.".into(),
        )),
    }
}
//------------ Unary Not Op --------------
pub fn unary_not(operand: InterpretedResult) -> Result<InterpretedResult, LoxError> {
    match operand {
        InterpretedResult::Boolean(b) => Ok(InterpretedResult::Boolean(!b)),
        _ => Err(LoxError::InterpretUnaryNotUndefined(
            "Only Booleans have Truthy negations".into(),
        )),
    }
}

//------------ Binary != Op --------------
pub fn binary_not_equal(
    operand1: InterpretedResult,
    operand2: InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    let result = match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => a != b,
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => a != b,
        (InterpretedResult::Boolean(a), InterpretedResult::Boolean(b)) => a != b,
        (InterpretedResult::String(a), InterpretedResult::String(b)) => a != b,
        (InterpretedResult::Nil, InterpretedResult::Nil) => false,
        _ => true,
    };
    Ok(InterpretedResult::Boolean(result))
}
//------------ Binary == Op --------------
pub fn binary_equal_equal(
    operand1: InterpretedResult,
    operand2: InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    let result = match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => a == b,
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => a == b,
        (InterpretedResult::Boolean(a), InterpretedResult::Boolean(b)) => a == b,
        (InterpretedResult::String(a), InterpretedResult::String(b)) => a == b,
        (InterpretedResult::Nil, InterpretedResult::Nil) => true,
        _ => false,
    };
    Ok(InterpretedResult::Boolean(result))
}
//------------ Binary < Op ---------------
pub fn binary_less_than(
    operand1: InterpretedResult,
    operand2: InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::Boolean(a < b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::Boolean(a < b))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            "< requires two numbers".into(),
        )),
    }
}
//------------ Binary <= Op --------------
pub fn binary_less_than_or_equal(
    operand1: InterpretedResult,
    operand2: InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::Boolean(a <= b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::Boolean(a <= b))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            "<= requires two numbers".into(),
        )),
    }
}
//------------ Binary > Op ---------------
pub fn binary_greater_than(
    operand1: InterpretedResult,
    operand2: InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::Boolean(a > b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::Boolean(a > b))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            "> requires two numbers".into(),
        )),
    }
}
//------------ Binary >= Op --------------
pub fn binary_greater_than_or_equal(
    operand1: InterpretedResult,
    operand2: InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::Boolean(a >= b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::Boolean(a >= b))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            ">= requires two numbers".into(),
        )),
    }
}
//------------ Binary + Op ---------------
pub fn binary_plus(
    operand1: InterpretedResult,
    operand2: InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::NumberInt(a + b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::NumberFloat(a + b))
        }
        (InterpretedResult::String(a), InterpretedResult::String(b)) => {
            Ok(InterpretedResult::String(a + &b))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            "+ requires two numbers or two strings".into(),
        )),
    }
}
//------------ Binary - Op ---------------
pub fn binary_minus(
    operand1: InterpretedResult,
    operand2: InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::NumberInt(a - b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::NumberFloat(a - b))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            "- requires two numbers".into(),
        )),
    }
}
//------------ Binary * Op ---------------
pub fn binary_multiply(
    operand1: InterpretedResult,
    operand2: InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::NumberInt(a * b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::NumberFloat(a * b))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            "* requires two numbers".into(),
        )),
    }
}
//------------ Binary / Op ---------------
pub fn binary_divide(
    operand1: InterpretedResult,
    operand2: InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::NumberInt(a / b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::NumberFloat(a / b))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            "/ requires two numbers".into(),
        )),
    }
}

pub fn evaluate(stmt: &Stmt, env: &mut Environment) -> Result<(), LoxError> {
    match stmt {
        Stmt::PrintStmt { expr } => {
            let result = interpret(expr, env)?;
            println!("{}", result);
            Ok(())
        }
        Stmt::ExprStmt { expr } => {
            interpret(expr, env)?;
            Ok(())
        }
        Stmt::VarDeclStmt {
            identifier_name,
            expr,
        } => {
            let value = interpret(expr, env)?;
            env.set(identifier_name.clone(), value);
            Ok(())
        }
    }
}

pub fn interpret(expr: &Expr, env: &mut Environment) -> Result<InterpretedResult, LoxError> {
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::NumberInt(n) => Ok(InterpretedResult::NumberInt(*n)),
            Literal::NumberFloat(n) => Ok(InterpretedResult::NumberFloat(*n)),
            Literal::String(s) => Ok(InterpretedResult::String(s.clone())),
            Literal::Identifier(s) => Ok(env.get(s)),
            Literal::Boolean(b) => Ok(InterpretedResult::Boolean(*b)),
            Literal::Nil => Ok(InterpretedResult::Nil),
        },
        Expr::Grouping(grpexpr) => interpret(grpexpr, env),
        Expr::Unary { operator, operand } => match operator {
            UnaryOperator::Minus => unary_minus(interpret(operand, env)?),
            UnaryOperator::Not => unary_not(interpret(operand, env)?),
        },
        Expr::Binary {
            operator,
            operand1,
            operand2,
        } => match operator {
            BinaryOperator::BangEqual => {
                binary_not_equal(interpret(operand1, env)?, interpret(operand2, env)?)
            }
            BinaryOperator::EqualEqual => {
                binary_equal_equal(interpret(operand1, env)?, interpret(operand2, env)?)
            }
            BinaryOperator::LessThan => {
                binary_less_than(interpret(operand1, env)?, interpret(operand2, env)?)
            }
            BinaryOperator::LessThanOrEqual => {
                binary_less_than_or_equal(interpret(operand1, env)?, interpret(operand2, env)?)
            }
            BinaryOperator::GreaterThan => {
                binary_greater_than(interpret(operand1, env)?, interpret(operand2, env)?)
            }
            BinaryOperator::GreaterThanOrEqual => {
                binary_greater_than_or_equal(interpret(operand1, env)?, interpret(operand2, env)?)
            }
            BinaryOperator::Plus => {
                binary_plus(interpret(operand1, env)?, interpret(operand2, env)?)
            }
            BinaryOperator::Minus => {
                binary_minus(interpret(operand1, env)?, interpret(operand2, env)?)
            }
            BinaryOperator::Multiply => {
                binary_multiply(interpret(operand1, env)?, interpret(operand2, env)?)
            }
            BinaryOperator::Divide => {
                binary_divide(interpret(operand1, env)?, interpret(operand2, env)?)
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grammar::{BinaryOperator, Expr, Literal, UnaryOperator},
        parser::parse,
        reader::Source,
        tokenize::scan,
    };

    fn parse_and_interpret(input: &str) -> InterpretedResult {
        try_parse_and_interpret(input).expect("interpret failed")
    }

    fn parse_stmt(input: &str) -> Result<Stmt, LoxError> {
        let mut source = Source::new(input.to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let stmt = parse(tokens).expect("parse failed");
        Ok(stmt)
    }

    fn evaluate_stmt(input: &str) -> Result<(), LoxError> {
        let stmt = parse_stmt(input)?;
        let mut env = Environment::new();
        evaluate(&stmt, &mut env)?;
        Ok(())
    }

    fn try_parse_and_interpret(input: &str) -> Result<InterpretedResult, LoxError> {
        let stmt = parse_stmt(input)?;
        let mut env = Environment::new();
        match stmt {
            Stmt::ExprStmt { expr } => interpret(&expr, &mut env),
            Stmt::PrintStmt { expr } => interpret(&expr, &mut env),
            Stmt::VarDeclStmt {
                identifier_name,
                expr,
            } => interpret(&expr, &mut env),
        }
    }

    #[test]
    fn minimal_interpret() {
        let expr = Expr::Unary {
            operator: UnaryOperator::Minus,
            operand: Box::new(Expr::Literal(Literal::NumberInt(3))),
        };
        let mut env = Environment::new();
        let result = interpret(&expr, &mut env).unwrap();
        assert_eq!(result, InterpretedResult::NumberInt(-3));
    }

    #[test]
    fn test_grouped_multiply() {
        // (3 + 4) * 5 = 35
        assert_eq!(
            parse_and_interpret("(3 + 4) * 5;"),
            InterpretedResult::NumberInt(35)
        );
    }

    #[test]
    fn test_comparison() {
        assert_eq!(
            parse_and_interpret("3 < 5;"),
            InterpretedResult::Boolean(true)
        );
        assert_eq!(
            parse_and_interpret("5 > 3;"),
            InterpretedResult::Boolean(true)
        );
        assert_eq!(
            parse_and_interpret("3 == 3;"),
            InterpretedResult::Boolean(true)
        );
        assert_eq!(
            parse_and_interpret("3 != 4;"),
            InterpretedResult::Boolean(true)
        );
    }

    #[test]
    fn test_type_error_less_than_string() {
        // 3 < "pancake" should be a runtime type error
        let result = try_parse_and_interpret(r#"3 < "pancake";"#);
        assert!(result.is_err());
    }

    #[test]
    fn test_print_statements() {
        let result = evaluate_stmt(r#"print "one";"#);
        assert!(result.is_ok());
    }

    #[test]
    fn test_var_assignment() {
        let result = evaluate_stmt("var a = 5;");
        assert!(result.is_ok());
    }

    #[test]
    fn test_var_lookup_across_stmts() {
        let mut env = Environment::new();
        let stmt_b = parse_stmt("var b = 6;").expect("parse failed");
        evaluate(&stmt_b, &mut env).expect("evaluate failed");
        let stmt_c = parse_stmt("var c = b * 3;").expect("parse failed");
        evaluate(&stmt_c, &mut env).expect("evaluate failed");
        assert_eq!(env.get("b"), InterpretedResult::NumberInt(6));
        assert_eq!(env.get("c"), InterpretedResult::NumberInt(18));
    }
}
