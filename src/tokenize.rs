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
    col: usize,
    line: usize,
    literal: String,
}

fn consume_till_end_of_line(source: &mut reader::Source) {
    while let Some(c) = source.advance() {
        if c == '\n' {
            break;
        }
    }
}

fn consume_till_end_of_block_comment(source: &mut reader::Source, start_line: usize, start_col: usize) -> Result<(usize, usize), LoxError> {
    let mut line = start_line;
    let mut col = start_col;
    loop {
        match source.advance() {
            Some('*') if source.peek_char() == Some('/') => {
                source.advance();
                col += 2;
                return Ok((line, col));
            }
            Some('/') if source.peek_char() == Some('*') => {
                // Challenge: Handle nested block comments.
                source.advance();
                col += 2;
                (line, col) = consume_till_end_of_block_comment(source, line, col)?;
            }
            Some('\n') => { line += 1; col = 0; }
            Some(_) => { col += 1; }
            None => return Err(LoxError::ScanError {
                line,
                col,
                message: "Unterminated block comment".into(),
            }),
        }
    }
}

fn add_boundary_token(token_type: TokenType, cp: &str, tokens: &mut Vec<Token>, line: usize, col: usize) {
    tokens.push(Token {
        lexeme: cp.to_string(),
        token_type,
        line,
        col,
        literal: String::new(),
    });
}

fn process_lexeme(lexeme: &str, tokens: &mut Vec<Token>, line: usize, col: usize) {
    if lexeme.is_empty() {
        return;
    }
    if let Some(token_type) = KEYWORDS.get(lexeme) {
        tokens.push(Token {
            lexeme: lexeme.to_string(),
            token_type: token_type.clone(),
            line,
            col,
            literal: String::new(),
        });
    } else {
        // TODO (vin): Handle string and number literals first.
        tokens.push(Token {
            lexeme: lexeme.to_string(),
            token_type: TokenType::Literals(Literals::Identifier),
            line,
            col,
            literal: String::new(),
        });
    }
}


