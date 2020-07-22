#[cfg(test)]
use crate::lexer::Lexer;
#[cfg(test)]
use crate::token::{Token, TokenType, TokenValue};

#[test]
fn lex_comment_line() {
    assert_eq!(
        Lexer::new("// comment").lex(),
        vec![Token {
            token_type: TokenType::Comment,
            value: Some(TokenValue::String("comment".into())),
            line_start: 1,
            line_end: 1,
            column_start: 1,
            column_end: 11,
        }]
    );
}

#[test]
fn lex_comment_block() {
    assert_eq!(
        Lexer::new("/* comment */").lex(),
        vec![Token {
            token_type: TokenType::Comment,
            value: Some(TokenValue::String("comment".into())),
            line_start: 1,
            line_end: 1,
            column_start: 1,
            column_end: 14,
        }],
    );
}

#[test]
fn lex_comment_block_multi() {
    assert_eq!(
        Lexer::new(
            "/*
comment on
multiple lines
*/"
        )
        .lex(),
        vec![Token {
            token_type: TokenType::Comment,
            value: Some(TokenValue::String("comment on\nmultiple lines".into())),
            line_start: 1,
            line_end: 4,
            column_start: 1,
            column_end: 3,
        }],
    );
}

#[test]
fn lex_cell_declaration() {
    assert_eq!(
        Lexer::new("new x = 5;").lex(),
        vec![
            Token {
                token_type: TokenType::New,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 1,
                column_end: 4,
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("x".into())),
                line_start: 1,
                line_end: 1,
                column_start: 5,
                column_end: 6,
            },
            Token {
                token_type: TokenType::Assign,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 7,
                column_end: 8,
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(5)),
                line_start: 1,
                line_end: 1,
                column_start: 9,
                column_end: 10,
            },
            Token {
                token_type: TokenType::Semicolon,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 10,
                column_end: 11,
            },
        ],
    );
}

#[test]
fn lex_float_decl() {
    assert_eq!(
        Lexer::new("new Float:x = 5.5;").lex(),
        vec![
            Token {
                token_type: TokenType::New,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 1,
                column_end: 4
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("Float".into())),
                line_start: 1,
                line_end: 1,
                column_start: 5,
                column_end: 10
            },
            Token {
                token_type: TokenType::Colon,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 10,
                column_end: 11
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("x".into())),
                line_start: 1,
                line_end: 1,
                column_start: 11,
                column_end: 12
            },
            Token {
                token_type: TokenType::Assign,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 13,
                column_end: 14
            },
            Token {
                token_type: TokenType::Float,
                value: Some(TokenValue::Float(5.5)),
                line_start: 1,
                line_end: 1,
                column_start: 15,
                column_end: 18
            },
            Token {
                token_type: TokenType::Semicolon,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 18,
                column_end: 19
            }
        ],
        "float declaration"
    );
}

#[test]
fn lex_array_decl_autosize() {
    assert_eq!(
        Lexer::new("new x[] = {1, 2, 3};").lex(),
        vec![
            Token {
                token_type: TokenType::New,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 1,
                column_end: 4
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("x".into())),
                line_start: 1,
                line_end: 1,
                column_start: 5,
                column_end: 6
            },
            Token {
                token_type: TokenType::LeftSquare,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 6,
                column_end: 7
            },
            Token {
                token_type: TokenType::RightSquare,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 7,
                column_end: 8
            },
            Token {
                token_type: TokenType::Assign,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 9,
                column_end: 10
            },
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 11,
                column_end: 12
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(1)),
                line_start: 1,
                line_end: 1,
                column_start: 12,
                column_end: 13
            },
            Token {
                token_type: TokenType::Comma,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 13,
                column_end: 14
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(2)),
                line_start: 1,
                line_end: 1,
                column_start: 15,
                column_end: 16
            },
            Token {
                token_type: TokenType::Comma,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 16,
                column_end: 17
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(3)),
                line_start: 1,
                line_end: 1,
                column_start: 18,
                column_end: 19
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 19,
                column_end: 20
            },
            Token {
                token_type: TokenType::Semicolon,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 20,
                column_end: 21
            },
        ]
    );
}

