use std::{collections::HashMap, sync::LazyLock};

use crate::{errors::LoxError, reader};

#[derive(Clone, Debug, PartialEq)]
pub enum Keywords {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literals {
    Identifier,
    String,
    Number,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BoundaryTokens {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    BoundaryTokens(BoundaryTokens),
    // Literals.
    Literals(Literals),
    // Keywords.
    Keywords(Keywords),
    // End of file.
    Eof,
}

static KEYWORDS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("and",    TokenType::Keywords(Keywords::And));
    m.insert("class",  TokenType::Keywords(Keywords::Class));
    m.insert("else",   TokenType::Keywords(Keywords::Else));
    m.insert("false",  TokenType::Keywords(Keywords::False));
    m.insert("fun",    TokenType::Keywords(Keywords::Fun));
    m.insert("for",    TokenType::Keywords(Keywords::For));
    m.insert("if",     TokenType::Keywords(Keywords::If));
    m.insert("nil",    TokenType::Keywords(Keywords::Nil));
    m.insert("or",     TokenType::Keywords(Keywords::Or));
    m.insert("print",  TokenType::Keywords(Keywords::Print));
    m.insert("return", TokenType::Keywords(Keywords::Return));
    m.insert("super",  TokenType::Keywords(Keywords::Super));
    m.insert("this",   TokenType::Keywords(Keywords::This));
    m.insert("true",   TokenType::Keywords(Keywords::True));
    m.insert("var",    TokenType::Keywords(Keywords::Var));
    m.insert("while",  TokenType::Keywords(Keywords::While));
    m
});


static BOUNDARY_TOKENS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("(", TokenType::BoundaryTokens(BoundaryTokens::LeftParen));
    m.insert(")", TokenType::BoundaryTokens(BoundaryTokens::RightParen));
    m.insert("{", TokenType::BoundaryTokens(BoundaryTokens::LeftBrace));
    m.insert("}", TokenType::BoundaryTokens(BoundaryTokens::RightBrace));
    m.insert(",", TokenType::BoundaryTokens(BoundaryTokens::Comma));
    m.insert(".", TokenType::BoundaryTokens(BoundaryTokens::Dot));
    m.insert("-", TokenType::BoundaryTokens(BoundaryTokens::Minus));
    m.insert("+", TokenType::BoundaryTokens(BoundaryTokens::Plus));
    m.insert(";", TokenType::BoundaryTokens(BoundaryTokens::Semicolon));
    m.insert("/", TokenType::BoundaryTokens(BoundaryTokens::Slash));
    m.insert("*", TokenType::BoundaryTokens(BoundaryTokens::Star));
    m.insert("!",  TokenType::BoundaryTokens(BoundaryTokens::Bang));
    m.insert("=",  TokenType::BoundaryTokens(BoundaryTokens::Equal));
    m.insert(">",  TokenType::BoundaryTokens(BoundaryTokens::Greater));
    m.insert("<",  TokenType::BoundaryTokens(BoundaryTokens::Less));
    m.insert("!=", TokenType::BoundaryTokens(BoundaryTokens::BangEqual));
    m.insert("==", TokenType::BoundaryTokens(BoundaryTokens::EqualEqual));
    m.insert(">=", TokenType::BoundaryTokens(BoundaryTokens::GreaterEqual));
    m.insert("<=", TokenType::BoundaryTokens(BoundaryTokens::LessEqual));
    m
});


pub struct Token {
    lexeme: String,
    token_type: TokenType,
    line: usize,
    literal: String,
}

pub fn consume_till_end_of_line(source: &mut reader::Source) {
    while let Some(c) = source.advance() {
        if c == '\n' {
            break;
        }
    }
}

pub fn add_token_if_boundary(cp: &str, tokens: &mut Vec<Token>, line: usize) -> bool {
    if let Some(token_type) = BOUNDARY_TOKENS.get(cp) {
        tokens.push(Token {
            lexeme: cp.to_string(),
            token_type: token_type.clone(),
            line,
            literal: String::new(),
        });
        true
    } else {
        false
    }
}

