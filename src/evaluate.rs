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

pub fn evaluate_block_stmt(
    stmts: &Vec<Stmt>,
    env: Rc<RefCell<Environment>>,
) -> Result<(), LoxError> {
    let new_env = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(&env))));
    for s in stmts {
        evaluate(s, Rc::clone(&new_env))?;
    }
    Ok(())
}

pub fn evaluate(stmt: &Stmt, env: Rc<RefCell<Environment>>) -> Result<(), LoxError> {
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
            let value = interpret(expr, Rc::clone(&env))?;
            env.borrow_mut().set(identifier_name.clone(), value);
            Ok(())
        }
        Stmt::BlockStmt { blk_stmts } => {
            let new_env = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(&env))));
            for s in blk_stmts {
                evaluate(s, Rc::clone(&new_env))?;
            }
            Ok(())
        }
        Stmt::NoopStmt => Ok(()),
        Stmt::WhileStmt { condition, body } => {
            loop {
                let result = interpret(condition, Rc::clone(&env))?;
                // TODO(vin): Should while loop get a new env on each iteration?
                if *result.borrow() != InterpretedResult::Boolean(true) {
                    break;
                }
                evaluate_block_stmt(body, Rc::clone(&env))?;
            }
            Ok(())
        }
        Stmt::ForStmt {
            initializer_stmt,
            condition,
            increment_stmt,
            body,
        } => {
            if let Some(stmt) = initializer_stmt {
                // eval initializer in parent env
                evaluate(stmt, Rc::clone(&env))?;
            }
            loop {
                // condition check, break if condition is false
                if let Some(cond_expr) = condition {
                    let result = interpret(cond_expr, Rc::clone(&env))?;
                    if *result.borrow() != InterpretedResult::Boolean(true) {
                        break;
                    }
                }
                // eval body in new env enclosed by parent env
                evaluate_block_stmt(body, Rc::clone(&env))?;
                // eval increment in parent env after body
                if let Some(incr_stmt) = increment_stmt {
                    evaluate(incr_stmt, Rc::clone(&env))?;
                }
            }
            Ok(())
        }
        Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        } => {
            let result = interpret(condition, Rc::clone(&env))?;
            if *result.borrow() == InterpretedResult::Boolean(true) {
                evaluate_block_stmt(then_branch, Rc::clone(&env))?;
            } else if let Some(else_stmts) = else_branch {
                evaluate_block_stmt(else_stmts, Rc::clone(&env))?;
            }
            Ok(())
        }
        Stmt::FunctionDeclStmt {
            name,
            parameters,
            body,
        } => {
            // TODO(vin): Implement function declarations
            todo!()
        }
    }
}

