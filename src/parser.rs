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

pub struct AST {
    // TODO (vin): Define the structure of the AST here.
}

use crate::{errors::LoxError, tokenize};

pub fn parse(tokens: Vec<tokenize::Token>) -> Result<AST, LoxError> {
    // TODO (vin): Implement the actual parsing logic here.
    // For now, just return an empty AST.
    Ok(AST {})
}
