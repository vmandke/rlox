use crate::{
    errors::LoxError,
    grammar::{BinaryOperator, Expr, InterpretedResult, Literal, UnaryOperator},
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

pub fn interpret(expr: &Expr) -> Result<InterpretedResult, LoxError> {
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::NumberInt(n) => Ok(InterpretedResult::NumberInt(*n)),
            Literal::NumberFloat(n) => Ok(InterpretedResult::NumberFloat(*n)),
            Literal::String(s) => Ok(InterpretedResult::String(s.clone())),
            Literal::Identifier(_s) => todo!(),
            Literal::Boolean(b) => Ok(InterpretedResult::Boolean(*b)),
            Literal::Nil => Ok(InterpretedResult::Nil),
        },
        Expr::Grouping(grpexpr) => interpret(grpexpr),
        Expr::Unary { operator, operand } => match operator {
            UnaryOperator::Minus => unary_minus(interpret(operand)?),
            UnaryOperator::Not => unary_not(interpret(operand)?),
        },
        Expr::Binary {
            operator,
            operand1,
            operand2,
        } => match operator {
            BinaryOperator::BangEqual => {
                binary_not_equal(interpret(operand1)?, interpret(operand2)?)
            }
            BinaryOperator::EqualEqual => {
                binary_equal_equal(interpret(operand1)?, interpret(operand2)?)
            }
            BinaryOperator::LessThan => {
                binary_less_than(interpret(operand1)?, interpret(operand2)?)
            }
            BinaryOperator::LessThanOrEqual => {
                binary_less_than_or_equal(interpret(operand1)?, interpret(operand2)?)
            }
            BinaryOperator::GreaterThan => {
                binary_greater_than(interpret(operand1)?, interpret(operand2)?)
            }
            BinaryOperator::GreaterThanOrEqual => {
                binary_greater_than_or_equal(interpret(operand1)?, interpret(operand2)?)
            }
            BinaryOperator::Plus => binary_plus(interpret(operand1)?, interpret(operand2)?),
            BinaryOperator::Minus => binary_minus(interpret(operand1)?, interpret(operand2)?),
            BinaryOperator::Multiply => binary_multiply(interpret(operand1)?, interpret(operand2)?),
            BinaryOperator::Divide => binary_divide(interpret(operand1)?, interpret(operand2)?),
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
        let mut source = Source::new(input.to_string());
        let tokens = scan(&mut source).expect("scan failed");
        let expr = parse(tokens).expect("parse failed");
        interpret(&expr).expect("interpret failed")
    }

    #[test]
    fn minimal_interpret() {
        let expr = Expr::Unary {
            operator: UnaryOperator::Minus,
            operand: Box::new(Expr::Literal(Literal::NumberInt(3))),
        };
        let result = interpret(&expr).unwrap();
        assert_eq!(result, InterpretedResult::NumberInt(-3));
    }

    #[test]
    fn test_grouped_multiply() {
        // (3 + 4) * 5 = 35
        assert_eq!(
            parse_and_interpret("(3 + 4) * 5"),
            InterpretedResult::NumberInt(35)
        );
    }

    #[test]
    fn test_comparison() {
        assert_eq!(
            parse_and_interpret("3 < 5"),
            InterpretedResult::Boolean(true)
        );
        assert_eq!(
            parse_and_interpret("5 > 3"),
            InterpretedResult::Boolean(true)
        );
        assert_eq!(
            parse_and_interpret("3 == 3"),
            InterpretedResult::Boolean(true)
        );
        assert_eq!(
            parse_and_interpret("3 != 4"),
            InterpretedResult::Boolean(true)
        );
    }
}
