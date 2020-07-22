use crate::token::Token;
use crate::token::TokenType;

#[derive(Debug)]
pub enum Expression {
    GlobalScope,
    Variable(Box<Variable>),
    BinaryOperation(Box<BinaryOperation>),
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub tag: String,
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub operator: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

/// Node represents an actual AST node on the syntax graph.
#[derive(Debug)]
pub struct Node {
    pub expr: Expression,
    pub tokens: Vec<Token>,
    pub children: Vec<Node>,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub root: Node,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
            root: Node {
                expr: Expression::GlobalScope,
                tokens: vec![],
                children: vec![],
            },
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        while self.current < self.tokens.len() {
            let token = &self.tokens[self.current];

            let node = match token.token_type {
                TokenType::New => Some(self.parse_declaration()?),
                _ => None,
            };

            if let Some(node) = node {
                self.root.children.push(node);
            }

            self.current += 1;
        }

        Ok(())
    }

    fn next(&mut self) -> &Token {
        self.current += 1;
        &self.tokens[self.current]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn parse_declaration(&mut self) -> Result<Node, String> {
        let token = match &self.next().token_type {
            // TODO: deal with new const, new static, new stock etc
            TokenType::Symbol => TokenType::Symbol,
            token @ _ => return Err(format!("expected symbol, found {:?}", token)),
        };

        let mut node = self.parse_symbol()?;

        if self.peek().token_type == TokenType::Equal {
            self.current += 1;
            node.children.push(self.parse_expression()?);
        };

        self.expect_semicolon()?;

        Ok(node)
    }

    fn expect_symbol(&mut self) -> Result<TokenType, String> {
        match &self.next().token_type {
            TokenType::Symbol => Ok(TokenType::Symbol),
            token @ _ => Err(format!("expected symbol, found {:?}", token)),
        }
    }

    fn expect_semicolon(&mut self) -> Result<TokenType, String> {
        match &self.next().token_type {
            TokenType::Semicolon => Ok(TokenType::Semicolon),
            token @ _ => Err(format!("expected semicolon, found {:?}", token)),
        }
    }

    fn parse_expression(&mut self) -> Result<Node, String> {
        Err(String::from("not implemented"))
    }

    fn parse_symbol(&mut self) -> Result<Node, String> {
        // (tag:)ident([<expr>])
        let token = self.expect_symbol()?;

        if self.peek().token_type == TokenType::Colon {
            self.current += 1;

            let tag = &token;
            let symbol = self.expect_symbol()?;

            return Ok(Node {
                expr: Expression::Variable(Box::new(Variable {
                    name: symbol.to_string(),
                    tag: tag.to_string(),
                })),
                tokens: vec![],
                children: vec![],
            });
        } else {
            return Ok(Node {
                expr: Expression::Variable(Box::new(Variable {
                    name: token.to_string(),
                    tag: String::from("_"),
                })),
                tokens: vec![],
                children: vec![],
            });
        }
    }

    fn parse_literal_scalar(&mut self) -> Result<Node, String> {
        // (tag:)(+|-)literal
        Err(String::from("not implemented"))
    }
}
