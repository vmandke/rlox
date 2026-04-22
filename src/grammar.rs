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

/*
NOTE::
This grammar currently only supports *valid* expressions.

Thoughts after reading chapter 5:
? Parser handles invalid expressions.
After reading chapter 5, I was confused as what exactly this grammar handles.
I thought it would describe the entire language.
However assignments, variables, control flow etc are not described here, and are handled later chapters.
*/

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

impl UnaryOperator {
    fn to_str(&self) -> &'static str {
        match self {
            UnaryOperator::Minus => "-",
            UnaryOperator::Not => "!",
        }
    }
}

// &self :: borrows enum value reference, without consuming
// return -> &'static str => reference to literals that live for prog duration.
impl BinaryOperator {
    fn to_str(&self) -> &'static str {
        match self {
            BinaryOperator::EqualEqual => "==",
            BinaryOperator::BangEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterThanOrEqual => ">=",
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
        }
    }
}

pub fn print_lisp(expr: &Expr) -> String {
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::Number(n) => n.to_string(),
            // Verify:: If the printer consumes the string, should it ??
            // ! invokes the format macro; Also create copies as this would be
            // consumed by the display / println
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
        },
        Expr::Grouping(grpexpr) => format!("(group {})", print_lisp(grpexpr)),
        Expr::Unary { operator, operand } => {
            format!("({} {})", operator.to_str(), print_lisp(operand))
        }
        Expr::Binary {
            operator,
            operand1,
            operand2,
        } => {
            format!(
                "({} {} {})",
                operator.to_str(),
                print_lisp(operand1),
                print_lisp(operand2)
            )
        }
    }
}

pub fn pretty_print(expr: &Expr) -> String {
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => format!("\"{}\"", s),
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
        },
        Expr::Grouping(grpexpr) => format!("({})", pretty_print(grpexpr)),
        Expr::Unary { operator, operand } => {
            format!("{}{}", operator.to_str(), pretty_print(operand))
        }
        Expr::Binary {
            operator,
            operand1,
            operand2,
        } => {
            format!(
                "({} {} {})",
                pretty_print(operand1),
                operator.to_str(),
                pretty_print(operand2)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_from_book() {
        //(* (- 123) (group 45.67))
        let expr = Expr::Binary {
            operator: BinaryOperator::Multiply,
            operand1: Box::new(Expr::Unary {
                operator: UnaryOperator::Minus,
                operand: Box::new(Expr::Literal(Literal::Number(123.0))),
            }),
            operand2: Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
                45.67,
            ))))),
        };
        assert_eq!(print_lisp(&expr), "(* (- 123) (group 45.67))");
        assert_eq!(pretty_print(&expr), "(-123 * (45.67))")
    }
}
