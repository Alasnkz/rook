#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<TokenValue>,
    pub line_start: i32,
    pub line_end: i32,
    pub column_start: i32,
    pub column_end: i32,
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    String(String),
    Integer(i32),
    Float(f32),
}

impl TokenValue {
    pub fn len(&self) -> usize {
        match self {
            TokenValue::String(v) => v.len(),
            TokenValue::Integer(v) => v.to_string().len(),
            TokenValue::Float(v) => v.to_string().len(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Illegal,
    End,

    // -
    // Symbols
    // -
    Equal,            // ==
    Assign,           // =
    Plus,             // +
    PlusPlus,         // ++
    PlusAssign,       // +=
    Minus,            // -
    MinusMinus,       // --
    MinusAssign,      // -=
    Asterisk,         // *
    AsteriskAssign,   // *=
    Slash,            // /
    SlashAssign,      // /=
    Percent,          // %
    PercentAssign,    // %=
    And,              // &&
    BitAnd,           // &
    BitAndAssign,     // &=
    Or,               // ||
    BitOr,            // |
    BitOrAssign,      // |=
    BitXor,           // ^
    BitXorAssign,     // ^=
    LowerThan,        // <
    LowerThanEqual,   // <=
    BitLeft,          // <<
    BitLeftAssign,    // <<=
    GreaterThan,      // >
    GreaterThanEqual, // >=
    BitRight,         // >>
    BitRightAssign,   // >>=
    NotEqual,         // !=
    Bang,             // !
    Colon,            // :
    Semicolon,        // ;
    Comma,            // ,
    LeftBrace,        // {
    RightBrace,       // }
    LeftBracket,      // (
    RightBracket,     // )
    LeftSquare,       // [
    RightSquare,      // ]
    Elipsis,          // ...
    Range,            // ..
    Directive,        // #

    // -
    // Keywords - declaration/definition
    // -
    Const,    // const
    New,      // new
    Static,   // static
    Stock,    // stock
    Forward,  // forward
    Public,   // public
    Native,   // native
    Operator, // operator
    Char,     // char
    Enum,     // enum
    State,    // state

    // -
    // Keywords - control flow
    // -
    If,       // if
    Else,     // else
    Switch,   // switch
    Case,     // case
    Default,  // default
    For,      // for
    While,    // while
    Do,       // do
    Break,    // break
    Continue, // continue
    Goto,     // goto
    Return,   // return
    Sizeof,   // sizeof
    Tagof,    // tagof
    Emit,     // __emit

    // -
    // Patterns
    // -
    Integer, // integer number
    Float,   // floating point number
    Symbol,  // a-zA-Z0-9_@
    Label,   // a-zA-Z0-9_
    Literal, // ".*"
    Comment,
}

impl Default for TokenType {
    fn default() -> TokenType {
        TokenType::Illegal
    }
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::Illegal => String::from("Illegal"),
            TokenType::End => String::from("End"),
            TokenType::Equal => String::from("=="),
            TokenType::Assign => String::from("="),
            TokenType::Plus => String::from("+"),
            TokenType::PlusPlus => String::from("++"),
            TokenType::PlusAssign => String::from("+="),
            TokenType::Minus => String::from("-"),
            TokenType::MinusMinus => String::from("--"),
            TokenType::MinusAssign => String::from("-="),
            TokenType::Asterisk => String::from("*"),
            TokenType::AsteriskAssign => String::from("*="),
            TokenType::Slash => String::from("/"),
            TokenType::SlashAssign => String::from("/="),
            TokenType::Percent => String::from("%"),
            TokenType::PercentAssign => String::from("%="),
            TokenType::And => String::from("&&"),
            TokenType::BitAnd => String::from("&"),
            TokenType::BitAndAssign => String::from("&="),
            TokenType::Or => String::from("||"),
            TokenType::BitOr => String::from("|"),
            TokenType::BitOrAssign => String::from("|="),
            TokenType::BitXor => String::from("^"),
            TokenType::BitXorAssign => String::from("^="),
            TokenType::LowerThan => String::from("<"),
            TokenType::LowerThanEqual => String::from("<="),
            TokenType::BitLeft => String::from("<<"),
            TokenType::BitLeftAssign => String::from("<<="),
            TokenType::GreaterThan => String::from(">"),
            TokenType::GreaterThanEqual => String::from(">="),
            TokenType::BitRight => String::from(">>"),
            TokenType::BitRightAssign => String::from(">>="),
            TokenType::NotEqual => String::from("!="),
            TokenType::Bang => String::from("!"),
            TokenType::Colon => String::from(":"),
            TokenType::Semicolon => String::from(";"),
            TokenType::Comma => String::from(","),
            TokenType::LeftBrace => String::from("{"),
            TokenType::RightBrace => String::from("}"),
            TokenType::LeftBracket => String::from("("),
            TokenType::RightBracket => String::from(")"),
            TokenType::LeftSquare => String::from("["),
            TokenType::RightSquare => String::from("]"),
            TokenType::Elipsis => String::from("..."),
            TokenType::Range => String::from(".."),
            TokenType::Directive => String::from("#"),
            TokenType::Const => String::from("const"),
            TokenType::New => String::from("new"),
            TokenType::Static => String::from("static"),
            TokenType::Stock => String::from("stock"),
            TokenType::Forward => String::from("forward"),
            TokenType::Public => String::from("public"),
            TokenType::Native => String::from("native"),
            TokenType::Operator => String::from("operator"),
            TokenType::Char => String::from("char"),
            TokenType::Enum => String::from("enum"),
            TokenType::State => String::from("state"),
            TokenType::If => String::from("if"),
            TokenType::Else => String::from("else"),
            TokenType::Switch => String::from("switch"),
            TokenType::Case => String::from("case"),
            TokenType::Default => String::from("default"),
            TokenType::For => String::from("for"),
            TokenType::While => String::from("while"),
            TokenType::Do => String::from("do"),
            TokenType::Break => String::from("break"),
            TokenType::Continue => String::from("continue"),
            TokenType::Goto => String::from("goto"),
            TokenType::Return => String::from("return"),
            TokenType::Sizeof => String::from("sizeof"),
            TokenType::Tagof => String::from("tagof"),
            TokenType::Emit => String::from("__emit"),
            TokenType::Integer => String::from("Integer"),
            TokenType::Float => String::from("Float"),
            TokenType::Symbol => String::from("Symbol"),
            TokenType::Label => String::from("Label"),
            TokenType::Literal => String::from("Literal"),
            TokenType::Comment => String::from("Comment"),
        }
    }
}

pub fn lookup_keyword(kw: &str) -> Option<TokenType> {
    match kw {
        "const" => Some(TokenType::Const),
        "new" => Some(TokenType::New),
        "static" => Some(TokenType::Static),
        "stock" => Some(TokenType::Stock),
        "forward" => Some(TokenType::Forward),
        "public" => Some(TokenType::Public),
        "native" => Some(TokenType::Native),
        "operator" => Some(TokenType::Operator),
        "char" => Some(TokenType::Char),
        "enum" => Some(TokenType::Enum),
        "state" => Some(TokenType::State),

        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),
        "switch" => Some(TokenType::Switch),
        "case" => Some(TokenType::Case),
        "default" => Some(TokenType::Default),
        "for" => Some(TokenType::For),
        "while" => Some(TokenType::While),
        "do" => Some(TokenType::Do),
        "break" => Some(TokenType::Break),
        "continue" => Some(TokenType::Continue),
        "goto" => Some(TokenType::Goto),
        "return" => Some(TokenType::Return),
        "sizeof" => Some(TokenType::Sizeof),
        "tagof" => Some(TokenType::Tagof),
        "__emit" => Some(TokenType::Emit),

        _ => None,
    }
}