pub fn interpret(
    expr: &Expr,
    env: Rc<RefCell<Environment>>,
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
        Expr::Variable { name } => match env.borrow().get(name) {
            Some(rc) => Ok(rc),
            None => Ok(Rc::new(RefCell::new(InterpretedResult::Nil))),
        },
        Expr::Assign { name, expr } => {
            let value = interpret(expr, Rc::clone(&env))?;
            if !env.borrow_mut().assign(name, Rc::clone(&value)) {
                return Err(LoxError::RuntimeLoxError(format!(
                    "Undefined variable '{}'.",
                    name
                )));
            }
            Ok(value)
        }
        Expr::Grouping(grpexpr) => interpret(grpexpr, env),
        Expr::LogicalOr { operand1, operand2 } => {
            let left = interpret(operand1, Rc::clone(&env))?;
            if *left.borrow() == InterpretedResult::Boolean(true) {
                // In Or if left is true, return left without evaluating right
                return Ok(left);
            }
            // Evaluate right if left is false
            interpret(operand2, env)
        }
        Expr::LogicalAnd { operand1, operand2 } => {
            let left = interpret(operand1, Rc::clone(&env))?;
            if *left.borrow() == InterpretedResult::Boolean(false) {
                // If left is false, return left without evaluating right
                return Ok(left);
            }
            // Evaluate right if left is true
            interpret(operand2, env)
        }
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
            let val1 = interpret(operand1, Rc::clone(&env))?;
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
        Expr::InvokeCall {
            callee: _,
            arguments: _,
        } => {
            // TODO (vin): Implement function calls
            todo!()
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

    fn make_env() -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment::new()))
    }

    fn run_program(input: &str, env: Rc<RefCell<Environment>>) {
        let mut source = Source::new(input.to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let stmts = parse(tokens).expect("parse failed");
        for stmt in &stmts {
            evaluate(stmt, Rc::clone(&env)).expect("evaluate failed");
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
        evaluate(&stmt, make_env())?;
        Ok(())
    }

    fn try_parse_and_interpret(input: &str) -> Result<Rc<RefCell<InterpretedResult>>, LoxError> {
        let stmt = parse_stmt(input)?;
        let env = make_env();
        match stmt {
            Stmt::ExprStmt { expr } => interpret(&expr, env),
            Stmt::PrintStmt { expr } => interpret(&expr, env),
            Stmt::VarDeclStmt { expr, .. } => interpret(&expr, env),
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
        let result = interpret(&expr, make_env()).unwrap();
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
        let env = make_env();
        let stmt_b = parse_stmt("var b = 6;").expect("parse failed");
        evaluate(&stmt_b, Rc::clone(&env)).expect("evaluate failed");
        let stmt_c = parse_stmt("var c = b * 3;").expect("parse failed");
        evaluate(&stmt_c, Rc::clone(&env)).expect("evaluate failed");
        assert_eq!(
            *env.borrow().get("b").unwrap().borrow(),
            InterpretedResult::NumberInt(6)
        );
        assert_eq!(
            *env.borrow().get("c").unwrap().borrow(),
            InterpretedResult::NumberInt(18)
        );
    }

    #[test]
    fn test_block_stmt() {
        let env = make_env();
        let stmt = parse_stmt("{ var x = 10; var y = x + 5; }").expect("parse failed");
        evaluate(&stmt, Rc::clone(&env)).expect("evaluate failed");
        // x, y are block-scoped — not visible in outer env
        assert!(env.borrow().get("x").is_none());
        assert!(env.borrow().get("y").is_none());
    }

    #[test]
    fn test_nested_block_stmt() {
        let env = make_env();
        let stmt = parse_stmt("{ var a = 2; { var b = a * 3; } }").expect("parse failed");
        evaluate(&stmt, Rc::clone(&env)).expect("evaluate failed");
        // a and b are not visible in outer env
        assert!(env.borrow().get("a").is_none());
        assert!(env.borrow().get("b").is_none());
    }

    #[test]
    fn test_global_and_block() {
        let env = make_env();
        let global = parse_stmt("var g = 100;").expect("parse failed");
        evaluate(&global, Rc::clone(&env)).expect("evaluate failed");
        let block = parse_stmt("{ var local = g + 1; }").expect("parse failed");
        evaluate(&block, Rc::clone(&env)).expect("evaluate failed");
        assert_eq!(
            *env.borrow().get("g").unwrap().borrow(),
            InterpretedResult::NumberInt(100)
        );
        // local is not visible in outer env
        assert!(env.borrow().get("local").is_none());
    }

    #[test]
    fn test_var_decl_then_assign() {
        let env = make_env();
        let decl = parse_stmt("var x = 10;").expect("parse failed");
        evaluate(&decl, Rc::clone(&env)).expect("evaluate failed");
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(10)
        );

        let assign = parse_stmt("x = 99;").expect("parse failed");
        evaluate(&assign, Rc::clone(&env)).expect("evaluate failed");
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(99)
        );
    }

    #[test]
    fn test_assign_uses_rhs_expr() {
        let env = make_env();
        run_program("var a = 4; var b = 3; a = a * b;", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("a").unwrap().borrow(),
            InterpretedResult::NumberInt(12)
        );
        assert_eq!(
            *env.borrow().get("b").unwrap().borrow(),
            InterpretedResult::NumberInt(3)
        );
    }

    #[test]
    fn test_block_assign_updates_outer() {
        // a = 2 inside block should update outer a
        let env = make_env();
        run_program("var a = 1; { a = 2; }", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("a").unwrap().borrow(),
            InterpretedResult::NumberInt(2)
        );
    }

    #[test]
    fn test_block_var_decl_shadows_outer() {
        // var a unchanged in outer
        let env = make_env();
        run_program("var a = 1; { var a = 2; }", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("a").unwrap().borrow(),
            InterpretedResult::NumberInt(1)
        );
    }

    #[test]
    fn test_chained_assign() {
        // a = b = c
        let env = make_env();
        run_program(
            "var a = 0; var b = 0; var c = 7; a = b = c;",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("c").unwrap().borrow(),
            InterpretedResult::NumberInt(7)
        );
        assert_eq!(
            *env.borrow().get("b").unwrap().borrow(),
            InterpretedResult::NumberInt(7)
        );
        assert_eq!(
            *env.borrow().get("a").unwrap().borrow(),
            InterpretedResult::NumberInt(7)
        );
    }

    #[test]
    fn test_if_true_branch_runs() {
        let env = make_env();
        run_program("var x = 0; if (true) { x = 1; }", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(1)
        );
    }

    #[test]
    fn test_if_false_branch_skipped() {
        let env = make_env();
        run_program("var x = 0; if (false) { x = 1; }", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(0)
        );
    }

    #[test]
    fn test_if_else_true() {
        let env = make_env();
        run_program(
            "var x = 0; if (true) { x = 1; } else { x = 2; }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(1)
        );
    }

    #[test]
    fn test_if_else_false() {
        let env = make_env();
        run_program(
            "var x = 0; if (false) { x = 1; } else { x = 2; }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(2)
        );
    }

    #[test]
    fn test_if_condition_expression() {
        let env = make_env();
        run_program(
            "var a = 5; var b = 0; if (a > 3) { b = 10; }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("b").unwrap().borrow(),
            InterpretedResult::NumberInt(10)
        );
    }

    #[test]
    fn test_if_branch_scope_does_not_leak() {
        let env = make_env();
        run_program("if (true) { var inner = 99; }", Rc::clone(&env));
        assert!(env.borrow().get("inner").is_none());
    }

    #[test]
    fn test_nested_if() {
        let env = make_env();
        run_program(
            "var x = 0; if (true) { if (true) { x = 42; } }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(42)
        );
    }

    #[test]
    fn test_nested_if_else_inner_else_taken() {
        // outer true, inner false -> inner else runs
        let env = make_env();
        run_program(
            "var x = 0; if (true) { if (false) { x = 1; } else { x = 2; } }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(2)
        );
    }

    #[test]
    fn test_nested_if_else_inner_else_not_taken() {
        // outer true, inner false -> inner else runs
        let env = make_env();
        run_program(
            "var x = 0; if (false) { if (false) { x = 1; } else { x = 2; } }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(0)
        );
    }

    #[test]
    fn test_if_else_chains_outer_else() {
        // outer false -> outer else runs, which itself has an if/else
        let env = make_env();
        run_program(
            "var x = 0; if (false) { x = 1; } else { if (true) { x = 3; } else { x = 4; } }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(3)
        );
    }

    #[test]
    fn test_logical_and_both_true() {
        let env = make_env();
        run_program("var x = 0; if (true and true) { x = 1; }", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(1)
        );
    }

    #[test]
    fn test_logical_and_one_false() {
        let env = make_env();
        run_program("var x = 0; if (true and false) { x = 1; }", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(0)
        );
    }

    #[test]
    fn test_logical_or_one_true() {
        let env = make_env();
        run_program("var x = 0; if (false or true) { x = 1; }", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(1)
        );
    }

    #[test]
    fn test_logical_or_both_false() {
        let env = make_env();
        run_program("var x = 0; if (false or false) { x = 1; }", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(0)
        );
    }

    #[test]
    fn test_logical_and_short_circuits() {
        // right side never evaluated if left is false
        let env = make_env();
        run_program(
            "var x = 0; if (false and true) { x = 1; } else { x = 2; }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(2)
        );
    }

    #[test]
    fn test_logical_or_short_circuits() {
        // right side never evaluated if left is true
        let env = make_env();
        run_program(
            "var x = 0; if (true or false) { x = 1; } else { x = 2; }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(1)
        );
    }

    #[test]
    fn test_logical_complex_and_or() {
        // (true and false) or true => true
        let env = make_env();
        run_program(
            "var x = 0; if ((true and false) or true) { x = 1; }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(1)
        );
    }

    #[test]
    fn test_logical_with_comparison() {
        // a > 2 and b < 10
        let env = make_env();
        run_program(
            "var a = 5; var b = 3; var x = 0; if (a > 2 and b < 10) { x = 1; }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(1)
        );
    }

    #[test]
    fn test_while_basic_count() {
        let env = make_env();
        run_program("var i = 0; while (i < 3) { i = i + 1; }", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("i").unwrap().borrow(),
            InterpretedResult::NumberInt(3)
        );
    }

    #[test]
    fn test_while_body_never_runs() {
        let env = make_env();
        run_program("var x = 5; while (false) { x = 99; }", Rc::clone(&env));
        assert_eq!(
            *env.borrow().get("x").unwrap().borrow(),
            InterpretedResult::NumberInt(5)
        );
    }

    #[test]
    fn test_while_accumulates() {
        let env = make_env();
        run_program(
            "var i = 0; var sum = 0; while (i < 5) { sum = sum + i; i = i + 1; }",
            Rc::clone(&env),
        );
        // 0+1+2+3+4 = 10
        assert_eq!(
            *env.borrow().get("sum").unwrap().borrow(),
            InterpretedResult::NumberInt(10)
        );
    }

    #[test]
    fn test_for_fibonacci() {
        let env = make_env();
        run_program(
            "var a = 0; var temp; for (var b = 1; a < 10000; b = temp + b) { temp = a; a = b; }",
            Rc::clone(&env),
        );
        // Fibonacci sequence stops when a reaches 10946 (first value >= 10000)
        assert_eq!(
            *env.borrow().get("a").unwrap().borrow(),
            InterpretedResult::NumberInt(10946)
        );
    }

    #[test]
    fn test_while_with_if_inside() {
        let env = make_env();
        run_program(
            "var i = 0; var evens = 0; while (i < 6) { if (i == 2 or i == 4) { evens = evens + 1; } i = i + 1; }",
            Rc::clone(&env),
        );
        assert_eq!(
            *env.borrow().get("evens").unwrap().borrow(),
            InterpretedResult::NumberInt(2)
        );
    }
}
