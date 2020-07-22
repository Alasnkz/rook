use std::iter::Peekable;
use std::str::Chars;

use crate::ring::Ring;
use crate::token;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenValue;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    initial_line: i32,
    current_line: i32,
    initial_column: i32,
    current_column: i32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
            initial_line: 1,
            current_line: 1,
            initial_column: 1,
            current_column: 1,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok.token_type == TokenType::End {
                break;
            }
            tokens.push(tok)
        }
        tokens
    }

    fn gen_token(&self, t: TokenType, v: Option<TokenValue>) -> Token {
        Token {
            token_type: t,
            value: v,
            line_start: self.initial_line,
            line_end: self.current_line,
            column_start: self.initial_column,
            column_end: self.current_column,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        let next = match self.input.next() {
            Some(v) => v,
            None => return None,
        };
        if next == '\n' {
            self.current_line += 1;
            self.current_column = 1;
        } else {
            self.current_column += 1;
        }
        Some(next)
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    // fn peek_char_eq(&mut self, ch: char) -> bool {
    //     match self.peek_char() {
    //         Some(&peek_ch) => peek_ch == ch,
    //         None => false,
    //     }
    // }

    fn peek_char_eq_consume(&mut self, ch: char) -> bool {
        match self.peek_char() {
            Some(&peek_ch) => {
                if peek_ch == ch {
                    self.read_char();
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek_char() {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn peek_is_letter(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => is_letter(ch),
            None => false,
        }
    }

    fn read_string_until(&mut self, until: &str) -> String {
        let mut result = String::new();
        let until_len = until.len();
        let mut recent_chars = Ring::new(until_len);

        while let Some(c) = self.read_char() {
            recent_chars.insert(c as u8);
            if String::from(until) == String::from_utf8(recent_chars.unroll()).unwrap() {
                result.truncate(result.len() - until_len);
                break;
            }
            result.push(c);
        }
        result
    }

    fn read_until_eol(&mut self) -> String {
        let mut line = String::new();
        while let Some(c) = self.read_char() {
            if c == '\n' {
                break;
            }
            line.push(c);
        }
        line
    }

    fn read_symbol(&mut self, first: char) -> Token {
        let mut ident = String::new();
        ident.push(first);

        while self.peek_is_letter() {
            ident.push(self.read_char().unwrap());
        }

        let (kw, v) = match token::lookup_keyword(&ident) {
            Some(v) => (v, None),
            None => (TokenType::Symbol, Some(TokenValue::String(ident.into()))),
        };

        Token {
            token_type: kw,
            value: v,
            line_start: self.initial_line,
            line_end: self.current_line,
            column_start: self.initial_column,
            column_end: self.current_column,
        }
    }

    fn read_number(&mut self, first: char) -> Token {
        let mut number = String::new();
        number.push(first);

        let mut has_decimal = false;
        while let Some(&c) = self.peek_char() {
            if !c.is_numeric() {
                if c == '.' {
                    has_decimal = true
                } else if c == '_' {
                    continue;
                } else {
                    break;
                }
            }

            number.push(self.read_char().unwrap());
        }

        if has_decimal {
            self.gen_token(
                TokenType::Float,
                Some(TokenValue::Float(number.parse().unwrap())),
            )
        } else {
            self.gen_token(
                TokenType::Integer,
                Some(TokenValue::Integer(number.parse().unwrap())),
            )
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        self.initial_line = self.current_line;
        self.initial_column = self.current_column;

        match self.read_char() {
            Some('=') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::Equal, None)
                } else {
                    self.gen_token(TokenType::Assign, None)
                }
            }
            Some('+') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::PlusAssign, None)
                } else if self.peek_char_eq_consume('+') {
                    self.gen_token(TokenType::PlusPlus, None)
                } else {
                    self.gen_token(TokenType::Plus, None)
                }
            }
            Some('-') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::MinusAssign, None)
                } else if self.peek_char_eq_consume('-') {
                    self.gen_token(TokenType::MinusMinus, None)
                } else {
                    self.gen_token(TokenType::Minus, None)
                }
            }
            Some('*') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::AsteriskAssign, None)
                } else {
                    self.gen_token(TokenType::Asterisk, None)
                }
            }
            Some('/') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::SlashAssign, None)
                } else if self.peek_char_eq_consume('/') {
                    let rest = self.read_until_eol();
                    self.gen_token(
                        TokenType::Comment,
                        Some(TokenValue::String(rest.trim_start().into())),
                    )
                } else if self.peek_char_eq_consume('*') {
                    let rest = self.read_string_until("*/");
                    self.gen_token(
                        TokenType::Comment,
                        Some(TokenValue::String(rest.trim_start().into())),
                    )
                } else {
                    self.gen_token(TokenType::Slash, None)
                }
            }
            Some('%') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::PercentAssign, None)
                } else {
                    self.gen_token(TokenType::Percent, None)
                }
            }
            Some('&') => {
                if self.peek_char_eq_consume('&') {
                    self.gen_token(TokenType::And, None)
                } else if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::BitAndAssign, None)
                } else {
                    self.gen_token(TokenType::BitAnd, None)
                }
            }
            Some('|') => {
                if self.peek_char_eq_consume('|') {
                    self.gen_token(TokenType::Or, None)
                } else if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::BitOrAssign, None)
                } else {
                    self.gen_token(TokenType::BitOr, None)
                }
            }
            Some('^') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::BitXorAssign, None)
                } else {
                    self.gen_token(TokenType::BitXor, None)
                }
            }
            Some('<') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::LowerThanEqual, None)
                } else if self.peek_char_eq_consume('<') {
                    if self.peek_char_eq_consume('=') {
                        self.gen_token(TokenType::BitLeftAssign, None)
                    } else {
                        self.gen_token(TokenType::BitLeft, None)
                    }
                } else {
                    self.gen_token(TokenType::LowerThan, None)
                }
            }
            Some('>') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::GreaterThanEqual, None)
                } else if self.peek_char_eq_consume('>') {
                    if self.peek_char_eq_consume('=') {
                        self.gen_token(TokenType::BitRightAssign, None)
                    } else {
                        self.gen_token(TokenType::BitRight, None)
                    }
                } else {
                    self.gen_token(TokenType::GreaterThan, None)
                }
            }
            Some('!') => {
                if self.peek_char_eq_consume('=') {
                    self.gen_token(TokenType::NotEqual, None)
                } else {
                    self.gen_token(TokenType::Bang, None)
                }
            }
            Some(';') => self.gen_token(TokenType::Semicolon, None),
            Some(':') => self.gen_token(TokenType::Colon, None),
            Some(',') => self.gen_token(TokenType::Comma, None),
            Some('{') => self.gen_token(TokenType::LeftBrace, None),
            Some('}') => self.gen_token(TokenType::RightBrace, None),
            Some('(') => self.gen_token(TokenType::LeftBracket, None),
            Some(')') => self.gen_token(TokenType::RightBracket, None),
            Some('[') => self.gen_token(TokenType::LeftSquare, None),
            Some(']') => self.gen_token(TokenType::RightSquare, None),
            Some('.') => {
                if self.peek_char_eq_consume('.') {
                    if self.peek_char_eq_consume('.') {
                        self.gen_token(TokenType::Elipsis, None)
                    } else {
                        self.gen_token(TokenType::Range, None)
                    }
                } else {
                    self.gen_token(TokenType::Illegal, None)
                }
            }
            Some('#') => self.gen_token(TokenType::Directive, None),

            Some(ch @ _) => {
                if is_letter(ch) {
                    self.read_symbol(ch)
                } else if ch.is_numeric() {
                    self.read_number(ch)
                } else {
                    self.gen_token(TokenType::Illegal, None)
                }
            }

            None => self.gen_token(TokenType::End, None),
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
