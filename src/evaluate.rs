use crate::{
    errors::LoxError,
    grammar::{BinaryOperator, Expr, InterpretedResult, Literal, Stmt, UnaryOperator},
    state::Environment,
};
use std::cell::RefCell;
use std::rc::Rc;

//------------ Unary Minus Op ------------
pub fn unary_minus(operand: &InterpretedResult) -> Result<InterpretedResult, LoxError> {
    match operand {
        InterpretedResult::NumberFloat(n) => Ok(InterpretedResult::NumberFloat(-1.0 * n)),
        InterpretedResult::NumberInt(n) => Ok(InterpretedResult::NumberInt(-1 * n)),
        _ => Err(LoxError::InterpretUnaryMinusUndefined(
            "Only Number has additive inverse.".into(),
        )),
    }
}
//------------ Unary Not Op --------------
pub fn unary_not(operand: &InterpretedResult) -> Result<InterpretedResult, LoxError> {
    match operand {
        InterpretedResult::Boolean(b) => Ok(InterpretedResult::Boolean(!b)),
        _ => Err(LoxError::InterpretUnaryNotUndefined(
            "Only Booleans have Truthy negations".into(),
        )),
    }
}

//------------ Binary != Op --------------
pub fn binary_not_equal(
    operand1: &InterpretedResult,
    operand2: &InterpretedResult,
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
    operand1: &InterpretedResult,
    operand2: &InterpretedResult,
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
    operand1: &InterpretedResult,
    operand2: &InterpretedResult,
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
    operand1: &InterpretedResult,
    operand2: &InterpretedResult,
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
    operand1: &InterpretedResult,
    operand2: &InterpretedResult,
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
    operand1: &InterpretedResult,
    operand2: &InterpretedResult,
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
    operand1: &InterpretedResult,
    operand2: &InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::NumberInt(*a + *b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::NumberFloat(*a + *b))
        }
        (InterpretedResult::String(a), InterpretedResult::String(b)) => {
            Ok(InterpretedResult::String(format!("{}{}", a, b)))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            "+ requires two numbers or two strings".into(),
        )),
    }
}
//------------ Binary - Op ---------------
pub fn binary_minus(
    operand1: &InterpretedResult,
    operand2: &InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::NumberInt(*a - *b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::NumberFloat(*a - *b))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            "- requires two numbers".into(),
        )),
    }
}
//------------ Binary * Op ---------------
pub fn binary_multiply(
    operand1: &InterpretedResult,
    operand2: &InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::NumberInt(*a * *b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::NumberFloat(*a * *b))
        }
        _ => Err(LoxError::InterpretBinaryOpUndefined(
            "* requires two numbers".into(),
        )),
    }
}
//------------ Binary / Op ---------------
pub fn binary_divide(
    operand1: &InterpretedResult,
    operand2: &InterpretedResult,
) -> Result<InterpretedResult, LoxError> {
    match (operand1, operand2) {
        (InterpretedResult::NumberInt(a), InterpretedResult::NumberInt(b)) => {
            Ok(InterpretedResult::NumberInt(*a / *b))
        }
        (InterpretedResult::NumberFloat(a), InterpretedResult::NumberFloat(b)) => {
            Ok(InterpretedResult::NumberFloat(*a / *b))
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
            println!("{}", result.borrow());
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
        Stmt::BlockStmt { blk_stmts } => {
            // FIXME (vin):: This needs a scoped env, for now assume
            // only global env is available
            for s in blk_stmts {
                evaluate(s, env)?;
            }
            Ok(())
        }
    }
}

pub fn interpret(
    expr: &Expr,
    env: &mut Environment,
) -> Result<Rc<RefCell<InterpretedResult>>, LoxError> {
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::NumberInt(n) => Ok(Rc::new(RefCell::new(InterpretedResult::NumberInt(*n)))),
            Literal::NumberFloat(n) => {
                Ok(Rc::new(RefCell::new(InterpretedResult::NumberFloat(*n))))
            }
            Literal::String(s) => Ok(Rc::new(RefCell::new(InterpretedResult::String(s.clone())))),
            Literal::Boolean(b) => Ok(Rc::new(RefCell::new(InterpretedResult::Boolean(*b)))),
            Literal::Nil => Ok(Rc::new(RefCell::new(InterpretedResult::Nil))),
        },
        Expr::Variable { name } => match env.get(name) {
            Some(rc) => Ok(rc),
            None => Ok(Rc::new(RefCell::new(InterpretedResult::Nil))),
        },
        Expr::Assign { name, expr } => {
            let value = interpret(expr, env)?;
            env.set(name.clone(), Rc::clone(&value));
            Ok(value)
        }
        Expr::Grouping(grpexpr) => interpret(grpexpr, env),
        Expr::Unary { operator, operand } => {
            let val = interpret(operand, env)?;
            let result = match operator {
                UnaryOperator::Minus => unary_minus(&val.borrow())?,
                UnaryOperator::Not => unary_not(&val.borrow())?,
            };
            Ok(Rc::new(RefCell::new(result)))
        }
        Expr::Binary {
            operator,
            operand1,
            operand2,
        } => {
            let val1 = interpret(operand1, env)?;
            let val2 = interpret(operand2, env)?;
            let result = match operator {
                BinaryOperator::BangEqual => binary_not_equal(&val1.borrow(), &val2.borrow())?,
                BinaryOperator::EqualEqual => binary_equal_equal(&val1.borrow(), &val2.borrow())?,
                BinaryOperator::LessThan => binary_less_than(&val1.borrow(), &val2.borrow())?,
                BinaryOperator::LessThanOrEqual => {
                    binary_less_than_or_equal(&val1.borrow(), &val2.borrow())?
                }
                BinaryOperator::GreaterThan => binary_greater_than(&val1.borrow(), &val2.borrow())?,
                BinaryOperator::GreaterThanOrEqual => {
                    binary_greater_than_or_equal(&val1.borrow(), &val2.borrow())?
                }
                BinaryOperator::Plus => binary_plus(&val1.borrow(), &val2.borrow())?,
                BinaryOperator::Minus => binary_minus(&val1.borrow(), &val2.borrow())?,
                BinaryOperator::Multiply => binary_multiply(&val1.borrow(), &val2.borrow())?,
                BinaryOperator::Divide => binary_divide(&val1.borrow(), &val2.borrow())?,
            };
            Ok(Rc::new(RefCell::new(result)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grammar::{Expr, Literal, UnaryOperator},
        parser::parse,
        reader::Source,
        tokenize::scan,
    };

    fn run_program(input: &str, env: &mut Environment) {
        let mut source = Source::new(input.to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let stmts = parse(tokens).expect("parse failed");
        for stmt in &stmts {
            evaluate(stmt, env).expect("evaluate failed");
        }
    }

    fn parse_and_interpret(input: &str) -> Rc<RefCell<InterpretedResult>> {
        try_parse_and_interpret(input).expect("interpret failed")
    }

    fn parse_stmt(input: &str) -> Result<Stmt, LoxError> {
        let mut source = Source::new(input.to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let mut stmts = parse(tokens).expect("parse failed");
        Ok(stmts.remove(0))
    }

    fn evaluate_stmt(input: &str) -> Result<(), LoxError> {
        let stmt = parse_stmt(input)?;
        let mut env = Environment::new();
        evaluate(&stmt, &mut env)?;
        Ok(())
    }

    fn try_parse_and_interpret(input: &str) -> Result<Rc<RefCell<InterpretedResult>>, LoxError> {
        let stmt = parse_stmt(input)?;
        let mut env = Environment::new();
        match stmt {
            Stmt::ExprStmt { expr } => interpret(&expr, &mut env),
            Stmt::PrintStmt { expr } => interpret(&expr, &mut env),
            Stmt::VarDeclStmt { expr, .. } => interpret(&expr, &mut env),
            _ => Err(LoxError::RuntimeLoxError(
                "Helper only used for non block stmts ".into(),
            )),
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
        assert_eq!(*result.borrow(), InterpretedResult::NumberInt(-3));
    }

    #[test]
    fn test_grouped_multiply() {
        // (3 + 4) * 5 = 35
        assert_eq!(
            *parse_and_interpret("(3 + 4) * 5;").borrow(),
            InterpretedResult::NumberInt(35)
        );
    }

    #[test]
    fn test_comparison() {
        assert_eq!(
            *parse_and_interpret("3 < 5;").borrow(),
            InterpretedResult::Boolean(true)
        );
        assert_eq!(
            *parse_and_interpret("5 > 3;").borrow(),
            InterpretedResult::Boolean(true)
        );
        assert_eq!(
            *parse_and_interpret("3 == 3;").borrow(),
            InterpretedResult::Boolean(true)
        );
        assert_eq!(
            *parse_and_interpret("3 != 4;").borrow(),
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
        assert_eq!(
            *env.get("b").unwrap().borrow(),
            InterpretedResult::NumberInt(6)
        );
        assert_eq!(
            *env.get("c").unwrap().borrow(),
            InterpretedResult::NumberInt(18)
        );
    }

    #[test]
    fn test_block_stmt() {
        let mut env = Environment::new();
        let stmt = parse_stmt("{ var x = 10; var y = x + 5; }").expect("parse failed");
        evaluate(&stmt, &mut env).expect("evaluate failed");
        // FIXME (vin):: Only global env, x, y should not be visible outside
        assert_eq!(
            *env.get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(10)
        );
        assert_eq!(
            *env.get("y").unwrap().borrow(),
            InterpretedResult::NumberInt(15)
        );
    }

    #[test]
    fn test_nested_block_stmt() {
        let mut env = Environment::new();
        let stmt = parse_stmt("{ var a = 2; { var b = a * 3; } }").expect("parse failed");
        evaluate(&stmt, &mut env).expect("evaluate failed");
        // FIXME (vin):: Only global env
        // a and b should not be visible ideally
        assert_eq!(
            *env.get("a").unwrap().borrow(),
            InterpretedResult::NumberInt(2)
        );
        assert_eq!(
            *env.get("b").unwrap().borrow(),
            InterpretedResult::NumberInt(6)
        );
    }

    #[test]
    fn test_global_and_block() {
        let mut env = Environment::new();
        let global = parse_stmt("var g = 100;").expect("parse failed");
        evaluate(&global, &mut env).expect("evaluate failed");
        let block = parse_stmt("{ var local = g + 1; }").expect("parse failed");
        evaluate(&block, &mut env).expect("evaluate failed");
        assert_eq!(
            *env.get("g").unwrap().borrow(),
            InterpretedResult::NumberInt(100)
        );
        // FIXME (vin):: Only global env
        // local should not be visible ideally
        assert_eq!(
            *env.get("local").unwrap().borrow(),
            InterpretedResult::NumberInt(101)
        );
    }

    #[test]
    fn test_var_decl_then_assign() {
        let mut env = Environment::new();
        let decl = parse_stmt("var x = 10;").expect("parse failed");
        evaluate(&decl, &mut env).expect("evaluate failed");
        assert_eq!(
            *env.get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(10)
        );

        let assign = parse_stmt("x = 99;").expect("parse failed");
        evaluate(&assign, &mut env).expect("evaluate failed");
        assert_eq!(
            *env.get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(99)
        );
    }

    #[test]
    fn test_assign_uses_rhs_expr() {
        let mut env = Environment::new();
        run_program("var a = 4; var b = 3; a = a * b;", &mut env);
        assert_eq!(
            *env.get("a").unwrap().borrow(),
            InterpretedResult::NumberInt(12)
        );
        assert_eq!(
            *env.get("b").unwrap().borrow(),
            InterpretedResult::NumberInt(3)
        );
    }

    #[test]
    fn test_chained_assign() {
        // a = b = c
        let mut env = Environment::new();
        run_program("var a = 0; var b = 0; var c = 7; a = b = c;", &mut env);
        assert_eq!(
            *env.get("c").unwrap().borrow(),
            InterpretedResult::NumberInt(7)
        );
        assert_eq!(
            *env.get("b").unwrap().borrow(),
            InterpretedResult::NumberInt(7)
        );
        assert_eq!(
            *env.get("a").unwrap().borrow(),
            InterpretedResult::NumberInt(7)
        );
    }
}