#[test]
fn lex_array_decl() {
    assert_eq!(
        Lexer::new("new x[4] = {1, 2, 3};").lex(),
        vec![
            Token {
                token_type: TokenType::New,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 1,
                column_end: 4
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("x".into())),
                line_start: 1,
                line_end: 1,
                column_start: 5,
                column_end: 6
            },
            Token {
                token_type: TokenType::LeftSquare,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 6,
                column_end: 7
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(4)),
                line_start: 1,
                line_end: 1,
                column_start: 7,
                column_end: 8
            },
            Token {
                token_type: TokenType::RightSquare,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 8,
                column_end: 9
            },
            Token {
                token_type: TokenType::Assign,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 10,
                column_end: 11
            },
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 12,
                column_end: 13
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(1)),
                line_start: 1,
                line_end: 1,
                column_start: 13,
                column_end: 14
            },
            Token {
                token_type: TokenType::Comma,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 14,
                column_end: 15
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(2)),
                line_start: 1,
                line_end: 1,
                column_start: 16,
                column_end: 17
            },
            Token {
                token_type: TokenType::Comma,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 17,
                column_end: 18
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(3)),
                line_start: 1,
                line_end: 1,
                column_start: 19,
                column_end: 20
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 20,
                column_end: 21
            },
            Token {
                token_type: TokenType::Semicolon,
                value: None,
                line_start: 1,
                line_end: 1,
                column_start: 21,
                column_end: 22
            }
        ],
        "array declaration specific size"
    );
}