pub fn scan(source: &mut reader::Source) -> Result<Vec<Token>, LoxError> {
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut col: usize = 0;
    let mut current_lexeme = String::new();
    loop {
        let c = source.advance();
        let p = source.peek_char();
        col += 1;
        if let Some(c) = c {
            match c {
                // Newline is a special case since it also resets the column number.
                '\n' => { line += 1; col = 0; },
                ' ' | '\r' | '\t' => {
                    process_lexeme(&current_lexeme, &mut tokens, line, col);
                    current_lexeme.clear();
                },
                '/' => {
                    match p {
                        Some('/') => {
                            process_lexeme(&current_lexeme, &mut tokens, line, col);
                            current_lexeme.clear();
                            consume_till_end_of_line(source);
                            // Advance the line and reset column after consuming the comment.
                            line += 1;
                            col = 0;
                        }
                        Some('*') => {
                            source.advance(); // consume '*'
                            process_lexeme(&current_lexeme, &mut tokens, line, col);
                            current_lexeme.clear();
                            (line, col) = consume_till_end_of_block_comment(source, line, col)?;
                        }
                        _ => {
                            // Handle '/' as a boundary token.
                            process_lexeme(&current_lexeme, &mut tokens, line, col);
                            current_lexeme.clear();
                            add_boundary_token(TokenType::BoundaryTokens(BoundaryTokens::Slash), &c.to_string(), &mut tokens, line, col);
                        }
                    }
                }
                '.' => {
                    // Handle if '.' is part of a number literal
                    if !current_lexeme.is_empty() && current_lexeme.chars().all(|c| c.is_ascii_digit()) {
                        current_lexeme.push(c);
                    } else {
                        // treat '.' as a boundary token and process the current lexeme if it's not empty.
                        process_lexeme(&current_lexeme, &mut tokens, line, col);
                        current_lexeme.clear();
                        add_boundary_token(TokenType::BoundaryTokens(BoundaryTokens::Dot), &c.to_string(), &mut tokens, line, col);
                        continue;
                    }
                }
                _ => {
                    // Check two-char boundary first (greedy).
                    if let Some(p) = p {
                        let cp = format!("{}{}", c, p);
                        if let Some(bt) = BOUNDARY_TOKENS.get(cp.as_str()) {
                            process_lexeme(&current_lexeme, &mut tokens, line, col);
                            current_lexeme.clear();
                            add_boundary_token(bt.clone(), &cp, &mut tokens, line, col);
                            source.advance();
                            // Advance the column for the second character in the two-char boundary token.
                            col += 1;
                            continue;
                        }
                    }
                    // Check single-char boundary.
                    let cs = c.to_string();
                    if let Some(bt) = BOUNDARY_TOKENS.get(cs.as_str()) {
                        process_lexeme(&current_lexeme, &mut tokens, line, col);
                        current_lexeme.clear();
                        add_boundary_token(bt.clone(), &cs, &mut tokens, line, col);
                        continue;
                    }
                    if c.is_alphanumeric() {
                        current_lexeme.push(c);
                    } else {
                        return Err(LoxError::ScanError {
                            line,
                            col,
                            message: format!("unexpected character '{c}' at column {col}"),
                        });
                    }
                },
            }
        } else {
            process_lexeme(&current_lexeme, &mut tokens, line, col);
            current_lexeme.clear();
            tokens.push(Token {
                lexeme: String::new(),
                token_type: TokenType::Eof,
                line,
                col,
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

    #[test]
    fn test_orchid() {
        let mut source = reader::Source::new("var orchid = 30;".into());
        let tokens = scan(&mut source).unwrap();
        assert_eq!(tokens.len(), 6);
        assert_tokens_eq(&tokens[0], "var", &TokenType::Keywords(Keywords::Var), 1);
        assert_tokens_eq(&tokens[1], "orchid", &TokenType::Literals(Literals::Identifier), 1);
        assert_tokens_eq(&tokens[2], "=", &TokenType::BoundaryTokens(BoundaryTokens::Equal), 1);
        assert_tokens_eq(&tokens[3], "30", &TokenType::Literals(Literals::Identifier), 1);
        assert_tokens_eq(&tokens[4], ";", &TokenType::BoundaryTokens(BoundaryTokens::Semicolon), 1);
        assert_tokens_eq(&tokens[5], "", &TokenType::Eof, 1);
    }

    #[test]
    fn test_nested_block_comment() {
        let mut source = reader::Source::new("var x = 10; /* this /*is*/ a comment */ var y = 20;".into());
        let tokens = scan(&mut source).unwrap();
        assert_eq!(tokens.len(), 11);
        assert_tokens_eq(&tokens[0],  "var", &TokenType::Keywords(Keywords::Var),             1);
        assert_tokens_eq(&tokens[1],  "x",   &TokenType::Literals(Literals::Identifier),      1);
        assert_tokens_eq(&tokens[2],  "=",   &TokenType::BoundaryTokens(BoundaryTokens::Equal), 1);
        assert_tokens_eq(&tokens[3],  "10",  &TokenType::Literals(Literals::Identifier),      1);
        assert_tokens_eq(&tokens[4],  ";",   &TokenType::BoundaryTokens(BoundaryTokens::Semicolon), 1);
        assert_tokens_eq(&tokens[5],  "var", &TokenType::Keywords(Keywords::Var),             1);
        assert_tokens_eq(&tokens[6],  "y",   &TokenType::Literals(Literals::Identifier),      1);
        assert_tokens_eq(&tokens[7],  "=",   &TokenType::BoundaryTokens(BoundaryTokens::Equal), 1);
        assert_tokens_eq(&tokens[8],  "20",  &TokenType::Literals(Literals::Identifier),      1);
        assert_tokens_eq(&tokens[9],  ";",   &TokenType::BoundaryTokens(BoundaryTokens::Semicolon), 1);
        assert_tokens_eq(&tokens[10], "",    &TokenType::Eof,                                 1);
    }
}