pub fn process_lexeme(lexeme: &str, tokens: &mut Vec<Token>, line: usize) {
    if lexeme.is_empty() {
        return;
    }
    if let Some(token_type) = KEYWORDS.get(lexeme) {
        tokens.push(Token {
            lexeme: lexeme.to_string(),
            token_type: token_type.clone(),
            line,
            literal: String::new(),
        });
    } else {
        tokens.push(Token {
            lexeme: lexeme.to_string(),
            token_type: TokenType::Literals(Literals::Identifier),
            line,
            literal: String::new(),
        });
    }
}


pub fn scan(source: &mut reader::Source) -> Result<Vec<Token>, LoxError> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut current_lexeme = String::new();
    loop {
        let c = source.advance();
        let p = source.peek_char();
        if let Some(c) = c {
            if c == '/' {
                if p == Some('/') {
                    process_lexeme(&current_lexeme, &mut tokens, line);
                    current_lexeme.clear();
                    consume_till_end_of_line(source);
                    line += 1;
                    continue;
                } else {
                    process_lexeme(&current_lexeme, &mut tokens, line);
                    current_lexeme.clear();
                    add_token_if_boundary(&c.to_string(), &mut tokens, line);
                    continue;
                }
            }
            match c {
                '\n' => line += 1,
                ' ' | '\r' | '\t' => {
                    process_lexeme(&current_lexeme, &mut tokens, line);
                    current_lexeme.clear();
                },
                _ => {
                    // Check two-char boundary first (greedy).
                    if let Some(p) = p {
                        let cp = format!("{}{}", c, p);
                        if BOUNDARY_TOKENS.contains_key(cp.as_str()) {
                            process_lexeme(&current_lexeme, &mut tokens, line);
                            current_lexeme.clear();
                            add_token_if_boundary(&cp, &mut tokens, line);
                            source.advance();
                            continue;
                        }
                    }
                    // Check single-char boundary.
                    if BOUNDARY_TOKENS.contains_key(c.to_string().as_str()) {
                        process_lexeme(&current_lexeme, &mut tokens, line);
                        current_lexeme.clear();
                        add_token_if_boundary(&c.to_string(), &mut tokens, line);
                        continue;
                    }
                    current_lexeme.push(c);
                },
            }
        } else {
            process_lexeme(&current_lexeme, &mut tokens, line);
            current_lexeme.clear();
            tokens.push(Token {
                lexeme: String::new(),
                token_type: TokenType::Eof,
                line,
                literal: String::new(),
            });
            break;
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens_eq(token: &Token, expected_lexeme: &str, expected_token_type: &TokenType, expected_line: usize) {
        assert_eq!(token.lexeme, expected_lexeme);
        assert_eq!(&token.token_type, expected_token_type);
        assert_eq!(token.line, expected_line);
    }

    #[test]
    fn test_assignment() {
        let mut source = reader::Source::new("var x = 10;\nvar y = 20;".into());
        let tokens = scan(&mut source).unwrap();
        assert_eq!(tokens.len(), 11);
        assert_tokens_eq(&tokens[0],  "var", &TokenType::Keywords(Keywords::Var),             1);
        assert_tokens_eq(&tokens[1],  "x",   &TokenType::Literals(Literals::Identifier),      1);
        assert_tokens_eq(&tokens[2],  "=",   &TokenType::BoundaryTokens(BoundaryTokens::Equal), 1);
        assert_tokens_eq(&tokens[3],  "10",  &TokenType::Literals(Literals::Identifier),      1);
        assert_tokens_eq(&tokens[4],  ";",   &TokenType::BoundaryTokens(BoundaryTokens::Semicolon), 1);
        assert_tokens_eq(&tokens[5],  "var", &TokenType::Keywords(Keywords::Var),             2);
        assert_tokens_eq(&tokens[6],  "y",   &TokenType::Literals(Literals::Identifier),      2);
        assert_tokens_eq(&tokens[7],  "=",   &TokenType::BoundaryTokens(BoundaryTokens::Equal), 2);
        assert_tokens_eq(&tokens[8],  "20",  &TokenType::Literals(Literals::Identifier),      2);
        assert_tokens_eq(&tokens[9],  ";",   &TokenType::BoundaryTokens(BoundaryTokens::Semicolon), 2);
        assert_tokens_eq(&tokens[10], "",    &TokenType::Eof,                                 2);
    }
}