#[test]
fn lex_basic_script() {
    assert_eq!(
        Lexer::new(
            "
// Comment
#include <a_samp>

main() {
    new a;
    if(a == 3) {
        a++;
    } else if(a != 3) {
        a--;
    } else {
        a = 0;
    }
}
"
        )
        .lex(),
        vec![
            Token {
                token_type: TokenType::Comment,
                value: Some(TokenValue::String("Comment".into())),
                line_start: 2,
                line_end: 3,
                column_start: 1,
                column_end: 1
            },
            Token {
                token_type: TokenType::Directive,
                value: None,
                line_start: 3,
                line_end: 3,
                column_start: 1,
                column_end: 2
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("include".into())),
                line_start: 3,
                line_end: 3,
                column_start: 2,
                column_end: 9
            },
            Token {
                token_type: TokenType::LowerThan,
                value: None,
                line_start: 3,
                line_end: 3,
                column_start: 10,
                column_end: 11
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("a_samp".into())),
                line_start: 3,
                line_end: 3,
                column_start: 11,
                column_end: 17
            },
            Token {
                token_type: TokenType::GreaterThan,
                value: None,
                line_start: 3,
                line_end: 3,
                column_start: 17,
                column_end: 18
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("main".into())),
                line_start: 5,
                line_end: 5,
                column_start: 1,
                column_end: 5
            },
            Token {
                token_type: TokenType::LeftBracket,
                value: None,
                line_start: 5,
                line_end: 5,
                column_start: 5,
                column_end: 6
            },
            Token {
                token_type: TokenType::RightBracket,
                value: None,
                line_start: 5,
                line_end: 5,
                column_start: 6,
                column_end: 7
            },
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line_start: 5,
                line_end: 5,
                column_start: 8,
                column_end: 9
            },
            Token {
                token_type: TokenType::New,
                value: None,
                line_start: 6,
                line_end: 6,
                column_start: 5,
                column_end: 8
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("a".into())),
                line_start: 6,
                line_end: 6,
                column_start: 9,
                column_end: 10
            },
            Token {
                token_type: TokenType::Semicolon,
                value: None,
                line_start: 6,
                line_end: 6,
                column_start: 10,
                column_end: 11
            },
            Token {
                token_type: TokenType::If,
                value: None,
                line_start: 7,
                line_end: 7,
                column_start: 5,
                column_end: 7
            },
            Token {
                token_type: TokenType::LeftBracket,
                value: None,
                line_start: 7,
                line_end: 7,
                column_start: 7,
                column_end: 8
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("a".into())),
                line_start: 7,
                line_end: 7,
                column_start: 8,
                column_end: 9
            },
            Token {
                token_type: TokenType::Equal,
                value: None,
                line_start: 7,
                line_end: 7,
                column_start: 10,
                column_end: 12
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(3)),
                line_start: 7,
                line_end: 7,
                column_start: 13,
                column_end: 14
            },
            Token {
                token_type: TokenType::RightBracket,
                value: None,
                line_start: 7,
                line_end: 7,
                column_start: 14,
                column_end: 15
            },
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line_start: 7,
                line_end: 7,
                column_start: 16,
                column_end: 17
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("a".into())),
                line_start: 8,
                line_end: 8,
                column_start: 9,
                column_end: 10
            },
            Token {
                token_type: TokenType::PlusPlus,
                value: None,
                line_start: 8,
                line_end: 8,
                column_start: 10,
                column_end: 12
            },
            Token {
                token_type: TokenType::Semicolon,
                value: None,
                line_start: 8,
                line_end: 8,
                column_start: 12,
                column_end: 13
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line_start: 9,
                line_end: 9,
                column_start: 5,
                column_end: 6
            },
            Token {
                token_type: TokenType::Else,
                value: None,
                line_start: 9,
                line_end: 9,
                column_start: 7,
                column_end: 11
            },
            Token {
                token_type: TokenType::If,
                value: None,
                line_start: 9,
                line_end: 9,
                column_start: 12,
                column_end: 14
            },
            Token {
                token_type: TokenType::LeftBracket,
                value: None,
                line_start: 9,
                line_end: 9,
                column_start: 14,
                column_end: 15
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("a".into())),
                line_start: 9,
                line_end: 9,
                column_start: 15,
                column_end: 16
            },
            Token {
                token_type: TokenType::NotEqual,
                value: None,
                line_start: 9,
                line_end: 9,
                column_start: 17,
                column_end: 19
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(3)),
                line_start: 9,
                line_end: 9,
                column_start: 20,
                column_end: 21
            },
            Token {
                token_type: TokenType::RightBracket,
                value: None,
                line_start: 9,
                line_end: 9,
                column_start: 21,
                column_end: 22
            },
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line_start: 9,
                line_end: 9,
                column_start: 23,
                column_end: 24
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("a".into())),
                line_start: 10,
                line_end: 10,
                column_start: 9,
                column_end: 10
            },
            Token {
                token_type: TokenType::MinusMinus,
                value: None,
                line_start: 10,
                line_end: 10,
                column_start: 10,
                column_end: 12
            },
            Token {
                token_type: TokenType::Semicolon,
                value: None,
                line_start: 10,
                line_end: 10,
                column_start: 12,
                column_end: 13
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line_start: 11,
                line_end: 11,
                column_start: 5,
                column_end: 6
            },
            Token {
                token_type: TokenType::Else,
                value: None,
                line_start: 11,
                line_end: 11,
                column_start: 7,
                column_end: 11
            },
            Token {
                token_type: TokenType::LeftBrace,
                value: None,
                line_start: 11,
                line_end: 11,
                column_start: 12,
                column_end: 13
            },
            Token {
                token_type: TokenType::Symbol,
                value: Some(TokenValue::String("a".into())),
                line_start: 12,
                line_end: 12,
                column_start: 9,
                column_end: 10
            },
            Token {
                token_type: TokenType::Assign,
                value: None,
                line_start: 12,
                line_end: 12,
                column_start: 11,
                column_end: 12
            },
            Token {
                token_type: TokenType::Integer,
                value: Some(TokenValue::Integer(0)),
                line_start: 12,
                line_end: 12,
                column_start: 13,
                column_end: 14
            },
            Token {
                token_type: TokenType::Semicolon,
                value: None,
                line_start: 12,
                line_end: 12,
                column_start: 14,
                column_end: 15
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line_start: 13,
                line_end: 13,
                column_start: 5,
                column_end: 6
            },
            Token {
                token_type: TokenType::RightBrace,
                value: None,
                line_start: 14,
                line_end: 14,
                column_start: 1,
                column_end: 2
            }
        ]
    );
}
